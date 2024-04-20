use crate::{
    api::lexicon::lexicon_model::{
        InflectionForm, LexiconEntryDefinition, NounInflectionCases, NounInflectionGenders,
        NounInflectionNumbers, WordInflection,
    },
    error::SafeError,
    grammar::{Case, Gender, Number},
    scrappers::wiki::table::parse_declension_table,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{
    definition, page,
    table::{get_words_dialects, ParsedWord, ParsingComp},
};

pub struct ScrappedPronoun {
    pub inflections: Vec<WordInflection>,
    pub definitions: Vec<LexiconEntryDefinition>,
}
pub async fn scrap_pronoun(lemma: &str) -> Result<ScrappedPronoun, SafeError> {
    let doc = page::scrap(lemma).await?;

    let selector = select(".NavFrame.grc-decl.grc-adecl")?;
    let decl_tables = doc.select(&selector);

    let mut inflections = vec![];
    for table in decl_tables {
        let words = parse_declension_table(&table)?;
        let infl = parsed_words_to_inflection(&words);
        let dialects = get_words_dialects(&words);
        inflections.push(WordInflection {
            dialects,
            pronoun: Some(Box::from(infl)),
            ..Default::default()
        });
    }

    let definitions = scrap_pronoun_defs(&doc)?;

    Ok(ScrappedPronoun {
        inflections,
        definitions,
    })
}

pub fn scrap_pronoun_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Pronoun")?)
        .next()
        .with_context(|| "cannot find pronoun header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}

fn parsed_words_to_inflection(words: &[ParsedWord]) -> NounInflectionGenders {
    let mut infl = NounInflectionGenders::default();

    for word in words {
        fill_genders(word, &mut infl);
    }

    infl
}

fn fill_genders(word: &ParsedWord, genders: &mut NounInflectionGenders) {
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

fn fill_forms(word: &ParsedWord, forms: &mut Vec<InflectionForm>) {
    for part in word.text.split('\n') {
        forms.push(InflectionForm {
            contracted: Some(part.into()),
            ..Default::default()
        })
    }
}
