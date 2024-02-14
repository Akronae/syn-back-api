use std::collections::HashMap;

use strum::Display;
use tracing::debug;
use url::Url;

use crate::{
    api::lexicon::lexicon_model::{
        NounInflectionCases, NounInflectionForm, NounInflectionGenders, NounInflectionNumbers,
        WordInflection,
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
    for table in tables {
        let inflection = cells_to_word_inflection(&table).await?;
        if inflection.noun.is_none() {
            continue;
        }
        word_inflections.push(inflection);
    }

    Ok(word_inflections)
}

async fn extract_inflection_tables(lemma: &str) -> Result<Vec<Vec<Cell>>, SafeError> {
    let url = &get_inflect_url(lemma)?;
    debug!("Fetching {}", url);
    let res = reqwest::get(url).await?.text().await?;

    let dom = tl::parse(res.as_str(), tl::ParserOptions::default())?;
    let parser = dom.parser();

    let tables = dom.query_selector("table.layout").unwrap();

    let mut res = Vec::<Vec<Cell>>::new();

    for table in tables {
        let table_html = table
            .get(parser)
            .unwrap()
            .as_tag()
            .unwrap()
            .inner_html(parser);
        let table_dom = tl::parse(&table_html, tl::ParserOptions::default())?;
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

        res.push(cells);
    }

    Ok(res)
}

fn get_inflect_url(lemma: &str) -> Result<String, SafeError> {
    let base_url = "https://lexicon.katabiblon.com/inflect.php";

    let mut url = Url::parse(base_url)?;
    url.query_pairs_mut().append_pair("lemma", lemma);

    Ok(url.to_string())
}

async fn cells_to_word_inflection(cells: &[Cell]) -> Result<WordInflection, SafeError> {
    let mut inflections = HashMap::<Vec<String>, String>::new();

    for cell in cells {
        if matches!(cell.cell_type, CellType::Data) {
            let mut headers = Vec::<String>::new();

            for c in cells {
                if matches!(c.cell_type, CellType::Header) && (c.y == cell.y || c.x == cell.x) {
                    headers.push(c.clone().data);
                }
            }

            let data = cell.data.trim().to_string();
            if data.is_empty() {
                continue;
            }
            inflections.insert(headers, data);
        }
    }

    let mut infl = WordInflection { noun: None };

    for (parsing, word) in inflections {
        let empty_str = &String::new();
        let parsing_1 = parsing.first().unwrap_or(empty_str);
        if parsing_1.contains("Noun") {
            if infl.noun.is_none() {
                infl.noun = Some(NounInflectionGenders::default());
            }
            let noun = infl.noun.as_mut().unwrap();

            let gender_opt;
            if parsing_1.contains("Feminine") {
                if noun.feminine.is_none() {
                    noun.feminine = Some(NounInflectionNumbers::default());
                }
                gender_opt = noun.feminine.as_mut();
            } else if parsing_1.contains("Masculine") {
                if noun.masculine.is_none() {
                    noun.masculine = Some(NounInflectionNumbers::default());
                }
                gender_opt = noun.masculine.as_mut();
            } else if parsing_1.contains("Neuter") {
                if noun.neuter.is_none() {
                    noun.neuter = Some(NounInflectionNumbers::default());
                }
                gender_opt = noun.neuter.as_mut();
            } else {
                return Err(format!("unknown gender {}", parsing_1).into());
            }
            let gender = gender_opt.unwrap();

            let number_opt;
            if parsing.contains(&"Sg".to_string()) {
                if gender.singular.is_none() {
                    gender.singular = Some(NounInflectionCases::default());
                }
                number_opt = gender.singular.as_mut();
            } else if parsing.contains(&"Pl".to_string()) {
                if gender.plural.is_none() {
                    gender.plural = Some(NounInflectionCases::default());
                }
                number_opt = gender.plural.as_mut();
            } else {
                return Err(format!("unknown number {}", parsing.join(", ")).into());
            }
            let number = number_opt.unwrap();

            let gram_case;
            if parsing.contains(&"Gen".to_string()) {
                if number.genitive.is_none() {
                    number.genitive = Some(NounInflectionForm::default());
                }
                gram_case = number.genitive.as_mut();
            } else if parsing.contains(&"Nom".to_string()) {
                if number.nominative.is_none() {
                    number.nominative = Some(NounInflectionForm::default());
                }
                gram_case = number.nominative.as_mut();
            } else if parsing.contains(&"Dat".to_string()) {
                if number.dative.is_none() {
                    number.dative = Some(NounInflectionForm::default());
                }
                gram_case = number.dative.as_mut();
            } else if parsing.contains(&"Acc".to_string()) {
                if number.accusative.is_none() {
                    number.accusative = Some(NounInflectionForm::default());
                }
                gram_case = number.accusative.as_mut();
            } else if parsing.contains(&"Voc".to_string()) {
                if number.vocative.is_none() {
                    number.vocative = Some(NounInflectionForm::default());
                }
                gram_case = number.vocative.as_mut();
            } else {
                return Err(format!("unknown case {}", parsing.join(", ")).into());
            }

            if parsing.contains(&"Contracted".to_string()) {
                gram_case.unwrap().contracted = Some(word.to_string());
            } else if parsing.contains(&"Uncontracted".to_string()) {
                gram_case.unwrap().uncontracted = Some(word.to_string());
            } else {
                return Err(format!("unknown contraction {}", parsing.join(", ")).into());
            }
        } else if parsing_1.contains("Indeclinable") {
            continue;
        } else {
            return Err(format!("unknown parsing {}", parsing_1).into());
        }
    }

    Ok(infl)
}
