pub mod details;
pub mod inflections;
pub mod parser;

use anyhow::Context;
use tracing::info;

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
    let verse_repo = VerseRepo {};
    let lexicon_repo = LexiconRepo {};

    let first_verse = verse_repo
        .find_one(VerseFilter {
            collection: Some("new_testament".to_string()),
            book: Some("matthew".to_string()),
            chapter_number: Some(1),
            verse_number: Some(1),
        })
        .await?
        .context("no verse")?;

    for mut word in first_verse.words {
        let parsed = parser::parse_word(&word.text, &word.declension).await?;
        let parsed_lemma = parsed.lemma.first().unwrap().to_string();

        if parsed_lemma != word.text {
            word.text = parsed_lemma.to_owned();
        }

        let lexicon_entry = lexicon_repo
            .find_one(LexiconFilter {
                lemma: Some(parsed_lemma.to_owned()),
            })
            .await?;

        if lexicon_entry.is_some() {
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

    Ok(())
}
