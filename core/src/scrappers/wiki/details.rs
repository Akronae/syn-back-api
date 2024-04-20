use anyhow::Context;
use async_recursion::async_recursion;
use reqwest::Method;
use serde::Deserialize;

use tracing::{debug, info};

use crate::{
    api::lexicon::lexicon_model::LexiconEntryDefinition,
    borrow::Cow,
    error::SafeError,
    grammar::{Declension, Mood, Noun, Number, PartOfSpeech, Person},
    request::request,
    scrappers::wiki::{noun, page},
    utils::{
        scrapper::select::select,
        str::{
            closest::{closest_with_score, similarity_score},
            skip_last::SkipLast,
        },
    },
};

use super::{article, conjunction, errors::ParseWordError, participle, preposition, pronoun, verb};

#[allow(dead_code)]
pub enum SearchMode {
    Query,
    Opensearch,
}

#[derive(Debug, Clone)]
pub struct WordDetails {
    pub lemma: Cow<str>,
}

#[async_recursion(?Send)]
pub async fn search_word_details(
    word: Cow<str>,
    declension: &Declension,
    mode: &SearchMode,
) -> Result<WordDetails, ParseWordError> {
    let pos = &declension.part_of_speech;
    let lemmas = match mode {
        SearchMode::Query => query(word.clone(), declension).await?,
        SearchMode::Opensearch => opensearch(word.clone()).await?,
    };

    let lemmas = closest_with_score(word.clone(), &lemmas)
        .into_iter()
        .filter(|x| x.score > 0.0);

    for lemma in lemmas {
        let lemma = validate_page(lemma.value.clone(), declension, word.clone()).await?;

        if let Some(lemma) = lemma {
            return Ok(WordDetails { lemma });
        }
    }

    if matches!(pos, PartOfSpeech::Verb)
        && matches!(declension.person, Some(Person::Third))
        && matches!(declension.number, Some(Number::Singular))
        && word.ends_with('ν')
    {
        let word = word.chars().skip_last(1).collect::<String>().into();
        info!("no match found. trying to remove optional (ν) with {word}");
        return search_word_details(word, declension, mode).await;
    } else {
        return Err(ParseWordError::NotFound(word.into()));
    }
}

#[async_recursion(?Send)]
async fn validate_page(
    lemma: Cow<str>,
    decl: &Declension,
    query: Cow<str>,
) -> Result<Option<Cow<str>>, SafeError> {
    let pos = decl.part_of_speech;
    let doc = page::scrap(lemma.as_ref()).await?;

    if matches!(pos, PartOfSpeech::Noun(Noun::Common))
        && doc.select(&select("#Noun")?).next().is_none()
    {
        return Ok(None);
    }
    if matches!(pos, PartOfSpeech::Noun(Noun::Proper))
        && doc.select(&select("#Proper_noun")?).next().is_none()
    {
        return Ok(None);
    }

    let def = match pos {
        PartOfSpeech::Noun(noun) => noun::scrap_noun_defs(&doc, &noun)?,
        PartOfSpeech::Verb => match decl.mood {
            Some(Mood::Participle) => participle::scrap_participle_defs(&doc)?,
            _ => verb::scrap_verb_defs(&doc)?,
        },
        PartOfSpeech::Article(_) => article::scrap_article_defs(&doc)?,
        PartOfSpeech::Pronoun(_) => pronoun::scrap_pronoun_defs(&doc)?,
        PartOfSpeech::Conjunction => conjunction::scrap_conjunction_defs(&doc)?,
        PartOfSpeech::Preposition => preposition::scrap_preposition_defs(&doc)?,
        _ => panic!("unsupported part of speech: {:?}", pos),
    };

    if let Some(LexiconEntryDefinition::FormOf(formof)) = def.first() {
        if similarity_score(query.clone(), formof.lemma.clone().into())
            >= similarity_score(query.clone(), lemma.clone())
        {
            return validate_page(formof.lemma.clone().into(), decl, query).await;
        }
    }

    let has_infl_table = doc.select(&select(".NavFrame.grc-decl")?).next().is_some()
        || doc
            .select(&select(".NavFrame .grce-conj")?)
            .next()
            .is_some();

    if !has_infl_table {
        match pos {
            PartOfSpeech::Conjunction | PartOfSpeech::Preposition => return Ok(Some(lemma)),
            _ => (),
        }

        if let Some(LexiconEntryDefinition::FormOf(formof)) = def.first() {
            debug!("found form of {}", formof.lemma);
            return validate_page(formof.lemma.clone().into(), decl, query.clone()).await;
        }
    }

    Ok(Some(lemma))
}

fn build_query_urls(word: Cow<str>, decl: &Declension) -> Vec<Cow<str>> {
    let pos = &decl.part_of_speech;
    let categories = match pos {
        PartOfSpeech::Noun(noun) => match noun {
            Noun::Common => vec!["Ancient_Greek_nouns", "Ancient_Greek_noun_forms"],
            Noun::Proper => vec![
                "Ancient_Greek_proper_nouns",
                "Ancient_Greek_proper_noun_forms",
            ],
        },
        PartOfSpeech::Verb => match decl.mood {
            Some(Mood::Participle) => vec!["Ancient_Greek_participles", "Ancient_Greek_verb_forms"],
            _ => vec!["Ancient_Greek_verbs", "Ancient_Greek_verb_forms"],
        },
        PartOfSpeech::Article(_) => vec!["Ancient_Greek_articles", "Ancient_Greek_article_forms"],
        PartOfSpeech::Conjunction => vec!["Ancient_Greek_conjunctions"],
        PartOfSpeech::Pronoun(_) => vec!["Ancient_Greek_pronouns", "Ancient_Greek_pronoun_forms"],
        PartOfSpeech::Preposition => vec!["Ancient_Greek_prepositions"],
        _ => panic!("unsupported part of speech: {:?}", pos),
    };

    return categories.iter().map(|x| Cow::<str>::from(format!("https://en.wiktionary.org/w/api.php?format=json&action=query&list=search&srsearch={word}+incategory:{x}"))).collect();
}

#[derive(Debug, Deserialize)]
struct ListResponseQuerySearch {
    pub title: Cow<str>,
}
#[derive(Debug, Deserialize)]
struct ListResponseQuery {
    pub search: Vec<ListResponseQuerySearch>,
}
#[derive(Debug, Deserialize)]
struct ListResponse {
    pub query: ListResponseQuery,
}

async fn query(word: Cow<str>, decl: &Declension) -> Result<Vec<Cow<str>>, SafeError> {
    let urls = build_query_urls(word.clone(), decl);
    let mut lemmas = Vec::new();

    for url in urls {
        debug!("fetching {}", url.as_ref());
        let response = request()
            .with_method(Method::GET)
            .with_url(url.to_string())
            .with_cache(true)
            .text()
            .await?;
        let response: ListResponse = serde_json::from_str::<ListResponse>(&response)?;

        let lems = response
            .query
            .search
            .iter()
            .map(|x| x.title.clone())
            .collect::<Vec<_>>();
        lemmas.extend(lems);
    }

    Ok(lemmas)
}

fn build_opensearch_url(word: Cow<str>) -> Cow<str> {
    Cow::from(format!(
        "https://en.wiktionary.org/w/api.php?format=json&action=opensearch&namespace=0&limit=10&search={word}"
    ))
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum VecOrSingle<T> {
    Vec(Vec<T>),
    Single(T),
}

#[derive(Debug, Deserialize)]
struct OpenSearchResponse(Vec<VecOrSingle<Cow<str>>>);

async fn opensearch(word: Cow<str>) -> Result<Vec<Cow<str>>, SafeError> {
    let url = build_opensearch_url(word);
    debug!("fetching {}", url.as_ref());
    let response = request()
        .with_method(Method::GET)
        .with_url(url.to_string())
        .with_cache(true)
        .text()
        .await?;
    let response = serde_json::from_str::<OpenSearchResponse>(&response)?;

    let elem_1 = response.0.get(1).context("no elem at index 1")?;
    if let VecOrSingle::Vec(vec) = elem_1 {
        Ok(vec.clone())
    } else {
        Err("expected to have a vector at index 1".into())
    }
}
