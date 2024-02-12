use tracing::log::info;

use crate::{api::verse::verse_repo::VerseRepo, error::SafeError, persistence, texts::Book};

pub mod declension;
pub mod parser;

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    let parsed = parser::parse_chapter(1, Book::Matthew).await?;

    persistence::get_db()
        .await?
        .collection(VerseRepo::COLLECTION_NAME)
        .insert_many(parsed.verses.to_owned(), None)
        .await?;

    info!(
        "{} verses imported into {}",
        parsed.verses.len(),
        VerseRepo::COLLECTION_NAME
    );

    Ok(())
}
