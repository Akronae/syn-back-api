use anyhow::Context;

use tracing::{debug, info, warn};

use crate::{
    api::{
        lexicon::{
            lexicon_model::{LexiconEntry, LexiconFilter, LexiconFilterInflection},
            lexicon_repo::LexiconRepo,
        },
        verse::{verse_model::VerseFilter, verse_repo::VerseRepo},
    },
    borrow::Cow,
    error::SafeError,
    grammar::{Declension, DeclensionType, PartOfSpeech, Verse, Word},
    scrappers::{
        katabiblon,
        wiki::{details::SearchMode, errors::ParseWordError},
    },
    utils::str::closest::closest,
};

mod adverb;
mod article;
mod definition;
mod details;
mod errors;
mod noun;
mod numeral;
mod page;
mod parser;
mod participle;
mod particle;
mod preposition;
mod pronoun;
mod quantifier;
mod table;
mod verb;

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    let mut verse = VerseRepo::find_one(&VerseFilter {
        collection: Some("new_testament".to_string()),
        book: Some("matthew".to_string()),
        chapter_number: Some(1),
        verse_number: Some(17),
    })
    .await?
    .context("no verse")?;

    async fn find_in_lexicon(
        word: &str,
        declension: &Declension,
    ) -> Result<Option<LexiconEntry>, SafeError> {
        if declension.decl_type == Some(DeclensionType::Indeclinable)
            || declension.part_of_speech == PartOfSpeech::Particle
        {
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

    async fn update_word(verse: &mut Verse, word: &Word, index: usize) -> Result<(), SafeError> {
        let old = verse.words[index].clone();
        if old.text == word.text {
            return Ok(());
        }
        let confirmed = cliclack::confirm(format!(
            "change {} -> {} at word #{index} of verse {}:{}:{}?\n  '{}'",
            old.text,
            word.text,
            verse.book,
            verse.chapter_number,
            verse.verse_number,
            verse
                .words
                .iter()
                .map(|w| w.text.clone())
                .collect::<Vec<String>>()
                .join(" ")
        ))
        .initial_value(true)
        .interact()?;
        if !confirmed {
            return Ok(());
        }

        verse.words[index] = word.clone();
        VerseRepo::update_one(verse).await?;
        debug!(
            "updated verse {} {} {} {} word {} '{}'",
            verse.collection,
            verse.book,
            verse.chapter_number,
            verse.verse_number,
            index,
            word.text
        );
        Ok(())
    }

    for (word_i, word) in &mut verse.words.clone().iter_mut().enumerate() {
        debug!("processing #{word_i} word {}", word.text);

        let parsed;
        let mut declension = word.declension.clone();
        if let Some(already) = find_in_lexicon(&word.text, &word.declension).await? {
            debug!("{} already in lexicon", word.text);
            parsed = already;
        } else {
            debug!("{} not in lexicon, fetching", word.text);
            let res = parser::parse_word(
                word.text.clone().into(),
                &word.declension,
                &SearchMode::Query,
            )
            .await;
            match res {
                Err(e) => match e {
                    ParseWordError::NotFound(_) => {
                        warn!("{}", e);
                        warn!("checking on Katabiblon.");
                        let res =
                            katabiblon::parser::parse_word(&word.text, &word.declension).await?;
                        parsed = res;
                    }
                    _ => {
                        return Err(e.into());
                    }
                },
                Ok(res) => {
                    parsed = res.entry;
                    declension = res.declension;
                }
            }
        }

        let is_indeclinable = matches!(
            word.declension.decl_type,
            Some(DeclensionType::Indeclinable)
        );

        if parsed.inflections.first().is_some() && !is_indeclinable {
            let parsed_inflection = parsed.inflections.first().unwrap();
            let inflecteds = parsed_inflection.find_inflection(&declension);
            let inflecteds = inflecteds
                .iter()
                .map(|x| x.clone().into())
                .collect::<Vec<Cow<str>>>();
            let close = closest(word.text.clone().into(), &inflecteds)
                .first()
                .map(|x| x.to_string());

            if let Some(inflected) = close {
                if inflected != word.text {
                    debug!("{} changing to {}", word.text, inflected);
                    word.text = inflected.to_string();
                    update_word(&mut verse, word, word_i).await?;
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
            update_word(&mut verse, word, word_i).await?;
            debug!(
                "{} has no inflection, so changing to lemma {}",
                word.text, parsed.lemma
            );
        }

        if word.declension != declension {
            word.declension = declension.clone();
            update_word(&mut verse, word, word_i).await?;
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

    Ok(())
}
