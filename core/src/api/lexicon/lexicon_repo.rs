use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    Collection, IndexModel,
};
use nameof::name_of;
use serde::Serialize;

use crate::{
    error::{MapErrSafe, SafeError},
    persistence::get_db,
    utils::str::camel_case::CamelCase,
};

use super::lexicon_model::{LexiconEntry, LexiconFilter};

pub struct LexiconRepo;

impl LexiconRepo {
    pub const COLLECTION_NAME: &'static str = "lexicon";

    pub async fn find_one(&self, filter: LexiconFilter) -> Result<Option<LexiconEntry>, SafeError> {
        get_collection()
            .await?
            .find_one(filter, None)
            .await
            .map_err_safe()
    }
}

async fn get_collection() -> Result<Collection<LexiconEntry>, SafeError> {
    Ok(get_db()
        .await?
        .collection::<LexiconEntry>(LexiconRepo::COLLECTION_NAME))
}

pub async fn configure() -> Result<(), SafeError> {
    let options = IndexOptions::builder().unique(true).build();
    let unique_key_lemma = IndexModel::builder()
        .keys(doc! {"lemma": 1})
        .options(options)
        .build();

    get_collection()
        .await?
        .create_index(unique_key_lemma, None)
        .await
        .expect("error creating index!");

    Ok(())
}

impl From<LexiconFilter> for Option<Document> {
    fn from(value: LexiconFilter) -> Self {
        let mut doc = Document::new();

        if let Some(lemma) = value.lemma {
            doc.insert(name_of!(lemma).camel_case(), lemma);
        }

        Some(doc)
    }
}
