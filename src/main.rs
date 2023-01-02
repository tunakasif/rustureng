use reqwest::{header::HeaderMap, header::USER_AGENT, Client, Response};

const WORD: &str = "telefon";
const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url: String = format!("{}{}", BASE_URL, WORD);
    let mut header_map: HeaderMap = HeaderMap::new();
    header_map.insert(USER_AGENT, MY_USER_AGENT.parse().unwrap());

    let response: Response = get_response(&url, header_map).await?;
    println!("Status: {:#?}", response.status());
    Ok(())
}

async fn get_response(
    url: &str,
    header_map: reqwest::header::HeaderMap,
) -> Result<reqwest::Response, reqwest::Error> {
    Client::new().get(url).headers(header_map).send().await
}
