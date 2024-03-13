use crate::{
    api::lexicon::lexicon_model::{
        LexiconEntryDefinition, NounInflectionCases, NounInflectionForm, NounInflectionGenders,
        NounInflectionNumbers,
    },
    borrow::Cow,
    error::SafeError,
    grammar::{Case, Declension, Gender, Noun, Number, PartOfSpeech},
    scrappers::wiki::table::parse_declension_table,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{
    definition, page,
    table::{ParsedWord, ParsingComp},
};

pub struct ScrappedNoun {
    pub inflection: Option<NounInflectionGenders>,
    pub definitions: Vec<LexiconEntryDefinition>,
    pub declension: Declension,
}
pub async fn scrap_noun(lemma: &str, declension: &Declension) -> Result<ScrappedNoun, SafeError> {
    let doc = page::scrap(lemma).await?;

    let gender = doc
        .select(&select(".gender")?)
        .next()
        .context("cannot find gender")?
        .text()
        .collect::<Cow<str>>()
        .trim()
        .to_lowercase();
    let gender = match gender.as_str() {
        "f" => Gender::Feminine,
        "m" => Gender::Masculine,
        "n" => Gender::Neuter,
        _ => return Err(format!("cannot match gender '{gender}' for {lemma}").into()),
    };
    let mut declension = declension.clone();
    declension.gender = Some(gender);

    let mut inflection = None;
    if !declension.indeclinable.unwrap_or(false) {
        let selector = select(".NavFrame")?;
        let decl_table = doc
            .select(&selector)
            .next()
            .context("cannot find declension table")?;

        let words = parse_declension_table(&decl_table)?;
        let infl = parsed_words_to_inflection(&words);

        let mut genders = NounInflectionGenders::default();
        match gender {
            Gender::Feminine => genders.feminine = Some(infl),
            Gender::Masculine => genders.masculine = Some(infl),
            Gender::Neuter => genders.neuter = Some(infl),
        }
        inflection = Some(genders);
    }

    let noun = match declension.part_of_speech {
        PartOfSpeech::Noun(x) => x,
        _ => return Err(format!("expected a noun declension for {lemma}").into()),
    };
    let definitions = scrap_noun_defs(&doc, &noun)?;

    Ok(ScrappedNoun {
        inflection,
        definitions,
        declension,
    })
}

pub fn scrap_noun_defs(doc: &Html, noun: &Noun) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    match noun {
        Noun::Common => scrap_common_noun_defs(doc),
        Noun::Proper => scrap_proper_noun_defs(doc),
    }
}

fn scrap_common_noun_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Noun")?)
        .next()
        .with_context(|| "cannot find common noun header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}

fn scrap_proper_noun_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Proper_noun")?)
        .next()
        .with_context(|| "cannot find common noun header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}

fn parsed_words_to_inflection(words: &[ParsedWord]) -> NounInflectionNumbers {
    let mut infl = NounInflectionNumbers::default();

    for word in words {
        fill_numbers(word, &mut infl);
    }

    infl
}

fn fill_numbers(word: &ParsedWord, numbers: &mut NounInflectionNumbers) {
    if word
        .parsing
        .contains(&ParsingComp::Number(Number::Singular))
    {
        if numbers.singular.is_none() {
            numbers.singular = Some(Default::default());
        }
        let singular = numbers.singular.as_mut().unwrap();

        fill_cases(word, singular);
    }
    if word.parsing.contains(&ParsingComp::Number(Number::Plural)) {
        if numbers.plural.is_none() {
            numbers.plural = Some(Default::default());
        }
        let plural = numbers.plural.as_mut().unwrap();

        fill_cases(word, plural);
    }
    if word.parsing.contains(&ParsingComp::Number(Number::Dual)) {
        if numbers.dual.is_none() {
            numbers.dual = Some(Default::default());
        }
        let dual = numbers.dual.as_mut().unwrap();

        fill_cases(word, dual);
    }
}

fn fill_cases(word: &ParsedWord, cases: &mut NounInflectionCases) {
    if word.parsing.contains(&ParsingComp::Case(Case::Nominative)) {
        if cases.nominative.is_none() {
            cases.nominative = Some(Default::default());
        }
        let nominative = cases.nominative.as_mut().unwrap();
        fill_forms(word, nominative);
    }
    if word.parsing.contains(&ParsingComp::Case(Case::Genitive)) {
        if cases.genitive.is_none() {
            cases.genitive = Some(Default::default());
        }
        let genitive = cases.genitive.as_mut().unwrap();
        fill_forms(word, genitive);
    }
    if word.parsing.contains(&ParsingComp::Case(Case::Dative)) {
        if cases.dative.is_none() {
            cases.dative = Some(Default::default());
        }
        let dative = cases.dative.as_mut().unwrap();
        fill_forms(word, dative);
    }
    if word.parsing.contains(&ParsingComp::Case(Case::Accusative)) {
        if cases.accusative.is_none() {
            cases.accusative = Some(Default::default());
        }
        let accusative = cases.accusative.as_mut().unwrap();
        fill_forms(word, accusative);
    }
    if word.parsing.contains(&ParsingComp::Case(Case::Vocative)) {
        if cases.vocative.is_none() {
            cases.vocative = Some(Default::default());
        }
        let vocative = cases.vocative.as_mut().unwrap();
        fill_forms(word, vocative);
    }
}

fn fill_forms(word: &ParsedWord, forms: &mut Vec<NounInflectionForm>) {
    for part in word.text.split('\n') {
        forms.push(NounInflectionForm {
            contracted: Some(part.into()),
            ..Default::default()
        })
    }
}

pub fn fill_genders(word: &ParsedWord, genders: &mut NounInflectionGenders) {
    if word
        .parsing
        .contains(&ParsingComp::Gender(Gender::Feminine))
    {
        if genders.feminine.is_none() {
            genders.feminine = Some(Default::default());
        }
        let feminine = genders.feminine.as_mut().unwrap();
        fill_numbers(word, feminine);
    }
    if word
        .parsing
        .contains(&ParsingComp::Gender(Gender::Masculine))
    {
        if genders.masculine.is_none() {
            genders.masculine = Some(Default::default());
        }
        let masculine = genders.masculine.as_mut().unwrap();
        fill_numbers(word, masculine);
    }
    if word.parsing.contains(&ParsingComp::Gender(Gender::Neuter)) {
        if genders.neuter.is_none() {
            genders.neuter = Some(Default::default());
        }
        let neuter = genders.neuter.as_mut().unwrap();
        fill_numbers(word, neuter);
    }
}
