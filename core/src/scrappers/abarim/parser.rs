use anyhow::Context;
use std::{collections::HashMap, error::Error, str::FromStr};
use tracing::{field::debug, *};

use crate::{
    error::SafeError,
    grammar::{Language, Verse, Word},
    scrappers::abarim::declension,
    texts::{Book, Collection},
    utils::str::{capitalize::Capitalize, decode_html::DecodeHtml},
};

pub struct ParsedChapter {
    pub verses: Vec<Verse>,
}

#[allow(dead_code)]
pub async fn parse_chapter(chapter: isize, book: Book) -> Result<ParsedChapter, SafeError> {
    info!("Parsing chapter {} of {}", chapter, book);

    let base_url = "https://www.abarim-publications.com/Interlinear-New-Testament";
    let collection = Collection::NewTestament;
    let full_url = &get_url(base_url, book, chapter);
    debug!("Fetching {}", full_url);
    let res = reqwest::get(full_url).await?.text().await?;

    let dom = tl::parse(res.as_str(), tl::ParserOptions::default())?;
    let parser = dom.parser();

    let verses = dom
        .query_selector("[id*='Byz-AVerse']")
        .context("Could not find verses")?;

    debug!("Found {} verses", verses.to_owned().count());

    let mut parsed_verses: Vec<Verse> = vec![];
    for (verse_i, _) in verses.enumerate() {
        let v = parse_verse(
            collection,
            book,
            chapter,
            verse_i as isize + 1,
            &dom,
            parser,
        )?;
        parsed_verses.push(v);
    }

    Ok(ParsedChapter {
        verses: parsed_verses,
    })
}

fn get_url(base: &str, book: Book, chapter: isize) -> String {
    format!(
        "{base}/{b}/{b}-{chapter}-parsed.html",
        b = book.to_string().capitalize()
    )
}

fn get_verse_translation(verse_number: isize, dom: &tl::VDom) -> Option<String> {
    let parser = dom.parser();
    let verse_selector = &format!("[id*='KJV-AVerse-{verse_number}']");
    let trans = dom
        .query_selector(verse_selector)?
        .next()?
        .get(parser)?
        .inner_text(parser)
        .to_string();

    Some(trans)
}

fn parse_verse(
    collection: Collection,
    book: Book,
    chapter_number: isize,
    verse_number: isize,
    dom: &tl::VDom,
    parser: &tl::Parser,
) -> Result<Verse, Box<dyn Error + Send + Sync>> {
    let trans = get_verse_translation(verse_number, dom)
        .with_context(|| format!("could not find verse {verse_number} translation"))?;

    let words_wrapper = dom
        .query_selector(&format!("[id*='Byz-AVerse-{verse_number}']"))
        .context("could not query selector")?
        .next()
        .with_context(|| format!("could not find greek word wrapper for verse {verse_number}"))?
        .get(parser)
        .context("could not get node")?;

    let words_wrapper_html = words_wrapper.inner_html(parser).to_string();
    let words_wrapper_dom = tl::parse(&words_wrapper_html, tl::ParserOptions::default())?;

    let words = words_wrapper_dom
        .query_selector(".contB")
        .context("could not query select")?;

    let mut parsed_words: Vec<Word> = vec![];
    for (word_i, word) in words.enumerate() {
        let w = parse_word(
            word,
            &words_wrapper_dom,
            book,
            chapter_number,
            verse_number,
            word_i as isize,
        )?;
        parsed_words.push(w);
    }

    Ok(Verse {
        collection: collection,
        book: book,
        chapter_number: chapter_number,
        verse_number: verse_number,
        translation: HashMap::from([(Language::English.lang_code(), trans)]),
        words: parsed_words,
    })
}

fn parse_word(
    word: tl::NodeHandle,
    words_wrapper_dom: &tl::VDom,
    book: Book,
    chapter_number: isize,
    verse_number: isize,
    word_number: isize,
) -> Result<Word, Box<dyn Error + Send + Sync>> {
    let word_html = word
        .get(words_wrapper_dom.parser())
        .context("context")?
        .inner_html(words_wrapper_dom.parser());

    let word_dom = tl::parse(&word_html, tl::ParserOptions::default())?;

    let greek = &word_dom
        .query_selector(".HebFs")
        .context("could not get query selector")?
        .next()
        .context("could not get nth")?
        .get(word_dom.parser())
        .context("could not get greek")?
        .inner_text(word_dom.parser())
        .to_string()
        .decode_html();

    let english = word_dom
        .query_selector(".blueF")
        .context("could not get query selector")?
        .next()
        .context("could not get nth")?
        .get(word_dom.parser())
        .context("could not get english")?
        .inner_text(word_dom.parser());

    let declension = word_dom
        .query_selector(".greenF")
        .context("could not get query selector")?
        .next()
        .context("could not get nth")?
        .get(word_dom.parser())
        .context("could not get declension")?
        .as_tag()
        .context("could not get tag")?;

    let declension_comps: Vec<String> = declension
        .children()
        .all(word_dom.parser())
        .iter()
        .filter(|d| d.inner_text(word_dom.parser()) != "")
        .map(|e| e.inner_text(word_dom.parser()).to_string())
        .collect();

    let declension =
        match declension::get_word_fix(book, chapter_number, verse_number, word_number, greek) {
            Some(d) => d,
            None => declension::get_word_declension(&declension_comps),
        };

    Ok(Word {
        text: greek.to_owned(),
        language: Language::Greek.lang_code(),
        translation: HashMap::from([(Language::English.lang_code(), english.to_string())]),
        declension,
    })
}
