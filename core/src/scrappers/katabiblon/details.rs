use std::{borrow::Cow, collections::HashMap};

use anyhow::Context;
use serde::Serialize;
use tracing_subscriber::fmt::format;

use crate::{
    error::SafeError,
    grammar::{Case, Declension, DeclensionType, Gender, Noun, Number, PartOfSpeech},
    utils::{scrapper::select::select, str::decode_html::DecodeHtml},
};

use super::page;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ParsingOption {
    word: String,
    uncontracted: String,
    parsing: String,
    opt_index: i32,
    inflection_lemma: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct WordDetails {
    pub lemma: String,
    pub translation: String,
    pub description: String,
    pub declension_type: DeclensionType,
}

pub async fn search_word_details(
    greek_word: &str,
    declension: &Declension,
) -> Result<WordDetails, SafeError> {
    let options = extract_options(greek_word).await?;
    let mut matching_scores = HashMap::<ParsingOption, i32>::new();

    for option in options {
        let score = compute_option_matching(&option, declension);
        matching_scores.insert(option, score);
    }

    let matching_opt = matching_scores
        .iter()
        .max_by_key(|(_, &score)| score)
        .map(|(opt, _)| opt)
        .unwrap();

    extract_details(
        greek_word,
        matching_opt.opt_index,
        &matching_opt.inflection_lemma,
    )
    .await
}

async fn extract_details(word: &str, opt: i32, lemma: &str) -> Result<WordDetails, SafeError> {
    let page = page::scrap(word, &Some(opt)).await?;

    let translation = page
        .select(&select("input[name='user-definition-basic']")?)
        .next()
        .unwrap()
        .attr("value")
        .unwrap();

    let desc = page
        .select(&select("textarea[name='user-definition-long']")?)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let decl_type_str = page
        .select(&select("#content p[style='margin-top:0'")?)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let declension_type = match decl_type_str.to_lowercase() {
        s if s.contains("1st decl") => DeclensionType::First,
        s if s.contains("2nd decl") => DeclensionType::Second,
        s if s.contains("3rd decl") => DeclensionType::Third,
        _ => {
            return Err(
                format!("could not match declension type for {word}: {decl_type_str}").into(),
            )
        }
    };

    Ok(WordDetails {
        lemma: lemma.to_string(),
        translation: translation.decode_html(),
        description: desc.decode_html(),
        declension_type,
    })
}

fn compute_option_matching(option: &ParsingOption, declension: &Declension) -> i32 {
    let mut score = 0;

    let parsing_comps = option.parsing.split_whitespace().collect::<Vec<&str>>();

    let case = match parsing_comps.get(1) {
        Some(&"nom") => Some(Case::Nominative),
        _ => None,
    };
    let gender = match parsing_comps.first() {
        Some(&"(fem)") => Some(Gender::Feminine),
        _ => None,
    };
    let mood = None;
    let number = match parsing_comps.get(2) {
        Some(&"sg") => Some(Number::Singular),
        _ => None,
    };
    let part_of_speech = PartOfSpeech::Noun(Noun::Common);
    let person = None;
    let tense = None;
    let theme = None;
    let voice = None;

    if case == declension.case {
        score += 1;
    }
    if gender == declension.gender {
        score += 1;
    }
    if mood == declension.mood {
        score += 1;
    }
    if number == declension.number {
        score += 1;
    }
    if part_of_speech == declension.part_of_speech {
        score += 1;
    }
    if person == declension.person {
        score += 1;
    }
    if tense == declension.tense {
        score += 1;
    }
    if theme == declension.theme {
        score += 1;
    }
    if voice == declension.voice {
        score += 1;
    }

    score
}

async fn extract_options(word: &str) -> Result<Vec<ParsingOption>, SafeError> {
    let page = page::scrap(word, &None).await?;

    let table = page
        .select(&select("#content table")?)
        .next()
        .with_context(|| format!("could not get first table"))?;

    let mut parsing_options = Vec::<ParsingOption>::new();

    for (i, tr) in table.select(&select("tr")?).enumerate() {
        let tds_txt = tr
            .select(&select("td")?)
            .map(|x| x.text().collect())
            .collect::<Vec<Cow<str>>>();

        if tds_txt.is_empty() {
            continue;
        }

        parsing_options.push(ParsingOption {
            word: tds_txt.get(1).unwrap().to_string(),
            uncontracted: tds_txt.get(3).unwrap().to_string(),
            parsing: tds_txt.last().unwrap().to_string(),
            inflection_lemma: tds_txt.get(2).unwrap().to_string(),
            opt_index: i as i32,
        });
    }

    Ok(parsing_options)
}
