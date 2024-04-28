use tracing::*;

use crate::{
    api::lexicon::lexicon_model::LexiconEntry,
    borrow::Cow,
    grammar::{Declension, Mood, PartOfSpeech},
    scrappers::wiki::{
        adverb, article,
        details::{search_word_details, SearchMode},
        noun, numeral, participle, particle, preposition, pronoun, quantifier, verb,
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
            inflections.extend(verb.inflections);
            definitions = verb.definitions;
        }
    } else if matches!(declension.part_of_speech, PartOfSpeech::Article(_)) {
        let article = article::scrap_article(&details.lemma).await?;
        inflections.extend(article.inflections);
        definitions = article.definitions;
    } else if matches!(declension.part_of_speech, PartOfSpeech::Pronoun(_)) {
        let pronoun = pronoun::scrap_pronoun(&details.lemma).await?;
        inflections.extend(pronoun.inflections);
        definitions = pronoun.definitions;
    } else if matches!(declension.part_of_speech, PartOfSpeech::Particle) {
        let particle = particle::scrap_particle(&details.lemma).await?;
        inflections.extend(particle.inflections);
        definitions = particle.definitions;
    } else if matches!(declension.part_of_speech, PartOfSpeech::Preposition) {
        let preposition = preposition::scrap_preposition(&details.lemma).await?;
        inflections.extend(preposition.inflections);
        definitions = preposition.definitions;
    } else if matches!(declension.part_of_speech, PartOfSpeech::Quantifier) {
        let quantifier = quantifier::scrap_quantifier(&details.lemma).await?;
        inflections.extend(quantifier.inflections);
        definitions = quantifier.definitions;
    } else if matches!(declension.part_of_speech, PartOfSpeech::Adverb) {
        let adverb = adverb::scrap_adverb(&details.lemma).await?;
        inflections.extend(adverb.inflections);
        definitions = adverb.definitions;
    } else if matches!(declension.part_of_speech, PartOfSpeech::Numeral(_)) {
        let numeral = numeral::scrap_numeral(&details.lemma).await?;
        inflections.extend(numeral.inflections);
        definitions = numeral.definitions;
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
