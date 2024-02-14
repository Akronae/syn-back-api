use crate::{
    error::SafeError,
    grammar::{Case, Declension, Gender, Number, PartOfSpeech},
};

use super::{
    lexicon_model::{LexiconEntry, LexiconFilter, WordInflection},
    lexicon_repo::LexiconRepo,
};

pub struct LexiconService {}

impl LexiconService {
    pub fn new() -> Self {
        LexiconService {}
    }

    pub async fn find_one(&self, filter: LexiconFilter) -> Result<Option<LexiconEntry>, SafeError> {
        LexiconRepo::find_one(filter).await
    }
}

impl WordInflection {
    pub fn find_inflection(&self, declension: &Declension) -> Option<String> {
        if matches!(declension.part_of_speech, PartOfSpeech::Noun(_)) {
            let noun = self.noun.as_ref()?;

            let gender = match declension.gender {
                Some(Gender::Masculine) => Some(noun.masculine.as_ref()?),
                Some(Gender::Feminine) => Some(noun.feminine.as_ref()?),
                Some(Gender::Neuter) => Some(noun.neuter.as_ref()?),
                None => None,
            }?;

            let number = match declension.number {
                Some(Number::Singular) => Some(gender.singular.as_ref()?),
                Some(Number::Plural) => Some(gender.plural.as_ref()?),
                None => None,
            }?;

            let case = match declension.case {
                Some(Case::Nominative) => Some(number.nominative.as_ref()?),
                Some(Case::Genitive) => Some(number.genitive.as_ref()?),
                Some(Case::Dative) => Some(number.dative.as_ref()?),
                Some(Case::Accusative) => Some(number.accusative.as_ref()?),
                Some(Case::Vocative) => Some(number.vocative.as_ref()?),
                None => None,
            }?;

            return case.contracted.clone();
        }

        None
    }
}
