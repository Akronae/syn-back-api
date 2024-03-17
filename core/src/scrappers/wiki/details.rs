use std::cmp::Ordering;

use async_recursion::async_recursion;
use serde::Deserialize;
use strsim::normalized_damerau_levenshtein;
use tracing::{debug, info};
use unidecode::unidecode;

use crate::{
    api::lexicon::lexicon_model::LexiconEntryDefinition,
    borrow::Cow,
    error::SafeError,
    grammar::{Declension, Noun, Number, PartOfSpeech, Person},
    scrappers::wiki::{noun, page},
    utils::{scrapper::select::select, str::skip_last::SkipLast},
};

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

#[derive(Debug, Clone)]
pub struct WordDetails {
    pub lemma: Cow<str>,
}

#[async_recursion(?Send)]
pub async fn search_word_details(
    word: Cow<str>,
    declension: &Declension,
) -> Result<WordDetails, SafeError> {
    let pos = &declension.part_of_speech;
    let url = build_list_url(word.clone(), pos);
    debug!("fetching {}", url.as_ref());
    let response = reqwest::get(url.as_ref()).await?.text().await?;
    let response: ListResponse = serde_json::from_str::<ListResponse>(&response)?;

    let mut lemmas = response.query.search.iter().collect::<Vec<_>>();

    lemmas.sort_by(|a, b| {
        let a_dst = normalized_damerau_levenshtein(&a.title, &word);
        let b_dst = normalized_damerau_levenshtein(&b.title, &word);
        let a_no_dia_dst = normalized_damerau_levenshtein(&unidecode(&a.title), &unidecode(&word));
        let b_no_dia_dst = normalized_damerau_levenshtein(&unidecode(&b.title), &unidecode(&word));
        debug!(
            "comparing {} ({a_dst} + {a_no_dia_dst}) with {} ({b_dst} + {b_no_dia_dst}) to {word}",
            a.title, b.title
        );
        if a_dst + a_no_dia_dst < b_dst + b_no_dia_dst {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let lemma = lemmas.first();

    if let Some(lemma) = lemma {
        let lemma = validate_page(lemma.title.clone(), pos).await?;

        return Ok(WordDetails { lemma });
    }

    if matches!(pos, PartOfSpeech::Verb)
        && matches!(declension.person, Some(Person::Third))
        && matches!(declension.number, Some(Number::Singular))
        && word.ends_with('ν')
    {
        let word = word.chars().skip_last(1).collect::<String>().into();
        info!("no match found. trying to remove optional (ν) with {word}");
        return search_word_details(word, declension).await;
    } else {
        return Err(format!("cannot find entry for {word}").into());
    }
}

#[async_recursion(?Send)]
async fn validate_page(lemma: Cow<str>, pos: &PartOfSpeech) -> Result<Cow<str>, SafeError> {
    let doc = page::scrap(lemma.as_ref()).await?;
    let has_infl_table = doc.select(&select(".NavFrame.grc-decl")?).next().is_some()
        || doc.select(&select(".NavFrame .grc-conj")?).next().is_some();

    if !has_infl_table {
        let def;
        if let PartOfSpeech::Noun(noun) = pos {
            def = noun::scrap_noun_defs(&doc, noun)?;
        } else {
            todo!()
        }

        if let Some(LexiconEntryDefinition::FormOf(formof)) = def.first() {
            debug!("found form of {formof}");
            return validate_page(formof.clone().into(), pos).await;
        }
    }

    Ok(lemma)
}

fn build_list_url(word: Cow<str>, pos: &PartOfSpeech) -> Cow<str> {
    let category = match pos {
        PartOfSpeech::Noun(noun) => match noun {
            Noun::Common => "Ancient_Greek_nouns+Ancient_Greek_noun_forms",
            Noun::Proper => "Ancient_Greek_proper_nouns+Ancient_Greek_proper_noun_forms",
        },
        PartOfSpeech::Verb => "Ancient_Greek_verbs+Ancient_Greek_verb_forms",
        PartOfSpeech::Article(_) => "Ancient_Greek_articles+Ancient_Greek_article_forms",
        _ => todo!(),
    };
    format!("https://en.wiktionary.org/w/api.php?format=json&action=query&list=search&srsearch={word}+incategory:{category}").into()
}
