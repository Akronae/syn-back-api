use tracing::*;

use crate::{
    api::lexicon::lexicon_model::{LexiconEntry, WordInflection},
    borrow::Cow,
    error::SafeError,
    grammar::{Declension, PartOfSpeech},
    scrappers::wiki::{details::search_word_details, noun},
};

pub struct ParseWordResult {
    pub entry: LexiconEntry,
    pub declension: Declension,
}

#[allow(dead_code)]
pub async fn parse_word(
    greek_word: Cow<str>,
    declension: &Declension,
) -> Result<ParseWordResult, SafeError> {
    info!("Parsing word {}", greek_word);

    let details = search_word_details(greek_word, &declension.part_of_speech).await?;
    debug!("{:?}", details.clone());

    let mut inflections = Vec::new();
    let definitions;
    let mut declension = declension.clone();

    if let PartOfSpeech::Noun(_) = declension.part_of_speech {
        let noun = noun::scrap_noun(&details.lemma, &declension).await?;
        if noun.inflection.is_some() {
            inflections.push(WordInflection {
                noun: noun.inflection,
                ..Default::default()
            });
        }
        definitions = noun.definitions;
        declension = noun.declension;
    } else {
        todo!()
    }

    Ok(ParseWordResult {
        entry: LexiconEntry {
            lemma: details.lemma.into(),
            inflections,
            definitions,
        },
        declension: declension,
    })
}
