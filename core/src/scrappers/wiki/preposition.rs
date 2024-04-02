use crate::{
    api::lexicon::lexicon_model::LexiconEntryDefinition, error::SafeError,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{definition, page};

pub struct ScrappedPreposition {
    pub definitions: Vec<LexiconEntryDefinition>,
}
pub async fn scrap_preposition(lemma: &str) -> Result<ScrappedPreposition, SafeError> {
    let doc = page::scrap(lemma).await?;

    let definitions = scrap_preposition_defs(&doc)?;

    Ok(ScrappedPreposition { definitions })
}

pub fn scrap_preposition_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Preposition")?)
        .next()
        .with_context(|| "cannot find preposition header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}
