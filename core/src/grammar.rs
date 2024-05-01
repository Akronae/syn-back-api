use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::Display;

use crate::texts::{Book, Collection};

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum PartOfSpeech {
    Verb,
    Adverb,
    Preposition,
    Particle,
    Interjection,
    Quantifier,
    #[serde(untagged)]
    Numeral(Numeral),
    #[serde(untagged)]
    Noun(Noun),
    #[serde(untagged)]
    Pronoun(Pronoun),
    #[serde(untagged)]
    Article(Article),
    #[serde(untagged)]
    Adjective(Adjective),
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Noun {
    #[serde(rename = "noun_common")]
    Common,
    #[serde(rename = "noun_proper")]
    Proper,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
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

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Article {
    #[serde(rename = "article_definite")]
    Definite,
    #[serde(rename = "article_indefinite")]
    Indefinite,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Adjective {
    #[serde(rename = "adjective_positive")]
    Positive,
    #[serde(rename = "adjective_comparative")]
    Comparative,
    #[serde(rename = "adjective_superlative")]
    Superlative,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Numeral {
    #[serde(rename = "numeral_cardinal")]
    Cardinal,
    #[serde(rename = "numeral_ordinal")]
    Ordinal,
    #[serde(rename = "numeral_adverbial")]
    Adverbial,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Number {
    Singular,
    Dual,
    Plural,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Case {
    Vocative,
    Nominative,
    Accusative,
    Dative,
    Genitive,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Voice {
    Active,
    Middle,
    Passive,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
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

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    FuturePerfect,
    Aorist,
    Perfect,
    Pluperfect,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Theme {
    Thematic,
    Athematic,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Contraction {
    Contracted,
    Uncontracted,
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
    Indeclinable,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
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
    pub contraction: Option<Contraction>,
    pub decl_type: Option<DeclensionType>,
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
            contraction: Default::default(),
            decl_type: Default::default(),
        }
    }
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Greek,
    English,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
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
    pub chapter_number: u8,
    pub verse_number: u8,
    pub translation: HashMap<LanguageCode, String>,
    pub words: Vec<Word>,
}

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Display, Hash, Eq, PartialOrd, Ord,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Dialect {
    Attic,
    Koine,
    Epic,
    Laconian,
    Doric,
    Ionic,
    Aeolic,
    Homeric,
    Arcadocypriot,
    Cretan,
    Macedonian,
}
