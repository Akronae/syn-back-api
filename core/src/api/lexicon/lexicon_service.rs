use crate::{
    error::SafeError,
    grammar::{
        Case, Contraction, Declension, Gender, Mood, Number, PartOfSpeech, Person, Tense, Theme,
        Voice,
    },
};

use super::{
    lexicon_model::{
        InflectionForm, LexiconEntry, LexiconFilter, NounInflectionGenders,
        VerbInflectionInfinitive, VerbInflectionParticiple, VerbInflectionTenses, WordInflection,
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
    pub fn find_inflection(&self, declension: &Declension) -> Vec<String> {
        if matches!(declension.part_of_speech, PartOfSpeech::Noun(_)) {
            let noun = self.noun.as_ref().unwrap();

            find_inflection_noun(declension, noun)
        } else if matches!(declension.part_of_speech, PartOfSpeech::Verb) {
            let verb = self.verb.as_ref().unwrap();

            return find_inflection_verb(declension, verb);
        } else if matches!(declension.part_of_speech, PartOfSpeech::Article(_)) {
            let article = self.article.as_ref().unwrap();

            return find_inflection_noun(declension, article);
        } else if matches!(declension.part_of_speech, PartOfSpeech::Pronoun(_)) {
            let pronoun = self.pronoun.as_ref().unwrap();

            return find_inflection_noun(declension, pronoun);
        } else if matches!(declension.part_of_speech, PartOfSpeech::Conjunction)
            || matches!(declension.part_of_speech, PartOfSpeech::Preposition)
        {
            return vec![];
        } else {
            panic!(
                "Unsupported part of speech: {:?}",
                declension.part_of_speech
            );
        }
    }
}

fn find_inflection_verb(declension: &Declension, verb: &VerbInflectionTenses) -> Vec<String> {
    let tense = match declension.tense {
        Some(Tense::Aorist) => verb.aorist.as_ref().unwrap(),
        Some(Tense::Aorist2nd) => verb.aorist_2nd.as_ref().unwrap(),
        Some(Tense::Future) => verb.future.as_ref().unwrap(),
        Some(Tense::FuturePerfect) => verb.future_perfect.as_ref().unwrap(),
        Some(Tense::Imperfect) => verb.imperfect.as_ref().unwrap(),
        Some(Tense::Perfect) => verb.perfect.as_ref().unwrap(),
        Some(Tense::Perfect2nd) => verb.perfect_2nd.as_ref().unwrap(),
        Some(Tense::Pluperfect) => verb.pluperfect.as_ref().unwrap(),
        Some(Tense::Present) => verb.present.as_ref().unwrap(),
        None => panic!("No tense found for {:?}", declension),
    };
    let theme = match declension.theme {
        Some(Theme::Athematic) => tense.athematic.as_ref().unwrap(),
        Some(Theme::Thematic) | None => tense.thematic.as_ref().unwrap(),
    };
    let contraction = match declension.contraction {
        Some(Contraction::Uncontracted) => theme.uncontracted.as_ref().unwrap(),
        Some(Contraction::Contracted) | None => theme.contracted.as_ref().unwrap(),
    };
    let mood = match declension.mood {
        Some(mood) => match mood {
            Mood::Indicative => contraction.indicative.as_ref().unwrap(),
            Mood::Imperative => contraction.imperative.as_ref().unwrap(),
            Mood::Optative => contraction.optative.as_ref().unwrap(),
            Mood::Subjunctive => contraction.subjunctive.as_ref().unwrap(),
            Mood::Infinitive => {
                return find_inflection_verb_infinitive(
                    declension,
                    contraction.infinitive.as_ref().unwrap(),
                )
            }
            Mood::Participle => {
                return find_inflection_verb_participle(
                    declension,
                    contraction.participle.as_ref().unwrap(),
                )
            }
        },
        None => panic!("No mood found for {:?}", declension),
    };
    let voice = match declension.voice {
        Some(voice) => match voice {
            Voice::Active => mood.active.as_ref().unwrap(),
            Voice::Middle => mood.middle.as_ref().unwrap(),
            Voice::Passive => mood.passive.as_ref().unwrap(),
        },
        None => panic!("No voice found for {:?}", declension),
    };
    let number = match declension.number {
        Some(Number::Singular) => voice.singular.as_ref().unwrap(),
        Some(Number::Plural) => voice.plural.as_ref().unwrap(),
        Some(Number::Dual) => panic!(
            "Dual is not a supported number for verb inflection. Found in {:?}",
            declension
        ),
        None => panic!("No number found for {:?}", declension),
    };
    let person = match declension.person {
        Some(person) => match person {
            Person::First => number.first.as_ref().unwrap(),
            Person::Second => number.second.as_ref().unwrap(),
            Person::Third => number.third.as_ref().unwrap(),
        },
        None => panic!("No person found for {:?}", declension),
    };

    find_inflection_verb_form(person)
}

fn find_inflection_noun(declension: &Declension, noun: &NounInflectionGenders) -> Vec<String> {
    let gender = match declension.gender {
        Some(Gender::Masculine) => noun.masculine.as_ref().unwrap(),
        Some(Gender::Feminine) => noun.feminine.as_ref().unwrap(),
        Some(Gender::Neuter) => noun.neuter.as_ref().unwrap(),
        None => return vec![],
    };
    let number = match declension.number {
        Some(Number::Singular) => gender.singular.as_ref().unwrap(),
        Some(Number::Dual) => gender.dual.as_ref().unwrap(),
        Some(Number::Plural) => gender.plural.as_ref().unwrap(),
        None => return vec![],
    };
    let case = match declension.case {
        Some(Case::Nominative) => number.nominative.as_ref().unwrap(),
        Some(Case::Genitive) => number.genitive.as_ref().unwrap(),
        Some(Case::Dative) => number.dative.as_ref().unwrap(),
        Some(Case::Accusative) => number.accusative.as_ref().unwrap(),
        Some(Case::Vocative) => number.vocative.as_ref().unwrap(),
        None => return vec![],
    };
    return case.iter().flat_map(|x| x.contracted.clone()).collect();
}

fn find_inflection_verb_infinitive(
    declension: &Declension,
    infinitive: &VerbInflectionInfinitive,
) -> Vec<String> {
    match declension.voice {
        Some(Voice::Active) => {
            return find_inflection_verb_form(infinitive.active.as_ref().unwrap())
        }
        Some(Voice::Middle) => {
            return find_inflection_verb_form(infinitive.middle.as_ref().unwrap())
        }
        Some(Voice::Passive) => {
            return find_inflection_verb_form(infinitive.passive.as_ref().unwrap())
        }
        None => panic!("No voice found for {:?}", declension),
    }
}

fn find_inflection_verb_participle(
    declension: &Declension,
    participle: &VerbInflectionParticiple,
) -> Vec<String> {
    match declension.voice {
        Some(Voice::Active) => {
            return find_inflection_noun(declension, participle.active.as_ref().unwrap())
        }
        Some(Voice::Middle) => {
            return find_inflection_noun(declension, participle.middle.as_ref().unwrap())
        }
        Some(Voice::Passive) => {
            return find_inflection_noun(declension, participle.passive.as_ref().unwrap())
        }
        None => panic!("No voice found for {:?}", declension),
    }
}

fn find_inflection_verb_form(form: &[InflectionForm]) -> Vec<String> {
    return form.iter().flat_map(|x| x.contracted.clone()).collect();
}
