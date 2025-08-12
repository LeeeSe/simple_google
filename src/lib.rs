use reqwest::header::{ACCEPT_LANGUAGE, USER_AGENT};
use scraper::{Html, Selector};
use serde::Serialize;
use std::error::Error;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub link: String,
    pub source: String,
    pub snippet: String,
}

pub async fn search_google(query: &str, num_pages: u32) -> Result<Vec<String>, Box<dyn Error>> {
    const USER_AGENT_STR: &str = "Mozilla/4.0 (compatible; MSIE 6.0; Nitro) Opera 8.50 [ja]";

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let consent_data = [
        ("set_eom", "true"),
        ("uxe", "none"),
        ("hl", "en"),
        ("pc", "srp"),
        ("gl", "DE"),
        ("x", "8"),
        ("bl", "user"),
        ("continue", "https://www.google.com/"),
    ];

    client
        .post("https://consent.google.com/save")
        .header(USER_AGENT, USER_AGENT_STR)
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .form(&consent_data)
        .send()
        .await?;

    let mut all_pages_html = Vec::new();

    for page in 0..num_pages {
        let start_index = page * 10;
        let encoded_query =
            url::form_urlencoded::byte_serialize(query.as_bytes()).collect::<String>();
        let url = format!(
            "https://www.google.com/search?hl=en&q={}&start={}",
            encoded_query, start_index
        );

        let res = client
            .get(&url)
            .header(USER_AGENT, USER_AGENT_STR)
            .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5")
            .send()
            .await?;

        if res.status().is_success() {
            all_pages_html.push(res.text().await?);
        } else {
            return Err(format!("Failed to fetch page {}, status: {}", page + 1, res.status()).into());
        }

        if num_pages > 1 {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(all_pages_html)
}

pub fn parse_results(html_content: &str) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let document = Html::parse_document(html_content);

    let result_selector = Selector::parse("div.ezO2md")?;
    let link_title_selector = Selector::parse("a.fuLhoc")?;
    let title_span_selector = Selector::parse("span.CVA68e")?;
    let source_selector = Selector::parse("span.dXDvrc")?;
    let snippet_selector = Selector::parse("span.FrIlee")?;

    let mut results = Vec::new();

    for element in document.select(&result_selector) {
        if let Some(link_element) = element.select(&link_title_selector).next() {
            let title = link_element
                .select(&title_span_selector)
                .next()
                .map_or(String::from("N/A"), |el| el.text().collect::<String>());

            let raw_link = link_element.value().attr("href").unwrap_or("");
            let link = if raw_link.starts_with("/url?q=") {
                raw_link
                    .split('&')
                    .next()
                    .unwrap_or("")
                    .replace("/url?q=", "")
            } else {
                raw_link.to_string()
            };

            let source = element
                .select(&source_selector)
                .next()
                .map_or(String::from("N/A"), |el| el.text().collect::<String>());

            let snippet = element
                .select(&snippet_selector)
                .next()
                .map_or(String::from("N/A"), |el| el.text().collect::<String>());

            results.push(SearchResult {
                title,
                link,
                source,
                snippet,
            });
        }
    }

    Ok(results)
}

pub async fn search_and_parse(query: &str, num_pages: u32) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let html_pages = search_google(query, num_pages).await?;
    let mut all_results = Vec::new();

    for html in html_pages.iter() {
        let mut parsed = parse_results(html)?;
        all_results.append(&mut parsed);
    }

    Ok(all_results)
}
