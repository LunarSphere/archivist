// 1 fetch the url with reqwest
// 2 extract hrefs
// 3 in main.rs print out the hrefs we found


use reqwest::header::{HeaderMap, USER_AGENT};

pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    // Some sites block requests with no/odd User-Agent.
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "rust-web-scraper/0.1".parse().unwrap());

    // TODO: learn how to build client once and reuseit
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    client
        .get(url)
        .send()
        .await?
        // Turns 404/500 into an error right here.
        // Without this, prevents parsing not found pages
        .error_for_status()?
        .text()
        .await
}