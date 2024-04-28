use crate::{
    api::lexicon::lexicon_model::{InflectionForm, LexiconEntryDefinition, WordInflection},
    error::SafeError,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::{selectable::Selectable, Html};

use super::{definition, page};

pub struct ScrappedAdverb {
    pub inflections: Vec<WordInflection>,
    pub definitions: Vec<LexiconEntryDefinition>,
}
pub async fn scrap_adverb(lemma: &str) -> Result<ScrappedAdverb, SafeError> {
    let doc = page::scrap(lemma).await?;

    let inflection = WordInflection {
        adverb: Some(vec![InflectionForm {
            contracted: Some(lemma.to_string()),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let definitions = scrap_adverb_defs(&doc)?;

    Ok(ScrappedAdverb {
        inflections: vec![inflection],
        definitions,
    })
}

pub fn scrap_adverb_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let adverb = doc.select(&select("#Adverb")?).next();

    let container = adverb.with_context(|| "cannot find adverb header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}
