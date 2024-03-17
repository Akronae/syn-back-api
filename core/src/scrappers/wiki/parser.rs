use tracing::*;

use crate::{
    api::lexicon::lexicon_model::{LexiconEntry, WordInflection},
    borrow::Cow,
    error::SafeError,
    grammar::{Declension, PartOfSpeech},
    scrappers::wiki::{details::search_word_details, noun, verb},
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

    let details = search_word_details(greek_word, declension).await?;
    debug!("{:?}", details.clone());

    let mut inflections = Vec::new();
    let definitions;
    let mut declension = declension.clone();

    if let PartOfSpeech::Noun(_) = declension.part_of_speech {
        let mut noun = noun::scrap_noun(&details.lemma, &declension).await?;
        inflections.extend(noun.inflections.iter_mut().map(|x| Box::new(x.to_owned())));
        definitions = noun.definitions;
        declension = noun.declension;
    } else if PartOfSpeech::Verb == declension.part_of_speech {
        let mut verb = verb::scrap_verb(&details.lemma, &declension).await?;
        definitions = verb.definitions;
        inflections.extend(verb.inflections.iter_mut().map(|x| Box::new(x.to_owned())));
    } else {
        todo!()
    }

    Ok(ParseWordResult {
        entry: LexiconEntry {
            lemma: details.lemma.into(),
            inflections,
            definitions,
        },
        declension,
    })
}
