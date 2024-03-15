use anyhow::Context;
use mongodb::bson::Document;
use tracing::{debug, info};

use crate::{
    api::{
        lexicon::{
            lexicon_model::{LexiconEntry, LexiconFilter, LexiconFilterInflection},
            lexicon_repo::LexiconRepo,
        },
        verse::{verse_model::VerseFilter, verse_repo::VerseRepo},
    },
    error::SafeError,
    grammar::{Contraction, Declension, Mood, Number, Person, Tense, Theme, Voice},
    persistence::get_db,
    task::sleep_ms,
};

mod definition;
mod details;
mod noun;
mod page;
mod parser;
mod table;
mod verb;

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    let mut verse = VerseRepo::find_one(&VerseFilter {
        collection: Some("new_testament".to_string()),
        book: Some("matthew".to_string()),
        chapter_number: Some(1),
        verse_number: Some(2),
    })
    .await?
    .context("no verse")?;

    dbg!("okkok!!!");
    let col = get_db().await?.collection::<Document>("lexicon");
    let a = col
        .find_one(
            LexiconFilter {
                lemma: Some("γεννάω".into()),
                // inflection: Some(LexiconFilterInflection {
                //     declension: Declension {
                //         tense: Some(Tense::Present),
                //         theme: Some(Theme::Thematic),
                //         contraction: Some(Contraction::Uncontracted),
                //         voice: Some(Voice::Active),
                //         person: Some(Person::First),
                //         number: Some(Number::Singular),
                //         mood: Some(Mood::Indicative),
                //         ..Declension::partial_default(crate::grammar::PartOfSpeech::Verb)
                //     },
                //     word: "γεννᾰ́ω".to_string(),
                // }),
                ..Default::default()
            }
            .to_document()?,
            None,
        )
        .await?;
    dbg!(a);
    dbg!("llalala!");

    async fn find_in_lexicon(
        word: &str,
        declension: &Declension,
    ) -> Result<Option<LexiconEntry>, SafeError> {
        if declension.indeclinable.unwrap_or(false) {
            return LexiconRepo::find_one(LexiconFilter {
                lemma: Some(word.to_owned()),
                ..Default::default()
            })
            .await;
        }

        LexiconRepo::find_one(LexiconFilter {
            inflection: Some(LexiconFilterInflection {
                declension: declension.to_owned(),
                word: word.to_string(),
            }),
            ..Default::default()
        })
        .await
    }

    // let mut has_changes = false;

    for (word_i, word) in &mut verse.words.clone().iter_mut().enumerate() {
        sleep_ms(1000).await;

        let parsed;
        let mut parsed_decl = None;
        if let Some(already) = find_in_lexicon(&word.text, &word.declension).await? {
            debug!("{} already in lexicon", word.text);
            parsed = already;
        } else {
            debug!("{} not in lexicon, fetching", word.text);
            let res = parser::parse_word(word.text.clone().into(), &word.declension).await?;
            parsed = res.entry;
            parsed_decl = Some(res.declension);
        }

        if let Some(parsed_inflection) = parsed.inflections.first() {
            let inflected = parsed_inflection.find_inflection(&word.declension);

            if let Some(inflected) = inflected {
                if inflected != word.text {
                    debug!("{} changing to {}", word.text, inflected);
                    word.text = inflected.to_owned();
                    // has_changes = true;
                    verse.words[word_i] = word.clone();
                    VerseRepo::update_one(&verse).await?;
                } else {
                    debug!("{} already inflected", word.text);
                }
            } else {
                debug!(
                    "{} not found with inflection {:?} in {:?}",
                    word.text, word.declension, parsed.inflections
                );
            }
        } else if parsed.lemma != word.text {
            word.text = parsed.lemma.to_owned();
            // has_changes = true;
            verse.words[word_i] = word.clone();
            VerseRepo::update_one(&verse).await?;
            debug!(
                "{} has no inflection, so changing to lemma {}",
                word.text, parsed.lemma
            );
        }

        if let Some(parsed_decl) = parsed_decl {
            word.declension = parsed_decl;
            // has_changes = true;
            verse.words[word_i] = word.clone();
            VerseRepo::update_one(&verse).await?;
        }

        if find_in_lexicon(&word.text, &word.declension)
            .await?
            .is_none()
        {
            LexiconRepo::insert_one(parsed.clone()).await?;

            info!(
                "{:?} imported into {}",
                parsed,
                LexiconRepo::COLLECTION_NAME
            );
        }
    }

    // if has_changes {
    //     debug!(
    //         "updating verse {} {} {} {}",
    //         verse.collection, verse.book, verse.chapter_number, verse.verse_number
    //     );
    //     VerseRepo::update_one(&verse).await?;
    // }

    Ok(())
}
