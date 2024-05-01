use crate::{
    api::lexicon::lexicon_model::{
        LexiconEntryDefinition, NounInflectionGenders, WordAdjective, WordInflection,
    },
    error::SafeError,
    grammar::{Adjective},
    scrappers::wiki::table::parse_declension_table,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{
    definition, noun, page,
    table::{get_words_dialects, ParsedWord},
};

pub struct ScrappedAdjective {
    pub inflections: Vec<WordInflection>,
    pub definitions: Vec<LexiconEntryDefinition>,
}
pub async fn scrap_adjective(lemma: &str, adj: &Adjective) -> Result<ScrappedAdjective, SafeError> {
    let doc = page::scrap(lemma).await?;

    let selector = select(".NavFrame.grc-decl.grc-adecl")?;
    let decl_tables = doc.select(&selector);

    let mut inflections = vec![];
    for table in decl_tables {
        let words = parse_declension_table(&table)?;
        let infl = parsed_words_to_inflection(&words);
        let dialects = get_words_dialects(&words);
        let mut adjectives = WordAdjective::default();
        match adj {
            Adjective::Positive => adjectives.positive = Some(Box::from(infl)),
            Adjective::Comparative => adjectives.comparative = Some(Box::from(infl)),
            Adjective::Superlative => adjectives.superlative = Some(Box::from(infl)),
        }
        inflections.push(WordInflection {
            dialects,
            adjective: Some(Box::from(adjectives)),
            ..Default::default()
        });
    }

    let definitions = scrap_adjective_defs(&doc)?;

    Ok(ScrappedAdjective {
        inflections,
        definitions,
    })
}

pub fn scrap_adjective_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Adjective")?)
        .next()
        .with_context(|| "cannot find adjective header".to_string())?;

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
