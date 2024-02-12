use std::{borrow::Cow, collections::HashMap};

use anyhow::Context;
use tracing::debug;
use url::Url;

use crate::{
    error::SafeError,
    grammar::{Case, Declension, Gender, Noun, Number, PartOfSpeech},
    utils::str::decode_html::DecodeHtml,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ParsingOption {
    word: String,
    uncontracted: String,
    parsing: String,
    opt_index: i32,
    inflection_lemma: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct WordDetails {
    pub lemma: Vec<String>,
    pub group: String,
    pub translation: String,
    pub description: String,
    pub inflection_lemma: String,
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

async fn extract_details(
    word: &str,
    opt: i32,
    inflection_lemma: &str,
) -> Result<WordDetails, SafeError> {
    let url = &get_search_url(word, &Some(opt))?;
    debug!("Fetching {}", url);
    let res = reqwest::get(url).await?.text().await?;

    let dom = tl::parse(res.as_str(), tl::ParserOptions::default())?;
    let parser = dom.parser();

    let title_str = dom
        .query_selector("h2[lang='el']")
        .unwrap().next()
        .unwrap()
        .get(parser)
        .unwrap()
        .inner_text(parser);
    let infos = title_str.trim().split(',').collect::<Vec<&str>>();

    let translation_bytes = dom
        .query_selector("input[name='user-definition-basic']")
        .unwrap().next()
        .unwrap()
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap()
        .attributes()
        .get("value")
        .unwrap()
        .unwrap();
    let translation = std::str::from_utf8(translation_bytes.as_bytes())?;

    let desc = dom
        .query_selector("textarea[name='user-definition-long']")
        .unwrap().next()
        .unwrap()
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap()
        .inner_text(parser);

    return Ok(WordDetails {
        lemma: infos.first()
            .unwrap()
            .split('/')
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>(),
        group: infos.get(1).unwrap().replace('-', "").trim().to_string(),
        translation: translation.to_string().decode_html(),
        description: desc.to_string().decode_html(),
        inflection_lemma: inflection_lemma.to_string(),
    });
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
    let mood = match parsing_comps.first() {
        _ => None,
    };
    let number = match parsing_comps.get(2) {
        Some(&"sg") => Some(Number::Singular),
        _ => None,
    };
    let part_of_speech = PartOfSpeech::Noun(Noun::Common);
    let person = match parsing_comps.first() {
        _ => None,
    };
    let tense = match parsing_comps.first() {
        _ => None,
    };
    let theme = match parsing_comps.first() {
        _ => None,
    };
    let voice = match parsing_comps.first() {
        _ => None,
    };

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
    let url = &get_search_url(word, &None)?;
    debug!("Fetching {}", url);
    let res = reqwest::get(url).await?.text().await?;

    let dom = tl::parse(res.as_str(), tl::ParserOptions::default())?;
    let parser = dom.parser();

    let table_html = dom
        .query_selector("#content")
        .unwrap().next()
        .unwrap()
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap()
        .query_selector(parser, "table")
        .context("Could not find table")?.next()
        .unwrap()
        .get(parser)
        .unwrap()
        .inner_html(parser);

    let table_dom = tl::parse(&table_html, tl::ParserOptions::default())?;
    let table_parser = table_dom.parser();

    let mut parsing_options = Vec::<ParsingOption>::new();

    for (i, tr) in table_dom.query_selector("tr").unwrap().enumerate() {
        let tr_html = tr
            .get(table_parser)
            .unwrap()
            .as_tag()
            .unwrap()
            .inner_html(table_parser);
        let tr_dom = tl::parse(&tr_html, tl::ParserOptions::default())?;
        let tr_parser = tr_dom.parser();

        let tds_txt = tr_dom
            .query_selector("td")
            .unwrap()
            .map(|x| x.get(tr_parser).unwrap().inner_text(tr_parser))
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

fn get_search_url(word: &str, opt: &Option<i32>) -> Result<String, SafeError> {
    let base_url = "https://lexicon.katabiblon.com/index.php";

    let mut url = Url::parse(base_url)?;
    url.query_pairs_mut()
        .append_pair("search", word)
        .append_pair("opt", &opt.unwrap_or(1).to_string());

    Ok(url.to_string())
}
