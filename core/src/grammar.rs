use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Noun {
    Common,
    Proper,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Article {
    Definite,
    Indefinite,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Case {
    Vocative,
    Nominative,
    Accusative,
    Dative,
    Genitive,
    Indeclinable,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Voice {
    Active,
    Middle,
    Passive,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Mood {
    Indicative,
    Subjunctive,
    Optative,
    Imperative,
    Infinitive,
    Participle,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Theme {
    Thematic,
    Athematic,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub language: Language,
    pub text: String,
    pub translation: HashMap<Language, String>,
    pub declension: Declension,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verse {
    pub collection: String,
    pub book: String,
    pub chapter_number: isize,
    pub verse_number: isize,
    pub translation: HashMap<Language, String>,
    pub words: Vec<Word>,
}
