use crate::{
    api::lexicon::lexicon_model::{
        InflectionForm, NounInflectionCases, NounInflectionGenders, NounInflectionNumbers,
    },
    error::SafeError,
    grammar::{Declension, DeclensionType, Gender},
};

pub fn inflect(lemma: &str, declension: &Declension) -> Result<NounInflectionGenders, SafeError> {
    let numbers = match declension.decl_type {
        Some(DeclensionType::First) => inflect_1st(lemma, declension)?,
        _ => {
            return Err(format!(
                "could not match declension type for {lemma}: {:?}",
                declension.decl_type
            )
            .into())
        }
    };

    let genders = match declension.gender {
        Some(Gender::Feminine) => NounInflectionGenders {
            feminine: Some(*numbers),
            ..Default::default()
        },
        Some(Gender::Masculine) => NounInflectionGenders {
            masculine: Some(*numbers),
            ..Default::default()
        },
        Some(Gender::Neuter) => NounInflectionGenders {
            neuter: Some(*numbers),
            ..Default::default()
        },
        None => return Err(format!("could not gendr for {lemma}").into()),
    };

    Ok(genders)
}

fn inflect_1st(
    lemma: &str,
    declension: &Declension,
) -> Result<Box<NounInflectionNumbers>, SafeError> {
    match declension.gender {
        Some(Gender::Feminine) => inflect_1st_fem(lemma),
        Some(Gender::Masculine) => inflect_1st_mas(lemma),
        Some(Gender::Neuter) => {
            Err(format!("1st decl is not supposed to have neuter, for {lemma}").into())
        }
        None => todo!(),
    }
}

fn inflect_1st_fem(lemma: &str) -> Result<Box<NounInflectionNumbers>, SafeError> {
    match lemma.chars().last() {
        Some('η') => Ok(conjugate(
            lemma.trim_end_matches('η'),
            &get_1st_fem_h_endings(),
        )),
        Some('ᾱ') => Ok(conjugate(
            lemma.trim_end_matches('ᾱ'),
            &get_1st_fem_a_macron_endings(),
        )),
        Some('ᾰ') => Ok(conjugate(
            lemma.trim_end_matches('ᾰ'),
            &get_1st_fem_a_breve_endings(),
        )),
        Some('α') => Ok(conjugate(
            lemma.trim_end_matches('α'),
            &get_1st_fem_a_breve_endings(),
        )),
        _ => Err(format!("could not match lemma {lemma}").into()),
    }
}

fn inflect_1st_mas(lemma: &str) -> Result<Box<NounInflectionNumbers>, SafeError> {
    match lemma {
        x if x.ends_with("ης") => Ok(conjugate(
            lemma.trim_end_matches("ης"),
            &get_1st_mas_hs_endings(),
        )),
        x if x.ends_with("ᾱς") => Ok(conjugate(
            lemma.trim_end_matches("ᾱς"),
            &get_1st_mas_as_endings(),
        )),
        x if x.ends_with("ας") => Ok(conjugate(
            lemma.trim_end_matches("ας"),
            &get_1st_mas_as_endings(),
        )),
        _ => Err(format!("could not match lemma {lemma}").into()),
    }
}

fn conjugate_inflection_forms(root: &str, forms: &[InflectionForm]) -> Vec<InflectionForm> {
    forms
        .iter()
        .map(|ending| InflectionForm {
            contracted: Some(format!("{}{}", root, ending.contracted.as_ref().unwrap())),
            uncontracted: Some(vec![
                root.to_string(),
                ending.contracted.as_ref().unwrap().to_string(),
            ]),
        })
        .collect()
}

fn conjugate_inflection_cases(root: &str, cases: &NounInflectionCases) -> NounInflectionCases {
    NounInflectionCases {
        nominative: Some(conjugate_inflection_forms(
            root,
            cases.nominative.as_ref().unwrap(),
        )),
        vocative: Some(conjugate_inflection_forms(
            root,
            cases.vocative.as_ref().unwrap(),
        )),
        accusative: Some(conjugate_inflection_forms(
            root,
            cases.accusative.as_ref().unwrap(),
        )),
        genitive: Some(conjugate_inflection_forms(
            root,
            cases.genitive.as_ref().unwrap(),
        )),
        dative: Some(conjugate_inflection_forms(
            root,
            cases.dative.as_ref().unwrap(),
        )),
    }
}

fn conjugate(root: &str, endings: &NounInflectionNumbers) -> Box<NounInflectionNumbers> {
    Box::from(NounInflectionNumbers {
        singular: Some(conjugate_inflection_cases(
            root,
            endings.singular.as_ref().unwrap(),
        )),
        dual: Some(conjugate_inflection_cases(
            root,
            endings.dual.as_ref().unwrap(),
        )),
        plural: Some(conjugate_inflection_cases(
            root,
            endings.plural.as_ref().unwrap(),
        )),
    })
}

fn get_1st_eta_du_endings() -> Box<NounInflectionCases> {
    Box::from(NounInflectionCases {
        nominative: Some(vec![InflectionForm {
            contracted: Some("ᾱ".to_string()),
            ..Default::default()
        }]),
        vocative: Some(vec![InflectionForm {
            contracted: Some("ᾱ".to_string()),
            ..Default::default()
        }]),
        accusative: Some(vec![InflectionForm {
            contracted: Some("ᾱ".to_string()),
            ..Default::default()
        }]),
        genitive: Some(vec![InflectionForm {
            contracted: Some("αιν".to_string()),
            ..Default::default()
        }]),
        dative: Some(vec![InflectionForm {
            contracted: Some("αιν".to_string()),
            ..Default::default()
        }]),
    })
}

fn get_1st_eta_pl_endings() -> Box<NounInflectionCases> {
    Box::from(NounInflectionCases {
        nominative: Some(vec![InflectionForm {
            contracted: Some("αι".to_string()),
            ..Default::default()
        }]),
        vocative: Some(vec![InflectionForm {
            contracted: Some("αι".to_string()),
            ..Default::default()
        }]),
        accusative: Some(vec![InflectionForm {
            contracted: Some("ᾱς".to_string()),
            ..Default::default()
        }]),
        genitive: Some(vec![InflectionForm {
            contracted: Some("ων".to_string()),
            ..Default::default()
        }]),
        dative: Some(vec![InflectionForm {
            contracted: Some("αις".to_string()),
            ..Default::default()
        }]),
    })
}

fn get_1st_fem_h_endings() -> Box<NounInflectionNumbers> {
    Box::from(NounInflectionNumbers {
        singular: Some(NounInflectionCases {
            nominative: Some(vec![InflectionForm {
                contracted: Some("η".to_string()),
                ..Default::default()
            }]),
            vocative: Some(vec![InflectionForm {
                contracted: Some("ης".to_string()),
                ..Default::default()
            }]),
            accusative: Some(vec![InflectionForm {
                contracted: Some("ην".to_string()),
                ..Default::default()
            }]),
            genitive: Some(vec![InflectionForm {
                contracted: Some("ης".to_string()),
                ..Default::default()
            }]),
            dative: Some(vec![InflectionForm {
                contracted: Some("ῃ".to_string()),
                ..Default::default()
            }]),
        }),
        dual: Some(*get_1st_eta_du_endings()),
        plural: Some(*get_1st_eta_pl_endings()),
    })
}

fn get_1st_fem_a_breve_endings() -> Box<NounInflectionNumbers> {
    Box::from(NounInflectionNumbers {
        singular: Some(NounInflectionCases {
            nominative: Some(vec![InflectionForm {
                contracted: Some("ᾰ".to_string()),
                ..Default::default()
            }]),
            vocative: Some(vec![InflectionForm {
                contracted: Some("ᾰ".to_string()),
                ..Default::default()
            }]),
            accusative: Some(vec![InflectionForm {
                contracted: Some("ᾰν".to_string()),
                ..Default::default()
            }]),
            genitive: Some(vec![InflectionForm {
                contracted: Some("ᾱς".to_string()),
                ..Default::default()
            }]),
            dative: Some(vec![InflectionForm {
                contracted: Some("ᾳ".to_string()),
                ..Default::default()
            }]),
        }),
        dual: Some(*get_1st_eta_du_endings()),
        plural: Some(*get_1st_eta_pl_endings()),
    })
}

fn get_1st_fem_a_macron_endings() -> Box<NounInflectionNumbers> {
    Box::from(NounInflectionNumbers {
        singular: Some(NounInflectionCases {
            nominative: Some(vec![InflectionForm {
                contracted: Some("ᾱ".to_string()),
                ..Default::default()
            }]),
            vocative: Some(vec![InflectionForm {
                contracted: Some("ᾱ".to_string()),
                ..Default::default()
            }]),
            accusative: Some(vec![InflectionForm {
                contracted: Some("ᾱν".to_string()),
                ..Default::default()
            }]),
            genitive: Some(vec![InflectionForm {
                contracted: Some("ᾱς".to_string()),
                ..Default::default()
            }]),
            dative: Some(vec![InflectionForm {
                contracted: Some("ᾳ".to_string()),
                ..Default::default()
            }]),
        }),
        dual: Some(*get_1st_eta_du_endings()),
        plural: Some(*get_1st_eta_pl_endings()),
    })
}

fn get_1st_mas_hs_endings() -> Box<NounInflectionNumbers> {
    Box::from(NounInflectionNumbers {
        singular: Some(NounInflectionCases {
            nominative: Some(vec![InflectionForm {
                contracted: Some("ης".to_string()),
                ..Default::default()
            }]),
            vocative: Some(vec![InflectionForm {
                contracted: Some("η".to_string()),
                ..Default::default()
            }]),
            accusative: Some(vec![InflectionForm {
                contracted: Some("ην".to_string()),
                ..Default::default()
            }]),
            genitive: Some(vec![InflectionForm {
                contracted: Some("ου".to_string()),
                ..Default::default()
            }]),
            dative: Some(vec![InflectionForm {
                contracted: Some("ῃ".to_string()),
                ..Default::default()
            }]),
        }),
        dual: Some(*get_1st_eta_du_endings()),
        plural: Some(*get_1st_eta_pl_endings()),
    })
}

fn get_1st_mas_as_endings() -> Box<NounInflectionNumbers> {
    Box::from(NounInflectionNumbers {
        singular: Some(NounInflectionCases {
            nominative: Some(vec![InflectionForm {
                contracted: Some("ᾱς".to_string()),
                ..Default::default()
            }]),
            vocative: Some(vec![InflectionForm {
                contracted: Some("ᾱ".to_string()),
                ..Default::default()
            }]),
            accusative: Some(vec![InflectionForm {
                contracted: Some("ᾱν".to_string()),
                ..Default::default()
            }]),
            genitive: Some(vec![InflectionForm {
                contracted: Some("ου".to_string()),
                ..Default::default()
            }]),
            dative: Some(vec![InflectionForm {
                contracted: Some("ᾳ".to_string()),
                ..Default::default()
            }]),
        }),
        dual: Some(*get_1st_eta_du_endings()),
        plural: Some(*get_1st_eta_pl_endings()),
    })
}
