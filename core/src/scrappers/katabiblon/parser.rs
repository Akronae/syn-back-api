use tracing::*;

use crate::{
    api::lexicon::lexicon_model::{LexiconEntry, LexiconEntryDefinition},
    error::SafeError,
    grammar::{Declension, PartOfSpeech},
    scrappers::katabiblon::{details::search_word_details, noun},
};

#[allow(dead_code)]
pub async fn parse_word(
    greek_word: &str,
    declension: &Declension,
) -> Result<LexiconEntry, SafeError> {
    info!("Parsing word {}", greek_word);

    let details = search_word_details(greek_word, declension).await?;
    debug!("{:?}", details.clone());

    let inflections = match details.declension.part_of_speech {
        PartOfSpeech::Noun(_) => vec![noun::inflect(&details.lemma, &details.declension)?],
        _ => Vec::new(),
    };

    let mut definitions = Vec::new();
    if !details.translation.is_empty() {
        definitions.push(LexiconEntryDefinition::Litteral(details.translation));
    }
    if !details.description.is_empty() {
        definitions.push(LexiconEntryDefinition::Litteral(details.description));
    }

    Ok(LexiconEntry {
        lemma: details.lemma,
        inflections,
        definitions,
    })
}
