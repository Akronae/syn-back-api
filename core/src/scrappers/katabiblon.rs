pub mod details;
pub mod inflections;
pub mod parser;

use anyhow::Context;
use tracing::{debug, info};

use crate::{
    api::{
        lexicon::{lexicon_model::LexiconFilter, lexicon_repo::LexiconRepo},
        verse::{verse_model::VerseFilter, verse_repo::VerseRepo},
    },
    error::SafeError,
    persistence,
};

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    let _lexicon_repo = LexiconRepo {};

    let mut first_verse = VerseRepo::find_one(&VerseFilter {
        collection: Some("new_testament".to_string()),
        book: Some("matthew".to_string()),
        chapter_number: Some(1),
        verse_number: Some(1),
    })
    .await?
    .context("no verse")?;

    let mut has_changes = false;

    for word in &mut first_verse.words {
        async fn is_already_in_lexicon(lemma: &str) -> Result<bool, SafeError> {
            Ok(LexiconRepo::find_one(LexiconFilter {
                lemma: Some(lemma.to_string()),
            })
            .await?
            .is_some())
        }

        if is_already_in_lexicon(&word.text).await? {
            debug!("{} already in lexicon", word.text);
            continue;
        }

        let parsed = parser::parse_word(&word.text, &word.declension).await?;

        if parsed.lemma != word.text {
            word.text = parsed.lemma.to_owned();
            has_changes = true;
        }

        if is_already_in_lexicon(&parsed.lemma).await? {
            debug!("{} already in lexicon", parsed.lemma);
            continue;
        }

        persistence::get_db()
            .await?
            .collection(LexiconRepo::COLLECTION_NAME)
            .insert_many(Vec::from([parsed.clone()]), None)
            .await?;

        info!(
            "{:?} imported into {}",
            parsed,
            LexiconRepo::COLLECTION_NAME
        );
    }

    if has_changes {
        debug!(
            "updating verse {} {} {} {}",
            first_verse.collection,
            first_verse.book,
            first_verse.chapter_number,
            first_verse.verse_number
        );
        VerseRepo::update_one(&first_verse).await?;
    }

    Ok(())
}
