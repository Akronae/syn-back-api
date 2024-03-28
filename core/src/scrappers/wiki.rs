use anyhow::Context;

use tracing::{debug, info};

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
    grammar::{Declension, PartOfSpeech, Verse, Word},
    task::sleep_ms,
    utils::str::closest::closest,
};

mod article;
mod conjunction;
mod definition;
mod details;
mod noun;
mod page;
mod parser;
mod pronoun;
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

    async fn find_in_lexicon(
        word: &str,
        declension: &Declension,
    ) -> Result<Option<LexiconEntry>, SafeError> {
        if declension.indeclinable.unwrap_or(false)
            || declension.part_of_speech == PartOfSpeech::Conjunction
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
        let mut parsed_decl = None;
        if let Some(already) = find_in_lexicon(&word.text, &word.declension).await? {
            debug!("{} already in lexicon", word.text);
            parsed = already;
        } else {
            debug!("{} not in lexicon, fetching", word.text);
            sleep_ms(1000).await;
            let res = parser::parse_word(word.text.clone().into(), &word.declension).await?;
            parsed = res.entry;
            parsed_decl = Some(res.declension);
        }

        if let Some(parsed_inflection) = parsed.inflections.first() {
            let inflecteds = parsed_inflection.find_inflection(&word.declension);
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

        if let Some(parsed_decl) = parsed_decl {
            word.declension = parsed_decl;
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
