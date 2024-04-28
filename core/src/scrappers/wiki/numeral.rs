use crate::{
    api::lexicon::lexicon_model::{
        LexiconEntryDefinition, NounInflectionGenders, WordInflection,
    },
    error::SafeError,
    scrappers::wiki::table::parse_declension_table,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{
    definition, noun, page,
    table::{get_words_dialects, ParsedWord},
};

pub struct ScrappedNumeral {
    pub inflections: Vec<WordInflection>,
    pub definitions: Vec<LexiconEntryDefinition>,
}
pub async fn scrap_numeral(lemma: &str) -> Result<ScrappedNumeral, SafeError> {
    let doc = page::scrap(lemma).await?;

    let selector = select(".NavFrame.grc-decl.grc-adecl")?;
    let decl_tables = doc.select(&selector);

    let mut inflections = vec![];
    for table in decl_tables {
        let words = parse_declension_table(&table)?;
        let infl = parsed_words_to_inflection(&words);
        let dialects = get_words_dialects(&words);
        inflections.push(WordInflection {
            dialects,
            numeral: Some(Box::from(infl)),
            ..Default::default()
        });
    }

    let definitions = scrap_numeral_defs(&doc)?;

    Ok(ScrappedNumeral {
        inflections,
        definitions,
    })
}

pub fn scrap_numeral_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Numeral")?)
        .next()
        .with_context(|| "cannot find numeral header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}

fn parsed_words_to_inflection(words: &[ParsedWord]) -> NounInflectionGenders {
    let mut infl = NounInflectionGenders::default();

    for word in words {
        noun::fill_genders(word, &mut infl);
    }

    infl
}
