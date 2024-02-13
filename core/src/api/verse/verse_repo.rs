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

    pub async fn find_one(filter: &VerseFilter) -> Result<Option<Verse>, SafeError> {
        get_collection()
            .await?
            .find_one(Some(filter.into()), None)
            .await
            .map_err_safe()
    }

    pub async fn update_one(update: &Verse) -> Result<(), SafeError> {
        get_collection()
            .await?
            .replace_one((&VerseFilter::from(update)).into(), update, None)
            .await
            .map_err_safe()?;

        Ok(())
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
