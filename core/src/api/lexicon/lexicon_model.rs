use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LexiconEntry {
    pub lemma: Vec<String>,
    pub translation: String,
    pub description: String,
    pub inflections: Vec<WordInflection>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash)]
pub struct WordInflection {
    pub noun: Option<NounInflectionDeclensions>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash)]
pub struct NounInflectionDeclensions {
    pub first_declension: Option<NounInflectionGenders>,
    pub second_declension: Option<NounInflectionGenders>,
    pub third_declension: Option<NounInflectionGenders>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash)]
pub struct NounInflectionGenders {
    pub masculine: Option<NounInflectionNumbers>,
    pub feminine: Option<NounInflectionNumbers>,
    pub neuter: Option<NounInflectionNumbers>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash)]
pub struct NounInflectionNumbers {
    pub singular: Option<NounInflectionCases>,
    pub plural: Option<NounInflectionCases>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash)]
pub struct NounInflectionCases {
    pub nominative: Option<NounInflectionForm>,
    pub genitive: Option<NounInflectionForm>,
    pub dative: Option<NounInflectionForm>,
    pub accusative: Option<NounInflectionForm>,
    pub vocative: Option<NounInflectionForm>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash)]
pub struct NounInflectionForm {
    pub contracted: String,
    pub uncontracted: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LexiconFilter {
    pub lemma: Option<String>,
}
