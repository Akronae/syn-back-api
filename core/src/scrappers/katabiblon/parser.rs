use anyhow::Context;
use mongodb::bson::datetime;
use paperclip::actix::web::delete;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    error::Error,
    fmt,
    str::FromStr,
};
use strum::Display;
use tracing::{field::debug, *};
use url::Url;

use crate::{
    error::SafeError,
    grammar::{
        Case, Declension, DeclensionType, Gender, Language, Noun, Number, PartOfSpeech, Verse, Word,
    },
    scrappers::{
        abarim::declension,
        katabiblon::{details::search_word_details, inflections::extract_inflections},
    },
    texts::{Book, Collection},
    utils::str::{capitalize::Capitalize, decode_html::DecodeHtml},
};

#[allow(dead_code)]
pub async fn parse_word(greek_word: &str, declension: &Declension) -> Result<(), SafeError> {
    info!("Parsing word {}", greek_word);

    let details = search_word_details(greek_word, declension).await?;
    dbg!(details.clone());

    let inflection = extract_inflections(&details.inflection_lemma).await?;
    dbg!(inflection.clone());

    Ok(())
}
