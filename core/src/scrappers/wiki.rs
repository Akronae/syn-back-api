use anyhow::Context;
use tracing::{debug, info};

use crate::{
    api::{
        lexicon::{
            lexicon_model::{LexiconEntry, LexiconFilter, LexiconFilterInflection},
            lexicon_repo::LexiconRepo,
        },
        verse::{verse_model::VerseFilter, verse_repo::VerseRepo},
    },
    error::SafeError,
    grammar::{Declension},
};

mod definition;
mod details;
mod noun;
mod page;
mod parser;
mod table;

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    let mut verse = VerseRepo::find_one(&VerseFilter {
        collection: Some("new_testament".to_string()),
        book: Some("matthew".to_string()),
        chapter_number: Some(1),
        verse_number: Some(1),
    })
    .await?
    .context("no verse")?;

    let mut has_changes = false;

    for word in &mut verse.words {
        async fn find_in_lexicon(
            word: &str,
            declension: &Declension,
        ) -> Result<Option<LexiconEntry>, SafeError> {
            if declension.indeclinable.unwrap_or(false) {
                return LexiconRepo::find_one(LexiconFilter {
                    lemma: Some(word.to_owned()),
                    ..Default::default()
                })
                .await;
            }

            LexiconRepo::find_one(LexiconFilter {
                inflection: Some(LexiconFilterInflection {
                    declension: declension.to_owned(),
                    word: word.to_string(),
                }),
                ..Default::default()
            })
            .await
        }

        let parsed;
        if let Some(already) = find_in_lexicon(&word.text, &word.declension).await? {
            debug!("{} already in lexicon", word.text);
            parsed = already;
        } else {
            debug!("{} not in lexicon, fetching", word.text);
            parsed = parser::parse_word(word.text.clone().into(), &word.declension).await?;
        }

        if let Some(parsed_inflection) = parsed.inflections.first() {
            let inflected = parsed_inflection.find_inflection(&word.declension);

            if let Some(inflected) = inflected {
                if inflected != word.text {
                    word.text = inflected.to_owned();
                    has_changes = true;
                    debug!("{} changing to {}", word.text, inflected);
                } else {
                    debug!("{} already inflected", word.text);
                }
            } else {
                debug!(
                    "{} not found with inflection {:?} in {:?}",
                    word.text, word.declension, parsed.inflections
                );
            }
        } else if parsed.lemma != word.text {
            word.text = parsed.lemma.to_owned();
            has_changes = true;
            debug!(
                "{} has no inflection, so changing to lemma {}",
                word.text, parsed.lemma
            );
        }

        if find_in_lexicon(&word.text, &word.declension)
            .await?
            .is_none()
        {
            LexiconRepo::insert_one(parsed.clone()).await?;

            info!(
                "{:?} imported into {}",
                parsed,
                LexiconRepo::COLLECTION_NAME
            );
        }
    }

    if has_changes {
        debug!(
            "updating verse {} {} {} {}",
            verse.collection, verse.book, verse.chapter_number, verse.verse_number
        );
        VerseRepo::update_one(&verse).await?;
    }

    Ok(())
}
