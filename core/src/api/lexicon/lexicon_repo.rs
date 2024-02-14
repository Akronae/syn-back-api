use std::borrow::Borrow;

use mongodb::{
    bson::{doc, Bson, Document},
    options::IndexOptions,
    Collection, IndexModel,
};
use nameof::name_of;
use tracing::debug;

use crate::{
    error::{MapErrSafe, SafeError},
    grammar::{Case, Declension, Gender, Number, PartOfSpeech},
    persistence::get_db,
    utils::str::camel_case::CamelCase,
};

use super::lexicon_model::{LexiconEntry, LexiconFilter};

pub struct LexiconRepo;

impl LexiconRepo {
    pub const COLLECTION_NAME: &'static str = "lexicon";

    pub async fn find_one(filter: LexiconFilter) -> Result<Option<LexiconEntry>, SafeError> {
        let res = get_collection()
            .await?
            .find_one(filter.clone(), None)
            .await
            .map_err_safe();

        res
    }

    pub async fn insert_many(entries: &[LexiconEntry]) -> Result<(), SafeError> {
        get_collection()
            .await?
            .insert_many(entries, None)
            .await
            .map_err_safe()?;

        Ok(())
    }

    pub async fn insert_one(entry: LexiconEntry) -> Result<(), SafeError> {
        LexiconRepo::insert_many(&[entry]).await
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
            doc.insert(
                name_of!(lemma).camel_case(),
                doc! {"$regex": lemma, "$options": "i"},
            );
        }

        if let Some(inflection) = value.inflection {
            let key = format!(
                "{}.contracted",
                inflection.declension.to_inflection_key().unwrap()
            );
            doc.insert(
                "inflections",
                doc! {"$elemMatch": doc! {key: doc! {"$regex": inflection.word, "$options": "i"}}},
            );
        }

        Some(doc)
    }
}

impl Declension {
    pub fn to_inflection_key(&self) -> Result<String, SafeError> {
        let mut s = Vec::<&str>::new();

        if let PartOfSpeech::Noun(_) = self.part_of_speech {
            s.push("noun");

            s.push(match self.gender {
                Some(Gender::Feminine) => "feminine",
                Some(Gender::Masculine) => "masculine",
                Some(Gender::Neuter) => "neuter",
                None => return Err(format!("part of speech required").into()),
            });

            s.push(match self.number {
                Some(Number::Singular) => "singular",
                Some(Number::Plural) => "plural",
                None => return Err(format!("number required").into()),
            });

            s.push(match self.case {
                Some(Case::Nominative) => "nominative",
                Some(Case::Genitive) => "genitive",
                Some(Case::Dative) => "dative",
                Some(Case::Accusative) => "accusative",
                Some(Case::Vocative) => "vocative",
                None => return Err(format!("case required").into()),
            });
        } else {
            return Err(format!("part of speech not supported {:?}", self.part_of_speech).into());
        }

        return Ok(s.join("."));
    }
}
