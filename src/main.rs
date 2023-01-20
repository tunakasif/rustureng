use reqwest::{header::HeaderMap, header::USER_AGENT, Client, Response};
mod err;

use err::RusTurengError;

const WORD: &str = "telefon";
const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[tokio::main]
async fn main() -> Result<(), RusTurengError> {
    let url: String = format!("{}{}", BASE_URL, WORD);
    let mut header_map: HeaderMap = HeaderMap::new();
    header_map.insert(USER_AGENT, MY_USER_AGENT.parse().unwrap());

    let content = get_content(&url, header_map).await?;
    println!("Lenght: {:#?}", content.len());
    Ok(())
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
