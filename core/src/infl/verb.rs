use crate::{
    api::lexicon::lexicon_model::{
        InflectionForm, VerbInflectionContractions, VerbInflectionMoods, VerbInflectionNumbers,
        VerbInflectionPersons, VerbInflectionTenses, VerbInflectionThemes, VerbInflectionVoices,
    },
    error::SafeError,
    utils::str::remove_diacritics::remove_diacritics_char,
};

#[allow(dead_code)]
pub fn inflect(lemma: &str) -> Result<VerbInflectionTenses, SafeError> {
    if lemma.ends_with('ω') {
        return Ok(inflect_w(lemma));
    }
    Err("cannot inflect verb".into())
}

fn inflect_w(lemma: &str) -> VerbInflectionTenses {
    VerbInflectionTenses {
        present: Some(Box::from(inflect_w_pres(lemma))),
        ..Default::default()
    }
}

fn inflect_w_pres(lemma: &str) -> VerbInflectionThemes {
    VerbInflectionThemes {
        thematic: Some(VerbInflectionContractions {
            contracted: Some(VerbInflectionMoods {
                indicative: Some(inflect_w_pre_ind(lemma)),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn uncontracted(stem: &str, ending: &str) -> Option<Vec<InflectionForm>> {
    Some(vec![InflectionForm {
        uncontracted: Some(vec![stem.to_string(), ending.to_string()]),
        ..Default::default()
    }])
}

fn is_vowel(c: char) -> bool {
    matches!(
        remove_diacritics_char(c),
        'α' | 'ε' | 'η' | 'ι' | 'ο' | 'υ' | 'ω'
    )
}

fn split_syllables(word: &str) {
    let mut syllables = Vec::new();
    let mut syllable = String::new();
    let mut last_vowel = false;

    for c in word.chars() {
        if is_vowel(c) {
            if last_vowel {
                syllables.push(syllable);
                syllable = String::new();
            }
            last_vowel = true;
        } else {
            last_vowel = false;
        }
        syllable.push(c);
    }
    syllables.push(syllable);
}

fn inflect_w_pre_ind(lemma: &str) -> VerbInflectionVoices {
    let stem = &lemma[..lemma.len() - 1];
    dbg!(split_syllables(stem));

    VerbInflectionVoices {
        active: Some(VerbInflectionNumbers {
            singular: Some(VerbInflectionPersons {
                first: uncontracted(stem, "ω"),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}
