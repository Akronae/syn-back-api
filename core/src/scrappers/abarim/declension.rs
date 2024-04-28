use crate::{
    grammar::{
        Adjective, Article, Case, Declension, DeclensionType, Gender, Mood, Noun, Number, Numeral,
        PartOfSpeech, Person, Pronoun, Tense, Voice,
    },
    texts::Book,
};

pub fn get_word_fix(
    book: Book,
    chapter: u8,
    verse: u8,
    word: u8,
    greek: &str,
) -> Option<Declension> {
    let onar = Some(Declension {
        gender: Some(Gender::Neuter),
        number: Some(Number::Singular),
        case: Some(Case::Nominative),
        ..Declension::partial_default(PartOfSpeech::Noun(Noun::Common))
    });

    let routh = Declension {
        gender: Some(Gender::Feminine),
        number: Some(Number::Singular),
        ..Declension::partial_default(PartOfSpeech::Noun(Noun::Proper))
    };

    if book == Book::Matthew && chapter == 0 && verse == 15 && word == 13 {
        return onar;
    }
    if greek == "οναρ" {
        return onar;
    }

    if book == Book::Matthew && chapter == 1 && verse == 5 && word == 15 {
        return Some(Declension {
            case: Some(Case::Genitive),
            ..routh
        });
    }

    None
}

fn is_cardinal(str: &str) -> bool {
    if str == "teen" || str == "for" || str == "bi" || str == "tri" {
        return false;
    }

    let number_parts = [
        "one", "two", "three", "four", "five", "six", "seven", "height", "nine", "ten", "eleven",
        "twelve", "thir", "fif", "twen", "thir", "for", "hundred", "thousand", "mi", "bi", "tri",
        "quadr", "teen", "ty", "illion",
    ];

    let mut str = str.to_string();
    for part in number_parts {
        str = str.replace(part, "");
    }

    str.is_empty()
}

fn is_ordinal(str: &str) -> bool {
    let parts = ["st", "nd", "rd", "th"];

    for part in parts {
        if str.ends_with(part) {
            let str = str.replace(part, "");
            return is_cardinal(&str);
        }
    }

    false
}

pub fn fix_declension(greek: &str, english: &str, declension: &mut Declension) {
    if greek == "πας" || greek == "παση" || greek == "πασαι" {
        declension.part_of_speech = PartOfSpeech::Quantifier;
    }

    if matches!(declension.part_of_speech, PartOfSpeech::Adjective(_)) {
        if is_cardinal(english) {
            declension.part_of_speech = PartOfSpeech::Numeral(Numeral::Cardinal);
        } else if is_ordinal(english) {
            declension.part_of_speech = PartOfSpeech::Numeral(Numeral::Ordinal);
        }
    }
}

pub fn get_word_declension(comps: &Vec<String>) -> Declension {
    let comp_0_opt = comps
        .first()
        .map(|s| {
            s.to_lowercase()[..s.find("+kai").unwrap_or(s.len())]
                .trim()
                .to_owned()
        })
        .unwrap_or("".to_string());
    let comp_0 = comp_0_opt.as_str();
    let comp_1_opt = comps
        .get(1)
        .map(|s| {
            s.to_lowercase()[..s.find("+kai").unwrap_or(s.len())]
                .trim()
                .to_owned()
        })
        .unwrap_or("".to_string());
    let comp_1 = comp_1_opt.as_str();
    let comp_2_opt = comps
        .get(2)
        .map(|s| {
            s.to_lowercase()[..s.find("+kai").unwrap_or(s.len())]
                .trim()
                .to_owned()
        })
        .unwrap_or("".to_string());
    let comp_2 = comp_2_opt.as_str();

    let comp_1_dash_splits = comp_1.split('-').collect::<Vec<&str>>();
    let comp_2_dash_splits = comp_2.split('-').collect::<Vec<&str>>();
    let comp_2_space_splits = comp_2.split(' ').collect::<Vec<&str>>();

    let pos = match comp_0 {
        "noun" => PartOfSpeech::Noun(Noun::Common),
        "noun (name)" => PartOfSpeech::Noun(Noun::Proper),
        "verb" => PartOfSpeech::Verb,
        "participle" => PartOfSpeech::Verb,
        "def art" => PartOfSpeech::Article(Article::Definite),
        "conjunction" => PartOfSpeech::Particle,
        "preposition" => PartOfSpeech::Preposition,
        "rel pron" => PartOfSpeech::Pronoun(Pronoun::Relative),
        "dem pron" => PartOfSpeech::Pronoun(Pronoun::Demonstrative),
        "adjective" => PartOfSpeech::Adjective(Adjective::Positive),
        "adjective (name)" => PartOfSpeech::Adjective(Adjective::Positive),
        "adverb" => PartOfSpeech::Adverb,
        s if s.ends_with("pers pron") => PartOfSpeech::Pronoun(Pronoun::Personal),
        default => panic!("unknown part of speech: {default} in comps: {:?}", comps),
    };

    if comp_1 == "indeclinable" {
        return Declension {
            part_of_speech: pos,
            decl_type: Some(DeclensionType::Indeclinable),
            ..Declension::partial_default(pos)
        };
    }

    let mood = match pos {
        PartOfSpeech::Verb => match comp_1_dash_splits.to_owned() {
            c if c.contains(&"ind") => Some(Mood::Indicative),
            c if c.contains(&"sub") => Some(Mood::Subjunctive),
            c if c.contains(&"imp") => Some(Mood::Imperative),
            c if c.contains(&"opt") => Some(Mood::Optative),
            c if c.contains(&"inf") => Some(Mood::Infinitive),
            c if c.contains(&"par") => Some(Mood::Participle),
            _ => panic!(
                "cannot find mood with comps: {:?}, comp_1_dash_splits: {:?}",
                comps, comp_1_dash_splits
            ),
        },
        PartOfSpeech::Noun(_)
        | PartOfSpeech::Adjective(_)
        | PartOfSpeech::Article(_)
        | PartOfSpeech::Adverb
        | PartOfSpeech::Particle
        | PartOfSpeech::Quantifier
        | PartOfSpeech::Interjection
        | PartOfSpeech::Numeral(_)
        | PartOfSpeech::Preposition
        | PartOfSpeech::Pronoun(_) => None,
    };

    let comp_person = match pos {
        PartOfSpeech::Verb => comp_2,
        PartOfSpeech::Pronoun(_) => comp_0,
        _ => comp_1,
    };
    let extract_person = || match comp_person {
        c if c.contains("1st") => Some(Person::First),
        c if c.contains("2nd") => Some(Person::Second),
        c if c.contains("3rd") => Some(Person::Third),
        _ => panic!(
            "cannot find person with comps: {:?}",
            (comps, pos.to_owned())
        ),
    };
    let person = match pos {
        PartOfSpeech::Article(Article::Definite)
        | PartOfSpeech::Noun(_)
        | PartOfSpeech::Particle
        | PartOfSpeech::Preposition
        | PartOfSpeech::Pronoun(Pronoun::Relative)
        | PartOfSpeech::Pronoun(Pronoun::Demonstrative)
        | PartOfSpeech::Adjective(_)
        | PartOfSpeech::Adverb
        | PartOfSpeech::Interjection => None,

        PartOfSpeech::Article(_)
        | PartOfSpeech::Quantifier
        | PartOfSpeech::Pronoun(_)
        | PartOfSpeech::Numeral(_) => extract_person(),
        PartOfSpeech::Verb => match mood {
            Some(Mood::Infinitive) | Some(Mood::Participle) => None,
            _ => extract_person(),
        },
    };

    let number_comp = match pos {
        PartOfSpeech::Verb => match mood {
            Some(Mood::Participle) => comp_2_dash_splits.to_owned(),
            _ => comp_2_space_splits.to_owned(),
        },
        _ => comp_1_dash_splits.to_owned(),
    };
    let extract_number = || match number_comp {
        s if s.contains(&"si") => Some(Number::Singular),
        s if s.contains(&"pl") => Some(Number::Plural),
        _ => panic!(
            "cannot find number with comps: {:?} in {:?}",
            comps, number_comp
        ),
    };
    let number = match pos {
        PartOfSpeech::Pronoun(Pronoun::Personal)
        | PartOfSpeech::Pronoun(Pronoun::Relative)
        | PartOfSpeech::Pronoun(Pronoun::Demonstrative)
        | PartOfSpeech::Adjective(_)
        | PartOfSpeech::Noun(_)
        | PartOfSpeech::Article(_) => extract_number(),
        PartOfSpeech::Verb => match mood {
            Some(Mood::Infinitive) => None,
            _ => extract_number(),
        },
        PartOfSpeech::Particle | PartOfSpeech::Preposition | PartOfSpeech::Adverb => None,
        _ => panic!("cannot find number with comps: {:?}", comps),
    };

    let comp_gender = if mood == Some(Mood::Participle) {
        comp_2
    } else {
        comp_1
    };

    let extract_gender = || match comp_gender {
        val if val.ends_with("-mas") => Some(Gender::Masculine),
        val if val.ends_with("-fem") => Some(Gender::Feminine),
        val if val.ends_with("-neu") => Some(Gender::Neuter),
        _ => panic!(
            " cannot find gender with comps: {:?} and pos {:?}",
            comps, pos
        ),
    };

    let gender = match pos {
        PartOfSpeech::Pronoun(Pronoun::Personal)
            if person != Some(Person::Third) || number != Some(Number::Singular) =>
        {
            None
        }
        PartOfSpeech::Noun(Noun::Common)
        | PartOfSpeech::Noun(Noun::Proper)
        | PartOfSpeech::Pronoun(_)
        | PartOfSpeech::Article(_)
        | PartOfSpeech::Adjective(_) => extract_gender(),
        PartOfSpeech::Verb => {
            if matches!(mood, Some(Mood::Participle)) {
                extract_gender()
            } else {
                None
            }
        }
        _ => None,
    };

    let extract_case = |comp: &[&str]| match comp.to_owned() {
        s if s.contains(&"nom") => Some(Case::Nominative),
        s if s.contains(&"gen") => Some(Case::Genitive),
        s if s.contains(&"dat") => Some(Case::Dative),
        s if s.contains(&"acc") => Some(Case::Accusative),
        s if s.contains(&"voc") => Some(Case::Vocative),
        _ => panic!("cannot find case with comps: {:?}", comps),
    };

    let case = match pos {
        PartOfSpeech::Particle | PartOfSpeech::Preposition | PartOfSpeech::Adverb => None,
        PartOfSpeech::Verb => match mood {
            Some(Mood::Participle) => extract_case(&comp_2_dash_splits),
            _ => None,
        },
        _ => extract_case(&comp_1_dash_splits),
    };

    let voice_comp = comp_1_dash_splits.to_owned();
    let voice = match pos {
        PartOfSpeech::Noun(_)
        | PartOfSpeech::Article(_)
        | PartOfSpeech::Pronoun(_)
        | PartOfSpeech::Preposition
        | PartOfSpeech::Adjective(_)
        | PartOfSpeech::Adverb
        | PartOfSpeech::Particle => None,
        _ => match voice_comp {
            s if s.contains(&"act") => Some(Voice::Active),
            s if s.contains(&"mid")
                | s.contains(&"mde")
                | s.contains(&"mi/pde")
                | s.contains(&"mi/pas") =>
            {
                Some(Voice::Middle)
            }
            s if s.contains(&"pas") | s.contains(&"pde") => Some(Voice::Passive),
            _ => panic!(
                "cannot find voice with comps: {:?} in {:?}",
                comps, voice_comp
            ),
        },
    };

    let tense = match pos {
        PartOfSpeech::Noun(_)
        | PartOfSpeech::Article(_)
        | PartOfSpeech::Particle
        | PartOfSpeech::Adverb
        | PartOfSpeech::Adjective(_)
        | PartOfSpeech::Interjection
        | PartOfSpeech::Preposition
        | PartOfSpeech::Pronoun(_) => None,
        _ => match comp_1_dash_splits.to_owned() {
            s if s.contains(&"pres") => Some(Tense::Present),
            s if s.contains(&"imp") => Some(Tense::Imperfect),
            s if s.contains(&"fut") => Some(Tense::Future),
            s if s.contains(&"aor") => Some(Tense::Aorist),
            s if s.contains(&"2aor") => Some(Tense::Aorist2nd),
            s if s.contains(&"perf") => Some(Tense::Perfect),
            s if s.contains(&"2perf") => Some(Tense::Perfect2nd),
            s if s.contains(&"plup") => Some(Tense::Pluperfect),
            _ => panic!(
                "cannot find tense with comps: {:?} in {:?}",
                comps, comp_1_dash_splits
            ),
        },
    };

    Declension {
        gender,
        number,
        person,
        mood,
        case,
        voice,
        tense,
        ..Declension::partial_default(pos)
    }
}
