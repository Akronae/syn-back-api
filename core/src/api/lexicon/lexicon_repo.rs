use anyhow::anyhow;
use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    Collection, IndexModel,
};
use nameof::name_of;

use crate::{
    borrow::Cow,
    error::{MapErrSafe, SafeError},
    grammar::{Declension, PartOfSpeech},
    persistence::get_db,
    utils::str::{camel_case::CamelCase, snake_case::SnakeCase},
};

use super::lexicon_model::{LexiconEntry, LexiconFilter};

pub struct LexiconRepo;

impl LexiconRepo {
    pub const COLLECTION_NAME: &'static str = "lexicon";

    pub async fn find_one(filter: LexiconFilter) -> Result<Option<LexiconEntry>, SafeError> {
        get_collection()
            .await?
            .find_one(filter.to_document()?, None)
            .await
            .map_err_safe()
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

fn fill_query(doc: &mut Document, stages: &Vec<Vec<String>>, i: usize, word: &str) {
    let stage = stages.get(i).unwrap();
    if i == stages.len() - 1 {
        doc.insert(
            stage.join("."),
            doc! {"$regex": word.to_string(), "$options": "i"},
        );
    } else {
        let mut subdoc = doc! {};
        fill_query(&mut subdoc, stages, i + 1, word);
        doc.insert(stage.join("."), doc! {"$elemMatch": subdoc});
    }
}

impl LexiconFilter {
    pub fn to_document(&self) -> Result<Document, SafeError> {
        let mut doc = Document::new();

        if let Some(lemma) = &self.lemma {
            doc.insert(
                name_of!(lemma).camel_case(),
                doc! {"$regex": lemma, "$options": "i"},
            );
        }

        if let Some(inflection) = &self.inflection {
            let key = inflection.declension.to_inflection_key()?;
            let key = format!("inflections.[].{}", key);
            let mut stages = Vec::<Vec<String>>::new();
            for part in key.split('.') {
                if stages.is_empty() {
                    stages.push(Vec::new());
                }
                if part != "[]" {
                    stages.last_mut().unwrap().push(part.to_owned());
                } else {
                    stages.push(Vec::new());
                }
            }

            fill_query(&mut doc, &stages, 0, &inflection.word);
        }

        Ok(doc)
    }
}

fn str(s: &impl ToString) -> Cow<str> {
    s.to_string().snake_case().into()
}

impl Declension {
    pub fn to_inflection_key(&self) -> Result<String, SafeError> {
        let mut s = Vec::<Cow<str>>::new();

        if let PartOfSpeech::Noun(_) = self.part_of_speech {
            s.push("noun".into());

            s.push(match &self.gender {
                Some(x) => str(x),
                None => return Err("gender required".to_string().into()),
            });

            s.push(match &self.number {
                Some(x) => str(x),
                None => return Err("number required".to_string().into()),
            });

            s.push(match &self.case {
                Some(x) => str(x),
                None => return Err("case required".to_string().into()),
            });

            s.push("[]".into());
            s.push("contracted".into());
        } else if PartOfSpeech::Verb == self.part_of_speech {
            s.push("verb".into());

            s.push(match &self.tense {
                Some(x) => str(x),
                None => return Err("tense required".to_string().into()),
            });

            s.push(match &self.theme {
                Some(x) => str(x),
                None => "thematic".into(),
            });

            s.push(match &self.contraction {
                Some(x) => str(x),
                None => "contracted".into(),
            });

            s.push(match &self.mood {
                Some(x) => str(x),
                None => return Err("mood required".to_string().into()),
            });

            s.push(match &self.voice {
                Some(x) => str(x),
                None => return Err("voice required".to_string().into()),
            });

            s.push(match &self.number {
                Some(x) => str(x),
                None => return Err("number required".to_string().into()),
            });

            s.push(match &self.person {
                Some(x) => str(x),
                None => return Err("person required".to_string().into()),
            });

            s.push("[]".into());
            s.push("contracted".into());
        } else if let PartOfSpeech::Article(_) = self.part_of_speech {
            s.push("article".into());
            s.push("[]".into());
            s.push("contracted".into());
        } else {
            return Err(anyhow!("part of speech not supported {:?}", self.part_of_speech).into());
        }

        Ok(s.join("."))
    }
}
