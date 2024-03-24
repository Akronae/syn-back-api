

use async_recursion::async_recursion;
use serde::Deserialize;

use tracing::{debug, info};

use crate::{
    api::lexicon::lexicon_model::LexiconEntryDefinition,
    borrow::Cow,
    error::SafeError,
    grammar::{Declension, Noun, Number, PartOfSpeech, Person},
    scrappers::wiki::{noun, page},
    utils::{
        scrapper::select::select,
        str::{closest::closest, skip_last::SkipLast},
    },
};

use super::{article, verb};

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
    let urls = build_list_urls(word.clone(), pos);

    let mut lemmas = Vec::new();
    for url in urls {
        debug!("fetching {}", url.as_ref());
        let response = reqwest::get(url.as_ref()).await?.text().await?;
        let response: ListResponse = serde_json::from_str::<ListResponse>(&response)?;

        let lems = response
            .query
            .search
            .iter()
            .map(|x| x.title.clone())
            .collect::<Vec<_>>();
        lemmas.extend(lems);
    }

    let lemma = closest(word.clone(), &lemmas);

    if let Some(lemma) = lemma {
        let lemma = validate_page(lemma, pos).await?;

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
        if matches!(pos, PartOfSpeech::Conjunction) {
            return Ok(lemma);
        }

        let def;
        if let PartOfSpeech::Noun(noun) = pos {
            def = noun::scrap_noun_defs(&doc, noun)?;
        } else if &PartOfSpeech::Verb == pos {
            def = verb::scrap_verb_defs(&doc)?;
        } else if let PartOfSpeech::Article(_) = pos {
            def = article::scrap_article_defs(&doc)?;
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

fn build_list_urls(word: Cow<str>, pos: &PartOfSpeech) -> Vec<Cow<str>> {
    let categories = match pos {
        PartOfSpeech::Noun(noun) => match noun {
            Noun::Common => vec!["Ancient_Greek_nouns", "Ancient_Greek_noun_forms"],
            Noun::Proper => vec![
                "Ancient_Greek_proper_nouns",
                "Ancient_Greek_proper_noun_forms",
            ],
        },
        PartOfSpeech::Verb => vec!["Ancient_Greek_verbs", "Ancient_Greek_verb_forms"],
        PartOfSpeech::Article(_) => vec!["Ancient_Greek_articles", "Ancient_Greek_article_forms"],
        PartOfSpeech::Conjunction => vec!["Ancient_Greek_conjunctions"],
        PartOfSpeech::Pronoun(_) => vec!["Ancient_Greek_pronouns", "Ancient_Greek_pronoun_forms"],
        _ => todo!(),
    };

    return categories.iter().map(|x| Cow::<str>::from(format!("https://en.wiktionary.org/w/api.php?format=json&action=query&list=search&srsearch={word}+incategory:{x}"))).collect();
}
