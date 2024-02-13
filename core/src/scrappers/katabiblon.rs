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
    grammar::Word,
    persistence,
};

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    let lexicon_repo = LexiconRepo {};

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
        let parsed = parser::parse_word(&word.text, &word.declension).await?;
        let parsed_lemma = parsed.lemma.first().unwrap().to_string();

        if parsed_lemma != word.text {
            word.text = parsed_lemma.to_owned();
            has_changes = true;
        }

        let lexicon_entry = lexicon_repo
            .find_one(LexiconFilter {
                lemma: Some(parsed_lemma.to_owned()),
            })
            .await?;

        if lexicon_entry.is_some() {
            debug!("{} already in lexicon", parsed_lemma);
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
