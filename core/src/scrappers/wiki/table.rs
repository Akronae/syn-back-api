use scraper::{CaseSensitivity, ElementRef};

use crate::{
    borrow::Cow,
    error::SafeError,
    grammar::{Case, DeclensionType, Dialect, Number},
    utils::scrapper::select::select,
};

pub fn parse_declension_table(table: &ElementRef) -> Result<Vec<ParsedWord>, SafeError> {
    let cells = extract_table_cells(table)?;
    let words = parse_table(cells);
    Ok(words)
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
fn extract_table_cells(table: &scraper::ElementRef) -> Result<Vec<TableCell>, SafeError> {
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

    for (y, tr) in trs.enumerate() {
        for (x, child) in tr.children().filter(|x| x.value().is_element()).enumerate() {
            let Some(elem) = child.value().as_element() else {
                continue;
            };
            let cell_type = match elem.name() {
                "th" => TableCellType::Header,
                "td" => TableCellType::Data,
                _ => continue,
            };

            let mut content = ElementRef::wrap(child)
                .unwrap()
                .text()
                .collect::<Cow<str>>()
                .trim()
                .to_string();

            if matches!(cell_type, TableCellType::Header) {
                content = content.to_lowercase();
            }

            if elem.has_class("form", CaseSensitivity::CaseSensitive) {
                content = ElementRef::wrap(child)
                    .unwrap()
                    .select(&select(".Polyt")?)
                    .last()
                    .unwrap()
                    .text()
                    .collect::<Cow<str>>()
                    .trim()
                    .to_string();
            }
            cells.push(TableCell {
                cell_type,
                content: content.into(),
                x,
                y,
            });
        }
    }

    cells = table_insert_header(cells, title);

    Ok(cells)
}

fn table_insert_header(mut table: Vec<TableCell>, header: String) -> Vec<TableCell> {
    let max_x = table.iter().map(|x| x.x).max().unwrap_or(0);
    for cell in &mut table {
        cell.y += 1;
    }
    table.extend((0..max_x + 1).map(|x| TableCell {
        cell_type: TableCellType::Header,
        content: header.clone().into(),
        x,
        y: 0,
    }));
    table
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParsingComp {
    Number(Number),
    Case(Case),
    Declension(DeclensionType),
    Dialect(Dialect),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ParsedWord {
    pub text: Cow<str>,
    pub parsing: Vec<ParsingComp>,
}
fn parse_table(cells: Vec<TableCell>) -> Vec<ParsedWord> {
    let mut words = Vec::<ParsedWord>::new();

    let headers = cells
        .iter()
        .filter(|x| matches!(x.cell_type, TableCellType::Header))
        .collect::<Vec<_>>();
    let data = cells
        .iter()
        .filter(|x| matches!(x.cell_type, TableCellType::Data))
        .collect::<Vec<_>>();

    for cell in &data {
        let mut parsing = Vec::<ParsingComp>::new();

        let cell_headers = headers
            .iter()
            .filter(|x| x.y == cell.y || x.x == cell.x)
            .collect::<Vec<_>>();
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
            if header.content.contains("attic") {
                parsing.push(ParsingComp::Dialect(Dialect::Attic));
            }
        }

        words.push(ParsedWord {
            text: cell.content.clone(),
            parsing,
        });
    }

    words
}
