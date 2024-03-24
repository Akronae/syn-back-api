use crate::{
    api::lexicon::lexicon_model::{
        LexiconEntryDefinition,
    },
    error::SafeError,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{
    definition, page,
};

pub struct ScrappedConjunction {
    pub definitions: Vec<LexiconEntryDefinition>,
}
pub async fn scrap_conjunction(lemma: &str) -> Result<ScrappedConjunction, SafeError> {
    let doc = page::scrap(lemma).await?;

    let definitions = scrap_conjunction_defs(&doc)?;

    Ok(ScrappedConjunction { definitions })
}

pub fn scrap_conjunction_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Conjunction")?)
        .next()
        .with_context(|| "cannot find conjunction header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}
