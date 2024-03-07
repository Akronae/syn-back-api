use tracing::*;

use crate::{
    api::lexicon::lexicon_model::{LexiconEntry, WordInflection},
    borrow::Cow,
    error::SafeError,
    grammar::{Declension, PartOfSpeech},
    scrappers::wiki::{details::search_word_details, noun},
};

#[allow(dead_code)]
pub async fn parse_word(
    greek_word: Cow<str>,
    declension: &Declension,
) -> Result<LexiconEntry, SafeError> {
    info!("Parsing word {}", greek_word);

    let details = search_word_details(greek_word, &declension.part_of_speech).await?;
    dbg!(details.clone());

    let mut inflections = Vec::new();
    let definitions;

    if let PartOfSpeech::Noun(_) = declension.part_of_speech {
        let noun = noun::scrap_noun(&details.lemma, declension).await?;
        if noun.inflection.is_some() {
            inflections.push(WordInflection {
                noun: noun.inflection,
                ..Default::default()
            });
        }
        definitions = noun.definitions;
    } else {
        todo!()
    }

    Ok(LexiconEntry {
        lemma: details.lemma.into(),
        inflections,
        definitions,
    })
}
