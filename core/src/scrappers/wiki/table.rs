use scraper::ElementRef;

use crate::{
    borrow::Cow,
    error::SafeError,
    grammar::{
        Case, Contraction, DeclensionType, Dialect, Gender, Mood, Number, Person, Tense, Voice,
    },
    utils::{scrapper::select::select, str::skip_last::SkipLastVec},
};

pub fn parse_declension_table(table: &ElementRef) -> Result<Vec<ParsedWord>, SafeError> {
    let extracted = extract_table_cells(table)?;
    let words = parse_table(&extracted);
    Ok(words)
}

pub fn get_words_dialects(words: &[ParsedWord]) -> Vec<Dialect> {
    let mut dialects = words
        .iter()
        .flat_map(|x| {
            x.parsing.iter().filter_map(|y| {
                if let ParsingComp::Dialect(d) = y {
                    Some(*d)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Dialect>>();
    dialects.sort();
    dialects.dedup();
    dialects
}

pub fn get_words_tenses(words: &[ParsedWord]) -> Vec<Tense> {
    let mut tenses = words
        .iter()
        .flat_map(|x| {
            x.parsing.iter().filter_map(|y| {
                if let ParsingComp::Tense(d) = y {
                    Some(*d)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Tense>>();
    tenses.dedup();
    tenses
}

#[derive(Debug, Clone)]
enum TableCellType {
    Header,
    Data,
}
#[derive(Debug, Clone)]
struct TableCell {
    pub cell_type: TableCellType,
    pub content: Cow<str>,
    pub x: usize,
    pub y: usize,
}
struct Table {
    title: Cow<str>,
    cells: Vec<TableCell>,
}
fn extract_table_cells(table: &scraper::ElementRef) -> Result<Table, SafeError> {
    let mut cells = Vec::<TableCell>::new();

    let title = table
        .select(&select(".NavHead")?)
        .next()
        .unwrap()
        .text()
        .collect::<Cow<str>>()
        .trim()
        .to_lowercase();

    let selector = select("tr")?;
    let trs = table.select(&selector);

    let mut y = 0;
    let mut x;

    for tr in trs {
        x = cells
            .iter()
            .filter(|c| c.y == y)
            .map(|c| c.x)
            .max()
            .map(|n| n + 1)
            .unwrap_or(0);

        for child in tr.children().filter(|x| x.value().is_element()) {
            let Some(elem) = child.value().as_element() else {
                continue;
            };
            let cell_type = match elem.name() {
                "th" => TableCellType::Header,
                "td" => TableCellType::Data,
                _ => continue,
            };

            let mut content = String::new();

            if matches!(cell_type, TableCellType::Header) {
                content = ElementRef::wrap(child)
                    .unwrap()
                    .text()
                    .collect::<Cow<str>>()
                    .trim()
                    .to_lowercase();
            } else {
                let s = select(".Polyt")?;
                let mut polyts = ElementRef::wrap(child)
                    .unwrap()
                    .select(&s)
                    .collect::<Vec<ElementRef>>();
                let mut append = "";
                if polyts
                    .clone()
                    .last()
                    .map(|x| x.text().collect::<String>() == "ν")
                    .unwrap_or(false)
                {
                    append = "(ν)";
                    polyts = polyts.skip_last(1);
                }
                let polyt = polyts.last();

                if let Some(polyt) = polyt {
                    for child in polyt.children() {
                        if child
                            .value()
                            .as_element()
                            .map(|x| x.name() == "br")
                            .unwrap_or(false)
                        {
                            content.push('\n');
                        } else {
                            content.push_str(
                                ElementRef::wrap(child)
                                    .unwrap()
                                    .text()
                                    .collect::<String>()
                                    .trim(),
                            );
                        }
                    }
                } else {
                    content = ElementRef::wrap(child)
                        .unwrap()
                        .text()
                        .collect::<Cow<str>>()
                        .trim()
                        .to_string();
                }

                content.push_str(append);
            }

            let height = elem
                .attr("rowspan")
                .map(|x| x.parse::<usize>().unwrap())
                .unwrap_or(1);
            let width = elem
                .attr("colspan")
                .map(|x| x.parse::<usize>().unwrap())
                .unwrap_or(1);

            for h in 0..height {
                for w in 0..width {
                    cells.push(TableCell {
                        cell_type: cell_type.clone(),
                        content: content.clone().into(),
                        x: x + w,
                        y: y + h,
                    });
                }
            }

            x += width;
        }

        y += 1;
    }

    Ok(Table {
        title: title.into(),
        cells,
    })
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum ParsingComp {
    Number(Number),
    Case(Case),
    Gender(Gender),
    Declension(DeclensionType),
    Dialect(Dialect),
    Mood(Mood),
    Tense(Tense),
    Voice(Voice),
    Person(Person),
    Contraction(Contraction),
}
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct ParsedWord {
    pub text: Cow<str>,
    pub parsing: Vec<ParsingComp>,
}
fn parse_table(table: &Table) -> Vec<ParsedWord> {
    let mut words = Vec::<ParsedWord>::new();

    let headers = table
        .cells
        .iter()
        .filter(|x| matches!(x.cell_type, TableCellType::Header))
        .collect::<Vec<_>>();
    let data = table
        .cells
        .iter()
        .filter(|x| matches!(x.cell_type, TableCellType::Data))
        .collect::<Vec<_>>();

    for cell in &data {
        if cell.content.is_empty() || cell.content == "—" {
            continue;
        }

        let mut parsing = Vec::<ParsingComp>::new();

        let mut cell_headers = Vec::<&TableCell>::new();
        for x in 0..cell.x {
            cell_headers.extend(
                headers
                    .iter()
                    .filter(|c| c.y == cell.y && c.x == x)
                    .collect::<Vec<_>>(),
            );
        }
        let mut found_y_header = false;
        for y in (0..cell.y).rev() {
            let h = headers
                .iter()
                .filter(|c| c.x == cell.x && c.y == y)
                .collect::<Vec<_>>();

            if h.is_empty() {
                if found_y_header {
                    break;
                }
            } else {
                found_y_header = true;
            }
            cell_headers.extend(h);
        }

        if cell_headers
            .iter()
            .any(|x| x.content.eq_ignore_ascii_case("notes:"))
        {
            continue;
        }

        for header in &cell_headers {
            if header.content.contains("singular") {
                parsing.push(ParsingComp::Number(Number::Singular));
            }
            if header.content.contains("dual") {
                parsing.push(ParsingComp::Number(Number::Dual));
            }
            if header.content.contains("plural") {
                parsing.push(ParsingComp::Number(Number::Plural));
            }
            if header.content.contains("nominative") {
                parsing.push(ParsingComp::Case(Case::Nominative));
            }
            if header.content.contains("genitive") {
                parsing.push(ParsingComp::Case(Case::Genitive));
            }
            if header.content.contains("dative") {
                parsing.push(ParsingComp::Case(Case::Dative));
            }
            if header.content.contains("accusative") {
                parsing.push(ParsingComp::Case(Case::Accusative));
            }
            if header.content.contains("vocative") {
                parsing.push(ParsingComp::Case(Case::Vocative));
            }
            if header.content.contains("second declension") {
                parsing.push(ParsingComp::Declension(DeclensionType::Second));
            }
            if header.content == ("middle/passive") {
                parsing.push(ParsingComp::Voice(Voice::Middle));
                parsing.push(ParsingComp::Voice(Voice::Passive));
            }
            if header.content == ("middle") {
                parsing.push(ParsingComp::Voice(Voice::Middle));
            }
            if header.content == ("passive") {
                parsing.push(ParsingComp::Voice(Voice::Passive));
            }
            if header.content == ("active") {
                parsing.push(ParsingComp::Voice(Voice::Active));
            }
            if header.content == ("participle") {
                parsing.push(ParsingComp::Mood(Mood::Participle));
            }
            if header.content == ("infinitive") {
                parsing.push(ParsingComp::Mood(Mood::Infinitive));
            }
            if header.content == ("indicative") {
                parsing.push(ParsingComp::Mood(Mood::Indicative));
            }
            if header.content == ("subjunctive") {
                parsing.push(ParsingComp::Mood(Mood::Subjunctive));
            }
            if header.content == ("optative") {
                parsing.push(ParsingComp::Mood(Mood::Optative));
            }
            if header.content == ("imperative") {
                parsing.push(ParsingComp::Mood(Mood::Imperative));
            }
            if header.content == ("first") {
                parsing.push(ParsingComp::Person(Person::First));
            }
            if header.content == ("second") {
                parsing.push(ParsingComp::Person(Person::Second));
            }
            if header.content == ("third") {
                parsing.push(ParsingComp::Person(Person::Third));
            }
            if header.content == ("m") || header.content == ("masculine") {
                parsing.push(ParsingComp::Gender(Gender::Masculine));
            }
            if header.content == ("f") || header.content == ("feminine") {
                parsing.push(ParsingComp::Gender(Gender::Feminine));
            }
            if header.content == ("n") || header.content == ("neuter") {
                parsing.push(ParsingComp::Gender(Gender::Neuter));
            }
        }

        if table.title.starts_with("present:") {
            parsing.push(ParsingComp::Tense(Tense::Present));
        }
        if table.title.starts_with("imperfect:") {
            parsing.push(ParsingComp::Tense(Tense::Imperfect));
        }
        if table.title.starts_with("future:") {
            parsing.push(ParsingComp::Tense(Tense::Future));
        }
        if table.title.starts_with("aorist:") {
            parsing.push(ParsingComp::Tense(Tense::Aorist));
        }
        if table.title.starts_with("perfect:") {
            parsing.push(ParsingComp::Tense(Tense::Perfect));
        }
        if table.title.starts_with("pluperfect:") {
            parsing.push(ParsingComp::Tense(Tense::Pluperfect));
        }
        if table.title.starts_with("future perfect:") {
            parsing.push(ParsingComp::Tense(Tense::FuturePerfect));
        }
        if table.title.ends_with("(contracted)") {
            parsing.push(ParsingComp::Contraction(Contraction::Contracted));
        }
        if table.title.ends_with("(uncontracted)") {
            parsing.push(ParsingComp::Contraction(Contraction::Uncontracted));
        }
        if table.title.contains("attic") {
            parsing.push(ParsingComp::Dialect(Dialect::Attic));
        }
        if table.title.contains("koine") {
            parsing.push(ParsingComp::Dialect(Dialect::Koine));
        }
        if table.title.contains("epic") {
            parsing.push(ParsingComp::Dialect(Dialect::Epic));
        }
        if table.title.contains("laconian") {
            parsing.push(ParsingComp::Dialect(Dialect::Laconian));
        }
        if table.title.contains("doric") {
            parsing.push(ParsingComp::Dialect(Dialect::Doric));
        }

        words.push(ParsedWord {
            text: cell.content.clone(),
            parsing,
        });
    }

    words.sort();
    words.dedup();
    words
}
