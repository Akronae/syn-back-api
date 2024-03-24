use tracing::*;

use crate::{
    api::lexicon::lexicon_model::{LexiconEntry},
    borrow::Cow,
    error::SafeError,
    grammar::{Declension, PartOfSpeech},
    scrappers::wiki::{article, conjunction, details::search_word_details, noun, verb},
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
        let mut verb = verb::scrap_verb(&details.lemma).await?;
        definitions = verb.definitions;
        inflections.extend(verb.inflections.iter_mut().map(|x| Box::new(x.to_owned())));
    } else if matches!(declension.part_of_speech, PartOfSpeech::Article(_)) {
        let mut article = article::scrap_article(&details.lemma).await?;
        definitions = article.definitions;
        inflections.extend(
            article
                .inflections
                .iter_mut()
                .map(|x| Box::new(x.to_owned())),
        );
    } else if matches!(declension.part_of_speech, PartOfSpeech::Conjunction) {
        let conjunction = conjunction::scrap_conjunction(&details.lemma).await?;
        definitions = conjunction.definitions;
    } else {
        panic!(
            "Unsupported part of speech: {:?}",
            declension.part_of_speech
        );
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
