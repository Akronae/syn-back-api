use reqwest::Method;
use scraper::{selectable::Selectable, ElementRef, Html};
use tracing::debug;

use crate::{error::SafeError, request::request, utils::scrapper::select::select};

pub async fn scrap(lemma: &str) -> Result<Html, SafeError> {
    let url = build_scrap_url(lemma);
    debug!("fetching {url}");
    let res = request()
        .with_method(Method::GET)
        .with_url(url)
        .with_cache(true)
        .text()
        .await?;
    let mut doc = Html::parse_document(&res);
    let doc_clone = doc.clone();
    let s = select("#Ancient_Greek")?;
    let header = doc_clone.select(&s).next().unwrap().parent().unwrap();

    let mut passed_anc_greek_section = false;

    for node in header.next_siblings() {
        if !node.value().is_element() {
            continue;
        }
        if ElementRef::wrap(node)
            .unwrap()
            .select(&select("#Greek")?)
            .next()
            .is_some()
        {
            passed_anc_greek_section = true;
        }
        if passed_anc_greek_section {
            doc.tree.get_mut(node.id()).unwrap().detach();
        }
    }
    for node in header.prev_siblings() {
        doc.tree.get_mut(node.id()).unwrap().detach();
    }

    Ok(doc)
}

fn build_scrap_url(lemma: &str) -> String {
    format!("https://en.wiktionary.org/wiki/{}", lemma)
}
