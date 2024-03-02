use std::{collections::HashMap, hash::Hash, iter, thread::panicking};

use strum::Display;
use tl::{queryselector::iterable::QueryIterable, Node, NodeHandle, Parser, VDom};
use tracing::debug;
use url::Url;

use crate::{
    api::lexicon::lexicon_model::{
        NounInflectionCases, NounInflectionForm, NounInflectionGenders, NounInflectionNumbers,
        VerbInflectionForm, VerbInflectionMoods, VerbInflectionNumbers, VerbInflectionPersons,
        VerbInflectionTenses, VerbInflectionThemes, VerbInflectionVoices, WordInflection,
    },
    error::SafeError,
};

#[derive(Debug, Clone, Display)]
enum CellType {
    Header,
    Data,
}
#[derive(Debug, Clone)]
struct Cell {
    cell_type: CellType,
    data: String,
    x: i32,
    y: i32,
}

pub async fn extract_inflections(lemma: &str) -> Result<Vec<WordInflection>, SafeError> {
    let tables = extract_inflection_tables(lemma).await?;
    let mut word_inflections = Vec::<WordInflection>::new();
    let mut infl = WordInflection::default();
    for table in tables {
        infl = cells_to_word_inflection(infl, &table).await?;
        // if inflection == WordInflection::default() {
        // continue;
        // }
        // word_inflections.push(inflection);
    }

    if infl != WordInflection::default() {
        word_inflections.push(infl);
    }
    Ok(word_inflections)
}

#[derive(Debug, Clone)]
struct Table {
    title: Option<String>,
    node_html: String,
}

async fn extract_inflection_tables(lemma: &str) -> Result<Vec<Vec<Cell>>, SafeError> {
    let url = &get_inflect_url(lemma)?;
    debug!("Fetching {}", url);
    let res = reqwest::get(url).await?.text().await?;

    let dom = tl::parse(res.as_str(), tl::ParserOptions::default())?;
    let parser = dom.parser();

    let word_part_of_speech = dom
        .query_selector("p")
        .unwrap()
        .nth(1)
        .unwrap()
        .get(parser)
        .unwrap()
        .inner_text(parser)
        .trim()
        .to_lowercase();

    let tables: Vec<Table>;

    if word_part_of_speech != "verb" {
        tables = dom
            .query_selector("table[border='1']")
            .unwrap()
            .map(|x| Table {
                title: None,
                node_html: x.get(parser).unwrap().outer_html(parser).to_string(),
            })
            .collect();
    } else {
        tables = extract_verb_tables_html(&dom, parser);
    }

    let mut res = Vec::<Vec<Cell>>::new();

    for table in tables {
        let table_dom = tl::parse(&table.node_html, tl::ParserOptions::default())?;
        let table_parser = table_dom.parser();

        let trs = table_dom.query_selector("tr").unwrap();

        let mut cells = Vec::<Cell>::new();

        for (tr_i, tr) in trs.enumerate() {
            let tr_html = tr
                .get(table_parser)
                .unwrap()
                .as_tag()
                .unwrap()
                .inner_html(table_parser);
            let tr_dom = tl::parse(&tr_html, tl::ParserOptions::default())?;
            let tr_parser = tr_dom.parser();

            let row_cells = tr_dom
                .children()
                .iter()
                .map(|x| x.get(tr_parser).unwrap().as_tag().unwrap());

            let mut cell_x = 0;
            for row_cell in row_cells {
                let colspan = match row_cell.attributes().get("colspan") {
                    Some(colspan) => {
                        std::str::from_utf8(colspan.unwrap().as_bytes())?.parse::<i32>()?
                    }
                    None => 1,
                };
                let rowspan = match row_cell.attributes().get("rowspan") {
                    Some(rowspan) => {
                        std::str::from_utf8(rowspan.unwrap().as_bytes())?.parse::<i32>()?
                    }
                    None => 1,
                };
                let tag = std::str::from_utf8(row_cell.name().as_bytes())?;
                let t = row_cell.inner_text(tr_parser);
                let text = t.split('[').next().unwrap().trim();
                let node_type = match tag {
                    "th" => Ok(CellType::Header),
                    "td" => Ok(CellType::Data),
                    _ => Err("Unknown tag"),
                }?;

                for row_i in 0..rowspan {
                    for col_i in 0..colspan {
                        let mut x = cell_x + col_i;
                        let y = tr_i as i32 + row_i;

                        while cells.iter().any(|c| c.x == x && c.y == y) {
                            x += 1;
                        }

                        cells.push(Cell {
                            data: text.to_string(),
                            cell_type: node_type.to_owned(),
                            x,
                            y,
                        });
                    }
                }

                cell_x += colspan;
            }
        }

        cells_insert_header(&mut cells, &word_part_of_speech);
        if let Some(title) = table.title {
            for part in title.split(' ') {
                cells_insert_header(&mut cells, part);
            }
        }

        res.push(cells.clone());
    }

    Ok(res)
}

fn cells_insert_header(cells: &mut Vec<Cell>, header: &str) {
    for cell in &mut *cells {
        cell.y += 1;
    }
    let max_x = cells.iter().map(|c| c.x).max().unwrap_or(0) + 1;
    cells.extend((0..max_x).map(|i| Cell {
        cell_type: CellType::Header,
        data: header.to_string(),
        x: i,
        y: 0,
    }));
}

fn extract_verb_tables_html(root_dom: &VDom<'_>, root_parser: &Parser<'_>) -> Vec<Table> {
    let tds = root_dom.query_selector("td").unwrap();
    let divs = root_dom.query_selector("div").unwrap();

    tds.chain(divs)
        .filter_map(|x| {
            let x_html = x
                .get(root_parser)
                .unwrap()
                .inner_html(root_parser)
                .to_string();
            let x_dom = tl::parse(x_html.as_str(), tl::ParserOptions::default()).unwrap();
            let x_parser = x_dom.parser();

            let mut tables = Vec::<Table>::new();

            let mut h4_count = 0;
            for (child_i, child) in x_dom.children().iter().enumerate() {
                let Some(h4) = child.get(x_parser).map(|c| c.as_tag()).unwrap_or(None) else {
                    continue;
                };
                if h4.name().as_bytes() != b"h4" {
                    continue;
                }
                let h4_text = h4.inner_text(x_parser);
                let table = x_dom
                    .query_selector("table[border='1']")
                    .unwrap()
                    .nth(h4_count)
                    .unwrap()
                    .get(x_parser)
                    .unwrap();
                h4_count += 1;

                let mut title = h4_text.trim().to_lowercase().to_string();

                if title.contains("θη-") {
                    title = title.replace("θη-", "") + " pass";
                }

                dbg!(title.clone());
                tables.push(Table {
                    title: Some(title),
                    node_html: table.outer_html(x_parser).to_string(),
                });
            }
            if tables.is_empty() {
                return None;
            }
            return Some(tables);
        })
        .flatten()
        .collect()
}

fn get_inflect_url(lemma: &str) -> Result<String, SafeError> {
    let base_url = "https://lexicon.katabiblon.com/inflect.php";

    let mut url = Url::parse(base_url)?;
    url.query_pairs_mut().append_pair("lemma", lemma);

    Ok(url.to_string())
}

async fn cells_to_word_inflection(
    mut infl: WordInflection,
    cells: &[Cell],
) -> Result<WordInflection, SafeError> {
    let mut inflections = HashMap::<Vec<String>, String>::new();

    for cell in cells {
        if matches!(cell.cell_type, CellType::Data) {
            let mut headers = Vec::<String>::new();

            for c in cells {
                if matches!(c.cell_type, CellType::Header) && (c.y == cell.y || c.x == cell.x) {
                    headers.push(c.data.to_lowercase().trim().to_string());
                }
            }

            let data = cell.data.trim().to_string();
            if data.is_empty() {
                continue;
            }
            inflections.insert(headers, data);
        }
    }

    // let mut infl = WordInflection::default();

    for (parsing, word) in inflections {
        if parsing.contains(&"aorist".to_string()) && parsing.contains(&"act".to_string()) {
            debug!("ok!");
        }

        let empty_str = &String::new();
        let parsing_1 = parsing.first().unwrap_or(empty_str);
        if parsing_1.contains("noun") {
            if infl.noun.is_none() {
                infl.noun = Some(NounInflectionGenders::default());
            }
            extract_noun_inflection(infl.noun.as_mut().unwrap(), &parsing, &word)?;
        } else if parsing_1.contains("indeclinable") {
            continue;
        } else if parsing.contains(&"verb".to_string()) {
            if infl.verb.is_none() {
                infl.verb = Some(VerbInflectionTenses::default());
            }
            let verb = infl.verb.as_mut().unwrap();

            let tense_opt;
            if parsing.contains(&"present".to_string()) {
                if verb.present.is_none() {
                    verb.present = Some(VerbInflectionThemes::default());
                }
                tense_opt = verb.present.as_mut();
            } else if parsing.contains(&"imperfect".to_string()) {
                if verb.imperfect.is_none() {
                    verb.imperfect = Some(VerbInflectionThemes::default());
                }
                tense_opt = verb.imperfect.as_mut();
            } else if parsing.contains(&"future".to_string()) {
                if verb.future.is_none() {
                    verb.future = Some(VerbInflectionThemes::default());
                }
                tense_opt = verb.future.as_mut();
            } else if parsing.contains(&"aorist".to_string()) {
                if verb.aorist.is_none() {
                    verb.aorist = Some(VerbInflectionThemes::default());
                }
                tense_opt = verb.aorist.as_mut();
            } else if parsing.contains(&"perfect".to_string()) {
                if verb.perfect.is_none() {
                    verb.perfect = Some(VerbInflectionThemes::default());
                }
                tense_opt = verb.perfect.as_mut();
            } else if parsing.contains(&"pluperfect".to_string()) {
                if verb.pluperfect.is_none() {
                    verb.pluperfect = Some(VerbInflectionThemes::default());
                }
                tense_opt = verb.pluperfect.as_mut();
            } else {
                return Err(format!("unknown tense {:?} for verb {word}", parsing).into());
            }
            let tense = tense_opt.unwrap();

            let theme_opt;
            if parsing.contains(&"athematic".to_string()) {
                if tense.athematic.is_none() {
                    tense.athematic = Some(VerbInflectionMoods::default());
                }
                theme_opt = tense.athematic.as_mut();
            } else {
                if tense.thematic.is_none() {
                    tense.thematic = Some(VerbInflectionMoods::default());
                }
                theme_opt = tense.thematic.as_mut();
            }
            let theme = theme_opt.unwrap();

            let voice_opt;
            if parsing.contains(&"indicative".to_string()) {
                if theme.indicative.is_none() {
                    theme.indicative = Some(VerbInflectionVoices::default());
                }
                voice_opt = theme.indicative.as_mut();
            } else if parsing.contains(&"subjunctive".to_string()) {
                if theme.subjunctive.is_none() {
                    theme.subjunctive = Some(VerbInflectionVoices::default());
                }
                voice_opt = theme.subjunctive.as_mut();
            } else if parsing.contains(&"optative".to_string()) {
                if theme.optative.is_none() {
                    theme.optative = Some(VerbInflectionVoices::default());
                }
                voice_opt = theme.optative.as_mut();
            } else if parsing.contains(&"imperative".to_string()) {
                if theme.imperative.is_none() {
                    theme.imperative = Some(VerbInflectionVoices::default());
                }
                voice_opt = theme.imperative.as_mut();
            } else if parsing.contains(&"infinitive".to_string()) {
                if theme.infinitive.is_none() {
                    theme.infinitive = Some(VerbInflectionForm::default());
                }
                extract_verb_contraction(&parsing, theme.infinitive.as_mut().unwrap(), &word)?;
                continue;
            } else if parsing.contains(&"pluperfect".to_string()) {
                if theme.pluperfect.is_none() {
                    theme.pluperfect = Some(VerbInflectionVoices::default());
                }
                voice_opt = theme.pluperfect.as_mut();
            } else if parsing.contains(&"participle".to_string()) {
                if theme.participle.is_none() {
                    theme.participle = Some(NounInflectionGenders::default());
                }
                extract_noun_inflection(theme.participle.as_mut().unwrap(), &parsing, &word)?;
                continue;
            } else {
                return Err(format!("unknown mood {:?}", parsing).into());
            }
            let voice = voice_opt.unwrap();

            let number_opt;
            if parsing.contains(&"act".to_string()) {
                if voice.active.is_none() {
                    voice.active = Some(VerbInflectionNumbers::default());
                }
                number_opt = voice.active.as_mut();
            } else if parsing.contains(&"m/p".to_string()) {
                if voice.middle.is_none() {
                    voice.middle = Some(VerbInflectionNumbers::default());
                }
                number_opt = voice.middle.as_mut();
            } else if parsing.contains(&"pass".to_string()) {
                if voice.passive.is_none() {
                    voice.passive = Some(VerbInflectionNumbers::default());
                }
                number_opt = voice.passive.as_mut();
            } else {
                return Err(format!("unknown voice {:?}", parsing).into());
            }
            let number = number_opt.unwrap();

            let person_opt;
            if parsing.contains(&"sg".to_string()) {
                if number.singular.is_none() {
                    number.singular = Some(VerbInflectionPersons::default());
                }
                person_opt = number.singular.as_mut();
            } else if parsing.contains(&"pl".to_string()) {
                if number.plural.is_none() {
                    number.plural = Some(VerbInflectionPersons::default());
                }
                person_opt = number.plural.as_mut();
            } else {
                return Err(format!("unknown number {:?}", parsing).into());
            }
            let person = person_opt.unwrap();

            let form_opt;
            if parsing.contains(&"1st".to_string()) {
                if person.first.is_none() {
                    person.first = Some(VerbInflectionForm::default());
                }
                form_opt = person.first.as_mut();
            } else if parsing.contains(&"2nd".to_string()) {
                if person.second.is_none() {
                    person.second = Some(VerbInflectionForm::default());
                }
                form_opt = person.second.as_mut();
            } else if parsing.contains(&"3rd".to_string()) {
                if person.third.is_none() {
                    person.third = Some(VerbInflectionForm::default());
                }
                form_opt = person.third.as_mut();
            } else {
                return Err(format!("unknown person {:?}", parsing).into());
            }
            let form = form_opt.unwrap();

            extract_verb_contraction(&parsing, form, &word)?;
        } else {
            return Err(format!("unknown parsing {:?} for {word}", parsing).into());
        }
    }

    Ok(infl)
}

fn extract_noun_inflection(
    noun: &mut NounInflectionGenders,
    parsing: &Vec<String>,
    word: &String,
) -> Result<(), SafeError> {
    let gender_opt;
    if parsing.contains(&"feminine".to_string()) {
        if noun.feminine.is_none() {
            noun.feminine = Some(NounInflectionNumbers::default());
        }
        gender_opt = noun.feminine.as_mut();
    } else if parsing.contains(&"masculine".to_string()) {
        if noun.masculine.is_none() {
            noun.masculine = Some(NounInflectionNumbers::default());
        }
        gender_opt = noun.masculine.as_mut();
    } else if parsing.contains(&"neuter".to_string()) {
        if noun.neuter.is_none() {
            noun.neuter = Some(NounInflectionNumbers::default());
        }
        gender_opt = noun.neuter.as_mut();
    } else {
        return Err(format!("unknown gender {:?} for word {word}", parsing).into());
    }
    let gender = gender_opt.unwrap();
    let number_opt;
    if parsing.contains(&"sg".to_string()) {
        if gender.singular.is_none() {
            gender.singular = Some(NounInflectionCases::default());
        }
        number_opt = gender.singular.as_mut();
    } else if parsing.contains(&"pl".to_string()) {
        if gender.plural.is_none() {
            gender.plural = Some(NounInflectionCases::default());
        }
        number_opt = gender.plural.as_mut();
    } else {
        return Err(format!("unknown number {}", parsing.join(", ")).into());
    }
    let number = number_opt.unwrap();
    let gram_case;
    if parsing.contains(&"gen".to_string()) {
        if number.genitive.is_none() {
            number.genitive = Some(NounInflectionForm::default());
        }
        gram_case = number.genitive.as_mut();
    } else if parsing.contains(&"nom".to_string()) {
        if number.nominative.is_none() {
            number.nominative = Some(NounInflectionForm::default());
        }
        gram_case = number.nominative.as_mut();
    } else if parsing.contains(&"dat".to_string()) {
        if number.dative.is_none() {
            number.dative = Some(NounInflectionForm::default());
        }
        gram_case = number.dative.as_mut();
    } else if parsing.contains(&"acc".to_string()) {
        if number.accusative.is_none() {
            number.accusative = Some(NounInflectionForm::default());
        }
        gram_case = number.accusative.as_mut();
    } else if parsing.contains(&"voc".to_string()) {
        if number.vocative.is_none() {
            number.vocative = Some(NounInflectionForm::default());
        }
        gram_case = number.vocative.as_mut();
    } else {
        return Err(format!("unknown case {}", parsing.join(", ")).into());
    }

    if parsing.contains(&"contracted".to_string()) {
        gram_case.unwrap().contracted = Some(word.to_string());
    } else if parsing.contains(&"uncontracted".to_string()) {
        gram_case.unwrap().uncontracted =
            Some(word.split(['.', '·']).map(|x| x.to_string()).collect());
    } else {
        return Err(format!("unknown contraction {}", parsing.join(", ")).into());
    }

    Ok(())
}

fn extract_verb_contraction(
    parsing: &Vec<String>,
    form: &mut VerbInflectionForm,
    word: &String,
) -> Result<(), SafeError> {
    if parsing.contains(&"inflection".to_string()) {
        form.contracted = Some(word.to_string());
        return Ok(());
    } else if parsing.contains(&"uncontracted".to_string()) {
        form.uncontracted = Some(word.split(['.', '·']).map(|x| x.to_string()).collect());
        return Ok(());
    }
    return Err(format!("unknown contraction {:?} for verb {word}", parsing).into());
}
