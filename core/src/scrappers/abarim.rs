use crate::{error::SafeError, persistence, texts::Book};

pub mod declension;
pub mod parser;

#[allow(dead_code)]
async fn import(collection: &str) -> Result<(), SafeError> {
    let parsed = parser::parse_chapter(1, Book::Matthew).await?;

    persistence::get_db()
        .await?
        .collection(collection)
        .insert_many(parsed.verses, None)
        .await?;

    Ok(())
}
