use crate::{
    error::SafeError,
    grammar::{Case, Declension, Gender, Mood, Number, PartOfSpeech, Person, Tense, Theme, Voice},
};

use super::{
    lexicon_model::{
        LexiconEntry, LexiconFilter, NounInflectionGenders, VerbInflectionForm,
        VerbInflectionTenses, WordInflection,
    },
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
            let noun = self.noun.as_ref().unwrap();

            return find_inflection_noun(declension, noun);
        } else if matches!(declension.part_of_speech, PartOfSpeech::Verb) {
            let verb = self.verb.as_ref().unwrap();

            return find_inflection_verb(declension, verb);
        }

        None
    }
}

fn find_inflection_verb(declension: &Declension, verb: &VerbInflectionTenses) -> Option<String> {
    let tense = match declension.tense {
        Some(Tense::Aorist) => Some(verb.aorist.as_ref().unwrap()),
        Some(Tense::Aorist2nd) => Some(verb.aorist_2nd.as_ref().unwrap()),
        Some(Tense::Future) => Some(verb.future.as_ref().unwrap()),
        Some(Tense::FuturePerfect) => Some(verb.future_perfect.as_ref().unwrap()),
        Some(Tense::Imperfect) => Some(verb.imperfect.as_ref().unwrap()),
        Some(Tense::Perfect) => Some(verb.perfect.as_ref().unwrap()),
        Some(Tense::Perfect2nd) => Some(verb.perfect_2nd.as_ref().unwrap()),
        Some(Tense::Pluperfect) => Some(verb.pluperfect.as_ref().unwrap()),
        Some(Tense::Present) => Some(verb.present.as_ref().unwrap()),
        None => panic!("No tense found for {:?}", declension),
    }?;
    let theme = match declension.theme {
        Some(Theme::Athematic) => Some(tense.athematic.as_ref().unwrap()),
        Some(Theme::Thematic) | None => Some(tense.thematic.as_ref().unwrap()),
    }?;
    let mood = match declension.mood {
        Some(mood) => match mood {
            Mood::Indicative => Some(theme.indicative.as_ref().unwrap()),
            Mood::Imperative => Some(theme.imperative.as_ref().unwrap()),
            Mood::Optative => Some(theme.optative.as_ref().unwrap()),
            Mood::Subjunctive => Some(theme.subjunctive.as_ref().unwrap()),
            Mood::Infinitive => {
                return find_inflection_verb_form(
                    theme.infinitive.as_ref().unwrap().first().unwrap(),
                )
            }
            Mood::Participle => {
                return find_inflection_noun(declension, theme.participle.as_ref().unwrap())
            }
        },
        None => panic!("No mood found for {:?}", declension),
    }?;
    let voice = match declension.voice {
        Some(voice) => match voice {
            Voice::Active => Some(mood.active.as_ref().unwrap()),
            Voice::Middle => Some(mood.middle.as_ref().unwrap()),
            Voice::Passive => Some(mood.passive.as_ref().unwrap()),
        },
        None => panic!("No voice found for {:?}", declension),
    }?;
    let number = match declension.number {
        Some(Number::Singular) => Some(voice.singular.as_ref().unwrap()),
        Some(Number::Plural) => Some(voice.plural.as_ref().unwrap()),
        Some(Number::Dual) => panic!(
            "Dual is not a supported number for verb inflection. Found in {:?}",
            declension
        ),
        None => panic!("No number found for {:?}", declension),
    }?;
    let person = match declension.person {
        Some(person) => match person {
            Person::First => Some(number.first.as_ref().unwrap()),
            Person::Second => Some(number.second.as_ref().unwrap()),
            Person::Third => Some(number.third.as_ref().unwrap()),
        },
        None => panic!("No person found for {:?}", declension),
    }?;

    return find_inflection_verb_form(person.first().unwrap());
}

fn find_inflection_noun(declension: &Declension, noun: &NounInflectionGenders) -> Option<String> {
    let gender = match declension.gender {
        Some(Gender::Masculine) => Some(noun.masculine.as_ref().unwrap()),
        Some(Gender::Feminine) => Some(noun.feminine.as_ref().unwrap()),
        Some(Gender::Neuter) => Some(noun.neuter.as_ref().unwrap()),
        None => None,
    }?;
    let number = match declension.number {
        Some(Number::Singular) => Some(gender.singular.as_ref().unwrap()),
        Some(Number::Dual) => Some(gender.dual.as_ref().unwrap()),
        Some(Number::Plural) => Some(gender.plural.as_ref().unwrap()),
        None => None,
    }?;
    let case = match declension.case {
        Some(Case::Nominative) => Some(number.nominative.as_ref().unwrap()),
        Some(Case::Genitive) => Some(number.genitive.as_ref().unwrap()),
        Some(Case::Dative) => Some(number.dative.as_ref().unwrap()),
        Some(Case::Accusative) => Some(number.accusative.as_ref().unwrap()),
        Some(Case::Vocative) => Some(number.vocative.as_ref().unwrap()),
        None => None,
    }?;
    return case.first().unwrap().contracted.clone();
}

fn find_inflection_verb_form(form: &VerbInflectionForm) -> Option<String> {
    if form.contracted.is_none() {
        panic!("No contracted form found for {:?}", form)
    }
    form.contracted.clone()
}
