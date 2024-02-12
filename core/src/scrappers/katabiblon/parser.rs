




use tracing::{*};


use crate::{
    error::SafeError,
    grammar::{
        Declension,
    },
    scrappers::{
        katabiblon::{details::search_word_details, inflections::extract_inflections},
    },
};

use super::{details::WordDetails, inflections::WordInflection};

#[derive(Debug)]
pub struct ParsingResult {
    pub details: WordDetails,
    pub inflections: Vec<WordInflection>,
}

#[allow(dead_code)]
pub async fn parse_word(
    greek_word: &str,
    declension: &Declension,
) -> Result<ParsingResult, SafeError> {
    info!("Parsing word {}", greek_word);

    let details = search_word_details(greek_word, declension).await?;
    dbg!(details.clone());

    let inflections = extract_inflections(&details.inflection_lemma).await?;
    dbg!(inflections.clone());

    Ok(ParsingResult {
        details,
        inflections,
    })
}
