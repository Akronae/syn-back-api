use tracing::*;

use crate::{
    api::lexicon::lexicon_model::LexiconEntry,
    error::SafeError,
    grammar::Declension,
    scrappers::katabiblon::{details::search_word_details, inflections::extract_inflections},
};

#[allow(dead_code)]
pub async fn parse_word(
    greek_word: &str,
    declension: &Declension,
) -> Result<LexiconEntry, SafeError> {
    info!("Parsing word {}", greek_word);

    let details = search_word_details(greek_word, declension).await?;
    dbg!(details.clone());

    let inflections = extract_inflections(&details.lemma).await?;
    dbg!(inflections.clone());

    Ok(LexiconEntry {
        lemma: details.lemma,
        description: details.description,
        translation: details.translation,
        inflections,
    })
}
