use crate::{
    api::lexicon::lexicon_model::{
        LexiconEntryDefinition, NounInflectionCases, NounInflectionForm, NounInflectionGenders,
        NounInflectionNumbers,
    },
    borrow::Cow,
    error::SafeError,
    grammar::{Case, Declension, Noun, Number, PartOfSpeech},
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
}
pub async fn scrap_noun(lemma: &str, declension: &Declension) -> Result<ScrappedNoun, SafeError> {
    let doc = page::scrap(lemma).await?;

    let mut inflection = None;
    if !declension.indeclinable.unwrap_or(false) {
        let selector = select(".NavFrame")?;
        let decl_table = doc
            .select(&selector)
            .next()
            .context("cannot find declension table")?;

        let words = parse_declension_table(&decl_table)?;
        let infl = parsed_words_to_inflection(&words);

        let gender = doc
            .select(&select(".gender")?)
            .next()
            .context("cannot find gender")?
            .text()
            .collect::<Cow<str>>()
            .trim()
            .to_lowercase();

        let mut genders = NounInflectionGenders::default();
        match gender.as_str() {
            "f" => genders.feminine = Some(infl),
            "m" => genders.masculine = Some(infl),
            "n" => genders.neuter = Some(infl),
            _ => return Err(format!("cannot match gender for {lemma}").into()),
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
        if word
            .parsing
            .contains(&ParsingComp::Number(Number::Singular))
        {
            if infl.singular.is_none() {
                infl.singular = Some(Default::default());
            }
            let singular = infl.singular.as_mut().unwrap();

            fill_cases(word, singular);
        }
        if word.parsing.contains(&ParsingComp::Number(Number::Plural)) {
            if infl.plural.is_none() {
                infl.plural = Some(Default::default());
            }
            let plural = infl.plural.as_mut().unwrap();

            fill_cases(word, plural);
        }
    }

    infl
}

fn fill_cases(word: &ParsedWord, cases: &mut NounInflectionCases) {
    if word.parsing.contains(&ParsingComp::Case(Case::Nominative)) {
        if cases.nominative.is_none() {
            cases.nominative = Some(Default::default());
        }
        let nominative = cases.nominative.as_mut().unwrap();
        nominative.push(NounInflectionForm {
            contracted: Some(word.text.clone().into()),
            ..Default::default()
        })
    }
    if word.parsing.contains(&ParsingComp::Case(Case::Genitive)) {
        if cases.genitive.is_none() {
            cases.genitive = Some(Default::default());
        }
        let genitive = cases.genitive.as_mut().unwrap();
        genitive.push(NounInflectionForm {
            contracted: Some(word.text.clone().into()),
            ..Default::default()
        })
    }
    if word.parsing.contains(&ParsingComp::Case(Case::Dative)) {
        if cases.dative.is_none() {
            cases.dative = Some(Default::default());
        }
        let dative = cases.dative.as_mut().unwrap();
        dative.push(NounInflectionForm {
            contracted: Some(word.text.clone().into()),
            ..Default::default()
        })
    }
    if word.parsing.contains(&ParsingComp::Case(Case::Accusative)) {
        if cases.accusative.is_none() {
            cases.accusative = Some(Default::default());
        }
        let accusative = cases.accusative.as_mut().unwrap();
        accusative.push(NounInflectionForm {
            contracted: Some(word.text.clone().into()),
            ..Default::default()
        })
    }
    if word.parsing.contains(&ParsingComp::Case(Case::Vocative)) {
        if cases.vocative.is_none() {
            cases.vocative = Some(Default::default());
        }
        let vocative = cases.vocative.as_mut().unwrap();
        vocative.push(NounInflectionForm {
            contracted: Some(word.text.clone().into()),
            ..Default::default()
        })
    }
}
