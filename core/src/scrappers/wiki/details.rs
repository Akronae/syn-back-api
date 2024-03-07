use std::cmp::Ordering;

use anyhow::Context;
use async_recursion::async_recursion;
use serde::Deserialize;
use strsim::normalized_damerau_levenshtein;
use tracing::debug;

use crate::{
    api::lexicon::lexicon_model::LexiconEntryDefinition,
    borrow::Cow,
    error::SafeError,
    grammar::{Noun, PartOfSpeech},
    scrappers::wiki::{noun, page},
    utils::scrapper::select::select,
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
    pos: &PartOfSpeech,
) -> Result<WordDetails, SafeError> {
    let url = build_list_url(word.clone(), pos);
    debug!("fetching {}", url.as_ref());
    let response = reqwest::get(url.as_ref()).await?.text().await?;
    let response: ListResponse = serde_json::from_str::<ListResponse>(&response)?;

    let mut lemmas = response.query.search.iter().collect::<Vec<_>>();

    lemmas.sort_by(|a, b| {
        if normalized_damerau_levenshtein(&a.title, &word)
            < normalized_damerau_levenshtein(&b.title, &word)
        {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let lemma = lemmas
        .first()
        .with_context(|| format!("cannot find entry for {word}"))?;

    let lemma = validate_page(lemma.title.clone(), pos).await?;

    Ok(WordDetails { lemma })
}

#[async_recursion(?Send)]
async fn validate_page(lemma: Cow<str>, pos: &PartOfSpeech) -> Result<Cow<str>, SafeError> {
    let doc = page::scrap(lemma.as_ref()).await?;
    let has_infl_table = doc.select(&select(".NavFrame.grc-decl")?).next().is_some();

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
        _ => todo!(),
    };
    format!("https://en.wiktionary.org/w/api.php?format=json&action=query&list=search&srsearch={word}+incategory:{category}").into()
}
