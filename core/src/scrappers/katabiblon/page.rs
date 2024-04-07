use scraper::{Html};
use tracing::debug;
use url::Url;

use crate::{error::SafeError};

pub async fn scrap(word: &str, opt: &Option<i32>) -> Result<Html, SafeError> {
    let url = build_scrap_url(word, opt)?;
    debug!("fetching {url}");
    let res = reqwest::get(&url).await?.text().await?;
    let doc = Html::parse_document(&res);
    Ok(doc)
}

fn build_scrap_url(word: &str, opt: &Option<i32>) -> Result<String, SafeError> {
    let base_url = "https://lexicon.katabiblon.com/index.php";

    let mut url = Url::parse(base_url)?;
    url.query_pairs_mut()
        .append_pair("search", word)
        .append_pair("opt", &opt.unwrap_or(1).to_string());

    Ok(url.to_string())
}
