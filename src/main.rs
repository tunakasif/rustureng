mod err;

use err::RusTurengError;
use reqwest::{header::HeaderMap, header::USER_AGENT, Client};
use scraper::{Html, Selector};
use std::io::Write;

const WORD: &str = "telefon";
const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";
const WRITE_TO_FILE: bool = false;

#[derive(Debug)]
enum TranslationResult {
    Valid,
    Suggestions(Vec<String>),
    TermNotFound,
}

#[tokio::main]
async fn main() -> Result<(), RusTurengError> {
    let url: String = format!("{BASE_URL}{WORD}");
    let mut header_map: HeaderMap = HeaderMap::new();
    header_map.insert(USER_AGENT, MY_USER_AGENT.parse().unwrap());

    let content = get_content(&url, header_map).await?;
    if WRITE_TO_FILE {
        save_string_to_file("content.html", &content);
    }
    let trans_result = parse_html_content(&content);
    println!("{trans_result:#?}");

    Ok(())
}

fn parse_html_content(content: &str) -> TranslationResult {
    let document = Html::parse_document(content);
    let table_selector = Selector::parse("table").unwrap();
    let h1_selector = Selector::parse("h1").unwrap();

    let tables = document.select(&table_selector);

    match tables.count() {
        0 => {
            let term_not_found_h1_exists = document.select(&h1_selector).any(|h1| {
                "term not found" == h1.text().collect::<Vec<_>>().join("").trim().to_lowercase()
            });

            match term_not_found_h1_exists {
                true => TranslationResult::TermNotFound,
                _ => TranslationResult::Suggestions(get_suggestions(&document)),
            }
        }
        _ => TranslationResult::Valid,
    }
}

fn get_suggestions(document: &Html) -> Vec<String> {
    let selector = Selector::parse("ul.suggestion-list > li > a").unwrap();
    document
        .select(&selector)
        .map(|li| li.text().collect::<Vec<_>>().join(""))
        .collect::<Vec<_>>()
}

fn save_string_to_file(file_name: &str, content: &str) {
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

async fn get_content(
    url: &str,
    header_map: reqwest::header::HeaderMap,
) -> Result<String, RusTurengError> {
    let response = Client::new().get(url).headers(header_map).send().await?;
    if response.status().is_success() {
        let content = response.text().await?;
        Ok(content)
    } else {
        Err(RusTurengError::ResponseNotOk(response))
    }
}
