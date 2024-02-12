use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use strum::Display;

use crate::texts::{Book, Collection};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum PartOfSpeech {
    Noun(Noun),
    Pronoun(Pronoun),
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
    Participle,
    Numeral,
    Article(Article),
    Determiner,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Noun {
    Common,
    Proper,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Pronoun {
    Relative,
    Interrogative,
    Indefinite,
    Reciprocal,
    Reflexive,
    Demonstrative,
    Personal,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Article {
    Definite,
    Indefinite,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Voice {
    Active,
    Middle,
    Passive,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Theme {
    Thematic,
    Athematic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Language {
    Greek,
    English,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, Display)]
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
pub struct Word {
    pub language: LanguageCode,
    pub text: String,
    pub translation: HashMap<LanguageCode, String>,
    pub declension: Declension,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Verse {
    pub collection: Collection,
    pub book: Book,
    pub chapter_number: isize,
    pub verse_number: isize,
    pub translation: HashMap<LanguageCode, String>,
    pub words: Vec<Word>,
}
