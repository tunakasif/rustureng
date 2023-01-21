mod err;

use err::RusTurengError;
use reqwest::{header::HeaderMap, header::USER_AGENT, Client};
use scraper::{Html, Selector};
use std::io::Write;

const WORD: &str = "telefon";
const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[tokio::main]
async fn main() -> Result<(), RusTurengError> {
    let url: String = format!("{}{}", BASE_URL, WORD);
    let mut header_map: HeaderMap = HeaderMap::new();
    header_map.insert(USER_AGENT, MY_USER_AGENT.parse().unwrap());

    let content = get_content(&url, header_map).await?;
    save_string_to_file("content.html", &content);
    parse_html_content(&content);

    Ok(())
}

fn parse_html_content(content: &str) {
    let document = Html::parse_document(content);
    let selector = Selector::parse(r#"table > tbody > tr > td > a"#).unwrap();

    for (i, elem) in document.select(&selector).enumerate() {
        if i == 10 {
            break;
        } else if i % 2 == 1 {
            println!("{}", elem.text().collect::<Vec<_>>().join(","));
        }
    }
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
