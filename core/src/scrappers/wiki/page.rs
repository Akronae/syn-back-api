use scraper::Html;
use tracing::debug;

use crate::error::SafeError;

pub async fn scrap(lemma: &str) -> Result<Html, SafeError> {
    let url = build_scrap_url(lemma);
    debug!("fetching {url}");
    let res = reqwest::get(&url).await?.text().await?;
    let doc = Html::parse_document(&res);
    Ok(doc)
}

fn build_scrap_url(lemma: &str) -> String {
    // format!("https://en.wiktionary.org/api/rest_v1/page/html/{}", lemma)
    format!("https://en.wiktionary.org/wiki/{}", lemma)
}
