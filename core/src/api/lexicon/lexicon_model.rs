use serde::{Deserialize, Serialize};

use crate::grammar::{Declension, DeclensionType, Dialect};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LexiconEntry {
    pub lemma: String,
    pub inflections: Vec<WordInflection>,
    pub definitions: Vec<LexiconEntryDefinition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LexiconEntryDefinition {
    Litteral(String),
    FormOf(String),
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct WordInflection {
    pub dialects: Vec<Dialect>,
    pub declension_type: Option<DeclensionType>,
    pub noun: Option<Box<NounInflectionGenders>>,
    pub article: Option<Box<NounInflectionGenders>>,
    pub pronoun: Option<Box<NounInflectionGenders>>,
    pub verb: Option<Box<VerbInflectionTenses>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionTenses {
    pub present: Option<Box<VerbInflectionThemes>>,
    pub imperfect: Option<Box<VerbInflectionThemes>>,
    pub future: Option<Box<VerbInflectionThemes>>,
    pub aorist: Option<Box<VerbInflectionThemes>>,
    pub aorist_2nd: Option<Box<VerbInflectionThemes>>,
    pub perfect: Option<Box<VerbInflectionThemes>>,
    pub perfect_2nd: Option<Box<VerbInflectionThemes>>,
    pub future_perfect: Option<Box<VerbInflectionThemes>>,
    pub pluperfect: Option<Box<VerbInflectionThemes>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionThemes {
    pub thematic: Option<VerbInflectionContractions>,
    pub athematic: Option<VerbInflectionContractions>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionContractions {
    pub contracted: Option<VerbInflectionMoods>,
    pub uncontracted: Option<VerbInflectionMoods>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionMoods {
    pub indicative: Option<VerbInflectionVoices>,
    pub subjunctive: Option<VerbInflectionVoices>,
    pub optative: Option<VerbInflectionVoices>,
    pub imperative: Option<VerbInflectionVoices>,
    pub infinitive: Option<VerbInflectionInfinitive>,
    pub participle: Option<VerbInflectionParticiple>,
    pub pluperfect: Option<VerbInflectionVoices>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionInfinitive {
    pub active: Option<Vec<VerbInflectionForm>>,
    pub middle: Option<Vec<VerbInflectionForm>>,
    pub passive: Option<Vec<VerbInflectionForm>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionParticiple {
    pub active: Option<NounInflectionGenders>,
    pub middle: Option<NounInflectionGenders>,
    pub passive: Option<NounInflectionGenders>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionVoices {
    pub active: Option<VerbInflectionNumbers>,
    pub middle: Option<VerbInflectionNumbers>,
    pub passive: Option<VerbInflectionNumbers>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionNumbers {
    pub singular: Option<VerbInflectionPersons>,
    pub plural: Option<VerbInflectionPersons>,
    pub dual: Option<VerbInflectionPersons>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionPersons {
    pub first: Option<Vec<VerbInflectionForm>>,
    pub second: Option<Vec<VerbInflectionForm>>,
    pub third: Option<Vec<VerbInflectionForm>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct VerbInflectionForm {
    pub contracted: Option<String>,
    pub uncontracted: Option<Vec<String>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct NounInflectionGenders {
    pub masculine: Option<NounInflectionNumbers>,
    pub feminine: Option<NounInflectionNumbers>,
    pub neuter: Option<NounInflectionNumbers>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct NounInflectionNumbers {
    pub singular: Option<NounInflectionCases>,
    pub dual: Option<NounInflectionCases>,
    pub plural: Option<NounInflectionCases>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct NounInflectionCases {
    pub nominative: Option<Vec<NounInflectionForm>>,
    pub genitive: Option<Vec<NounInflectionForm>>,
    pub dative: Option<Vec<NounInflectionForm>>,
    pub accusative: Option<Vec<NounInflectionForm>>,
    pub vocative: Option<Vec<NounInflectionForm>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq)]
pub struct NounInflectionForm {
    pub contracted: Option<String>,
    pub uncontracted: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LexiconFilter {
    pub lemma: Option<String>,
    pub inflection: Option<LexiconFilterInflection>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LexiconFilterInflection {
    pub word: String,
    pub declension: Declension,
}
