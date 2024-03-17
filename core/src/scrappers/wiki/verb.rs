use crate::{
    api::lexicon::lexicon_model::{
        LexiconEntryDefinition, VerbInflectionContractions, VerbInflectionForm,
        VerbInflectionMoods, VerbInflectionNumbers, VerbInflectionPersons, VerbInflectionTenses,
        VerbInflectionThemes, VerbInflectionVoices, WordInflection,
    },
    error::SafeError,
    grammar::{Contraction, Declension, Mood, Number, Person, Tense, Voice},
    scrappers::wiki::table::parse_declension_table,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{
    definition, noun, page,
    table::{get_words_dialects, ParsedWord, ParsingComp},
};

pub struct ScrappedVerb {
    pub inflections: Vec<WordInflection>,
    pub definitions: Vec<LexiconEntryDefinition>,
}
pub async fn scrap_verb(lemma: &str, _declension: &Declension) -> Result<ScrappedVerb, SafeError> {
    let doc = page::scrap(lemma).await?;

    let selector = select(".NavFrame")?;
    let decl_tables = doc.select(&selector);

    let mut inflections = vec![WordInflection::default()];
    for table in decl_tables {
        let words = parse_declension_table(&table)?;
        let infl = parsed_words_to_inflection(&words);
        let tense = words
            .first()
            .unwrap()
            .parsing
            .iter()
            .find_map(|x| {
                if let ParsingComp::Tense(t) = x {
                    Some(t)
                } else {
                    None
                }
            })
            .with_context(|| {
                format!(
                    "cannot get any tense parsing comp in {:?} with words {:?}",
                    words.first().unwrap(),
                    words
                )
            })?;
        let dialects = get_words_dialects(&words);

        let mut avail_infl = inflections.iter_mut().find_map(|x| {
            if grab_tense_field(x, tense).is_none()
                && x.dialects.iter().all(|y| dialects.contains(y))
            {
                Some(x)
            } else {
                None
            }
        });

        if avail_infl.is_none() {
            inflections.push(WordInflection {
                dialects,
                ..Default::default()
            });
            avail_infl = inflections.last_mut();
        }
        let avail_infl = avail_infl.as_mut().unwrap();

        set_tense_field(avail_infl, tense, infl);
    }

    let definitions = scrap_verb_defs(&doc)?;

    Ok(ScrappedVerb {
        inflections,
        definitions,
    })
}

fn grab_tense_field<'a>(
    infl: &'a mut WordInflection,
    tense: &Tense,
) -> Option<&'a mut VerbInflectionThemes> {
    let tenses = infl.verb.as_mut()?;
    match tense {
        Tense::Present => tenses.present.as_mut(),
        Tense::Imperfect => tenses.imperfect.as_mut(),
        Tense::Future => tenses.future.as_mut(),
        Tense::Aorist => tenses.aorist.as_mut(),
        Tense::Perfect => tenses.perfect.as_mut(),
        Tense::Pluperfect => tenses.pluperfect.as_mut(),
        Tense::FuturePerfect => tenses.future_perfect.as_mut(),
        Tense::Aorist2nd => tenses.aorist_2nd.as_mut(),
        Tense::Perfect2nd => tenses.perfect_2nd.as_mut(),
    }
}

fn set_tense_field(infl: &mut WordInflection, tense: &Tense, value: VerbInflectionThemes) {
    let tenses = infl.verb.get_or_insert(VerbInflectionTenses::default());
    match tense {
        Tense::Present => tenses.present = Some(value),
        Tense::Imperfect => tenses.imperfect = Some(value),
        Tense::Future => tenses.future = Some(value),
        Tense::Aorist => tenses.aorist = Some(value),
        Tense::Perfect => tenses.perfect = Some(value),
        Tense::Pluperfect => tenses.pluperfect = Some(value),
        Tense::FuturePerfect => tenses.future_perfect = Some(value),
        Tense::Aorist2nd => tenses.aorist_2nd = Some(value),
        Tense::Perfect2nd => tenses.perfect_2nd = Some(value),
    }
}

pub fn scrap_verb_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Verb")?)
        .next()
        .with_context(|| "cannot find common noun header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}

fn parsed_words_to_inflection(words: &[ParsedWord]) -> VerbInflectionThemes {
    let mut infl = VerbInflectionThemes::default();

    for word in words {
        fill_themes(word, &mut infl);
    }

    infl
}

fn fill_themes(word: &ParsedWord, themes: &mut VerbInflectionThemes) {
    if themes.thematic.is_none() {
        themes.thematic = Some(Default::default());
    }
    let thematic = themes.thematic.as_mut().unwrap();
    fill_contractions(word, thematic);
}

fn fill_contractions(word: &ParsedWord, contractions: &mut VerbInflectionContractions) {
    if word
        .parsing
        .contains(&ParsingComp::Contraction(Contraction::Uncontracted))
    {
        if contractions.uncontracted.is_none() {
            contractions.uncontracted = Some(Default::default());
        }
        let uncontracted = contractions.uncontracted.as_mut().unwrap();

        fill_moods(word, uncontracted);
    } else {
        if contractions.contracted.is_none() {
            contractions.contracted = Some(Default::default());
        }
        let contraction = contractions.contracted.as_mut().unwrap();

        fill_moods(word, contraction);
    }
}

fn fill_moods(word: &ParsedWord, moods: &mut VerbInflectionMoods) {
    if word.parsing.contains(&ParsingComp::Mood(Mood::Indicative)) {
        if moods.indicative.is_none() {
            moods.indicative = Some(Default::default());
        }
        let indicative = moods.indicative.as_mut().unwrap();

        fill_voices(word, indicative);
    }
    if word.parsing.contains(&ParsingComp::Mood(Mood::Subjunctive)) {
        if moods.subjunctive.is_none() {
            moods.subjunctive = Some(Default::default());
        }
        let subjunctive = moods.subjunctive.as_mut().unwrap();

        fill_voices(word, subjunctive);
    }
    if word.parsing.contains(&ParsingComp::Mood(Mood::Optative)) {
        if moods.optative.is_none() {
            moods.optative = Some(Default::default());
        }
        let optative = moods.optative.as_mut().unwrap();

        fill_voices(word, optative);
    }
    if word.parsing.contains(&ParsingComp::Mood(Mood::Imperative)) {
        if moods.imperative.is_none() {
            moods.imperative = Some(Default::default());
        }
        let imperative = moods.imperative.as_mut().unwrap();

        fill_voices(word, imperative);
    }
    if word.parsing.contains(&ParsingComp::Mood(Mood::Infinitive)) {
        if moods.infinitive.is_none() {
            moods.infinitive = Some(Default::default());
        }
        let infinitive = moods.infinitive.as_mut().unwrap();

        fill_forms(word, infinitive);
    }
    if word.parsing.contains(&ParsingComp::Mood(Mood::Participle)) {
        if moods.participle.is_none() {
            moods.participle = Some(Default::default());
        }
        let participle = moods.participle.as_mut().unwrap();

        noun::fill_genders(word, participle);
    }
}

fn fill_voices(word: &ParsedWord, voices: &mut VerbInflectionVoices) {
    if word.parsing.contains(&ParsingComp::Voice(Voice::Active)) {
        if voices.active.is_none() {
            voices.active = Some(Default::default());
        }
        let active = voices.active.as_mut().unwrap();
        fill_numbers(word, active);
    }
    if word.parsing.contains(&ParsingComp::Voice(Voice::Middle)) {
        if voices.middle.is_none() {
            voices.middle = Some(Default::default());
        }
        let middle = voices.middle.as_mut().unwrap();
        fill_numbers(word, middle);
    }
    if word.parsing.contains(&ParsingComp::Voice(Voice::Passive)) {
        if voices.passive.is_none() {
            voices.passive = Some(Default::default());
        }
        let passive = voices.passive.as_mut().unwrap();
        fill_numbers(word, passive);
    }
}

fn fill_numbers(word: &ParsedWord, numbers: &mut VerbInflectionNumbers) {
    if word
        .parsing
        .contains(&ParsingComp::Number(Number::Singular))
    {
        if numbers.singular.is_none() {
            numbers.singular = Some(Default::default());
        }
        let singular = numbers.singular.as_mut().unwrap();
        fill_persons(word, singular);
    }
    if word.parsing.contains(&ParsingComp::Number(Number::Plural)) {
        if numbers.plural.is_none() {
            numbers.plural = Some(Default::default());
        }
        let plural = numbers.plural.as_mut().unwrap();
        fill_persons(word, plural);
    }
    if word.parsing.contains(&ParsingComp::Number(Number::Dual)) {
        if numbers.dual.is_none() {
            numbers.dual = Some(Default::default());
        }
        let dual = numbers.dual.as_mut().unwrap();
        fill_persons(word, dual);
    }
}

fn fill_persons(word: &ParsedWord, persons: &mut VerbInflectionPersons) {
    if word.parsing.contains(&ParsingComp::Person(Person::First)) {
        if persons.first.is_none() {
            persons.first = Some(Default::default());
        }
        let first = persons.first.as_mut().unwrap();
        fill_forms(word, first);
    }
    if word.parsing.contains(&ParsingComp::Person(Person::Second)) {
        if persons.second.is_none() {
            persons.second = Some(Default::default());
        }
        let second = persons.second.as_mut().unwrap();
        fill_forms(word, second);
    }
    if word.parsing.contains(&ParsingComp::Person(Person::Third)) {
        if persons.third.is_none() {
            persons.third = Some(Default::default());
        }
        let third = persons.third.as_mut().unwrap();
        fill_forms(word, third);
    }
}

fn fill_forms(word: &ParsedWord, forms: &mut Vec<VerbInflectionForm>) {
    for part in word.text.split('\n') {
        if part.ends_with(')') {
            let end_group = part.chars().skip_while(|c| *c != '(').collect::<String>();
            let form_a = part.chars().take_while(|c| *c != '(').collect::<String>();
            let form_b = format!("{}{}", form_a, end_group.replace(['(', ')'], ""));

            forms.push(VerbInflectionForm {
                contracted: Some(form_a),
                ..Default::default()
            });
            forms.push(VerbInflectionForm {
                contracted: Some(form_b),
                ..Default::default()
            });
        } else {
            forms.push(VerbInflectionForm {
                contracted: Some(part.into()),
                ..Default::default()
            })
        }
    }
}
