use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    Collection, IndexModel,
};
use nameof::name_of;

use crate::{
    error::{MapErrSafe, SafeError},
    grammar::Verse,
    persistence::get_db,
    utils::str::camel_case::CamelCase,
};

use super::verse_model::VerseFilter;

pub struct VerseRepo;

impl VerseRepo {
    pub const COLLECTION_NAME: &'static str = "verses";

    pub async fn find_one(&self, filter: VerseFilter) -> Result<Option<Verse>, SafeError> {
        get_collection()
            .await?
            .find_one(filter, None)
            .await
            .map_err_safe()
    }
}

async fn get_collection() -> Result<Collection<Verse>, SafeError> {
    Ok(get_db()
        .await?
        .collection::<Verse>(VerseRepo::COLLECTION_NAME))
}

pub async fn configure() -> Result<(), SafeError> {
    let options = IndexOptions::builder().unique(true).build();
    let unique_key = IndexModel::builder()
        .keys(doc! {"collection": 1, "book": 1, "chapterNumber": 1, "verseNumber": 1})
        .options(options)
        .build();

    get_collection()
        .await?
        .create_index(unique_key, None)
        .await
        .expect("error creating index!");

    Ok(())
}

impl From<VerseFilter> for Option<Document> {
    fn from(value: VerseFilter) -> Self {
        let mut doc = Document::new();

        if let Some(collection) = value.collection {
            doc.insert(name_of!(collection).camel_case(), collection);
        }
        if let Some(book) = value.book {
            doc.insert(name_of!(book).camel_case(), book);
        }
        if let Some(chapter_number) = value.chapter_number {
            doc.insert(name_of!(chapter_number).camel_case(), chapter_number);
        }
        if let Some(verse_number) = value.verse_number {
            doc.insert(name_of!(verse_number).camel_case(), verse_number);
        }

        Some(doc)
    }
}
