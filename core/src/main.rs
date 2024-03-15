use api::lexicon::lexicon_model::{
    LexiconEntry, VerbInflectionContractions, VerbInflectionForm, VerbInflectionMoods,
    VerbInflectionNumbers, VerbInflectionPersons, VerbInflectionTenses, VerbInflectionThemes,
    VerbInflectionVoices, WordInflection,
};
use error::SafeError;
use futures::StreamExt;
use grammar::{Person, Word};
use mongodb::{bson::doc, Client};

mod api;
mod borrow;
mod config;
mod error;
mod grammar;
mod log;
mod persistence;
mod scrappers;
mod task;
mod texts;
mod utils;

#[tokio::main]
async fn main() -> Result<(), SafeError> {
    log::init()?;

    let db =
        Client::with_uri_str("mongodb+srv://root:O9ocuxMHUy1gCLu2@wl-dev.e9nbpbd.mongodb.net/")
            .await?;
    let col = db.database("mydbbb").collection::<LexiconEntry>("mycol");

    let base_doc = LexiconEntry {
        lemma: "a".into(),
        definitions: Default::default(),
        inflections: Vec::from([WordInflection {
            verb: Some(VerbInflectionTenses {
                present: Some(VerbInflectionThemes {
                    thematic: Some(VerbInflectionContractions {
                        contracted: Some(VerbInflectionMoods {
                            indicative: Some(VerbInflectionVoices {
                                active: Some(VerbInflectionNumbers {
                                    singular: Some(VerbInflectionPersons {
                                        first: Some(Vec::from([VerbInflectionForm {
                                            contracted: Some("b".into()),
                                            ..Default::default()
                                        }])),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }]),
    };

    dbg!("fetching entry..");
    let entry = col.find(doc! {}, None).await?.next().await;
    if entry.is_none() {
        dbg!("no entry, inserting default..");
        col.insert_one(base_doc, None).await?;
    }
    dbg!("fetching entry again..");
    let entry = col.find(doc! {}, None).await?.next().await;

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), SafeError> {
//     log::init()?;

//     // scrappers::abarim::import().await?;
//     // scrappers::katabiblon::import().await?;
//     scrappers::wiki::import().await?;

//     api::init().await?;

//     Ok(())
// }
