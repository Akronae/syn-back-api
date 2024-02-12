use std::{collections::HashMap};

use serde::{Deserialize, Serialize};
use strum::Display;

use crate::texts::{Book, Collection};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum PartOfSpeech {
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
    Participle,
    Numeral,
    Determiner,
    #[serde(untagged)]
    Noun(Noun),
    #[serde(untagged)]
    Pronoun(Pronoun),
    #[serde(untagged)]
    Article(Article),
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Noun {
    #[serde(rename = "noun_common")]
    Common,
    #[serde(rename = "noun_proper")]
    Proper,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Pronoun {
    #[serde(rename = "pronoun_relative")]
    Relative,
    #[serde(rename = "pronoun_interrogative")]
    Interrogative,
    #[serde(rename = "pronoun_indefinite")]
    Indefinite,
    #[serde(rename = "pronoun_reciprocal")]
    Reciprocal,
    #[serde(rename = "pronoun_reflexive")]
    Reflexive,
    #[serde(rename = "pronoun_demonstrative")]
    Demonstrative,
    #[serde(rename = "pronoun_personal")]
    Personal,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Article {
    #[serde(rename = "article_definite")]
    Definite,
    #[serde(rename = "article_indefinite")]
    Indefinite,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Case {
    Vocative,
    Nominative,
    Accusative,
    Dative,
    Genitive,
    Indeclinable,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Voice {
    Active,
    Middle,
    Passive,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Mood {
    Indicative,
    Subjunctive,
    Optative,
    Imperative,
    Infinitive,
    Participle,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Aorist,
    Aorist2nd,
    Perfect,
    Perfect2nd,
    Pluperfect,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Theme {
    Thematic,
    Athematic,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, PartialOrd, Eq, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum DeclensionType {
    First,
    Second,
    Third,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Declension {
    pub part_of_speech: PartOfSpeech,
    pub mood: Option<Mood>,
    pub person: Option<Person>,
    pub number: Option<Number>,
    pub gender: Option<Gender>,
    pub case: Option<Case>,
    pub voice: Option<Voice>,
    pub tense: Option<Tense>,
    pub theme: Option<Theme>,
}

impl Declension {
    pub fn partial_default(pos: PartOfSpeech) -> Self {
        Self {
            part_of_speech: pos,
            mood: Default::default(),
            person: Default::default(),
            number: Default::default(),
            gender: Default::default(),
            case: Default::default(),
            voice: Default::default(),
            tense: Default::default(),
            theme: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Greek,
    English,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Display, Hash)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LanguageCode {
    Grc,
    En,
}

impl Language {
    pub fn lang_code(&self) -> LanguageCode {
        match &self {
            Language::Greek => LanguageCode::Grc,
            Language::English => LanguageCode::En,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    pub language: LanguageCode,
    pub text: String,
    pub translation: HashMap<LanguageCode, String>,
    pub declension: Declension,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Verse {
    pub collection: Collection,
    pub book: Book,
    pub chapter_number: isize,
    pub verse_number: isize,
    pub translation: HashMap<LanguageCode, String>,
    pub words: Vec<Word>,
}
