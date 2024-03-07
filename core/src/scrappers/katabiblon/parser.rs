use tracing::*;

use crate::{
    api::lexicon::lexicon_model::LexiconEntry, error::SafeError, grammar::Declension,
    scrappers::katabiblon::details::search_word_details,
};

#[allow(dead_code)]
pub async fn parse_word(
    greek_word: &str,
    declension: &Declension,
) -> Result<LexiconEntry, SafeError> {
    info!("Parsing word {}", greek_word);

    let details = search_word_details(greek_word, declension).await?;
    dbg!(details.clone());

    Ok(LexiconEntry {
        lemma: details.lemma,
        inflections: Vec::new(),
        definitions: Vec::new(),
    })
}
