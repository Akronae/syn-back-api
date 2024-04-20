use tracing::*;

use crate::{
    api::lexicon::lexicon_model::LexiconEntry,
    borrow::Cow,
    grammar::{Declension, Mood, PartOfSpeech},
    scrappers::wiki::{
        article, conjunction,
        details::{search_word_details, SearchMode},
        noun, participle, preposition, pronoun, verb,
    },
};

use super::errors::ParseWordError;

pub struct ParseWordResult {
    pub entry: LexiconEntry,
    pub declension: Declension,
}

#[allow(dead_code)]
pub async fn parse_word(
    greek_word: Cow<str>,
    declension: &Declension,
    mode: &SearchMode,
) -> Result<ParseWordResult, ParseWordError> {
    info!("Parsing word {}", greek_word);

    let details = search_word_details(greek_word.clone(), declension, mode).await?;
    debug!("{:?}", details.clone());

    let mut lemma = details.lemma.clone();
    let mut inflections = Vec::new();
    let definitions;
    let mut declension = declension.clone();

    if let PartOfSpeech::Noun(_) = declension.part_of_speech {
        let noun = noun::scrap_noun(&details.lemma, &declension).await?;
        inflections.extend(noun.inflections);
        definitions = noun.definitions;
        declension = noun.declension;
    } else if PartOfSpeech::Verb == declension.part_of_speech {
        if matches!(declension.mood, Some(Mood::Participle)) {
            let participle = participle::scrap_participle(&details.lemma).await?;
            inflections.extend(participle.inflections);
            definitions = vec![];
            lemma = participle.verb_lemma.into();
        } else {
            let verb = verb::scrap_verb(&details.lemma).await?;
            definitions = verb.definitions;
            inflections.extend(verb.inflections);
        }
    } else if matches!(declension.part_of_speech, PartOfSpeech::Article(_)) {
        let article = article::scrap_article(&details.lemma).await?;
        definitions = article.definitions;
        inflections.extend(article.inflections);
    } else if matches!(declension.part_of_speech, PartOfSpeech::Pronoun(_)) {
        let pronoun = pronoun::scrap_pronoun(&details.lemma).await?;
        definitions = pronoun.definitions;
        inflections.extend(pronoun.inflections);
    } else if matches!(declension.part_of_speech, PartOfSpeech::Conjunction) {
        let conjunction = conjunction::scrap_conjunction(&details.lemma).await?;
        definitions = conjunction.definitions;
    } else if matches!(declension.part_of_speech, PartOfSpeech::Preposition) {
        let preposition = preposition::scrap_preposition(&details.lemma).await?;
        definitions = preposition.definitions;
    } else {
        panic!(
            "Unsupported part of speech: {:?}",
            declension.part_of_speech
        );
    }

    Ok(ParseWordResult {
        entry: LexiconEntry {
            lemma: lemma.into(),
            inflections,
            definitions,
        },
        declension,
    })
}
