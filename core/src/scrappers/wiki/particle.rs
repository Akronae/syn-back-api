use crate::{
    api::lexicon::lexicon_model::{InflectionForm, LexiconEntryDefinition, WordInflection},
    error::SafeError,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{definition, page};

pub struct ScrappedParticle {
    pub inflections: Vec<WordInflection>,
    pub definitions: Vec<LexiconEntryDefinition>,
}
pub async fn scrap_particle(lemma: &str) -> Result<ScrappedParticle, SafeError> {
    let doc = page::scrap(lemma).await?;

    let inflection = WordInflection {
        particle: Some(vec![InflectionForm {
            contracted: Some(lemma.to_string()),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let definitions = scrap_particle_defs(&doc)?;

    Ok(ScrappedParticle {
        inflections: vec![inflection],
        definitions,
    })
}

pub fn scrap_particle_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let particle = doc.select(&select("#Particle")?).next();
    let conjunction = doc.select(&select("#Conjunction")?).next();

    let container = particle
        .or(conjunction)
        .with_context(|| "cannot find particle header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}
