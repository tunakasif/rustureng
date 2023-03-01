use reqwest::{header::HeaderMap, header::USER_AGENT, Client, StatusCode};
use url::{ParseError, Url};

const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[derive(Debug)]
pub enum RetrieverError {
    UrlParse(ParseError),
    ReqwestClient(reqwest::Error),
    ReqwestTextRetrieval,
    ResponseNotOk(StatusCode),
    ResponseNotOkReqwest(StatusCode),
}

pub async fn search_term(term: &str) -> Result<String, RetrieverError> {
    let url = format!("{BASE_URL}{term}");
    let url = Url::parse(&url).map_err(RetrieverError::UrlParse)?;

    let mut header_map = HeaderMap::new();
    header_map.insert(USER_AGENT, MY_USER_AGENT.parse().unwrap());

    let response = Client::new()
        .get(url)
        .headers(header_map)
        .send()
        .await
        .map_err(RetrieverError::ReqwestClient)?;

    match response.status() {
        StatusCode::OK => response
            .text()
            .await
            .map_err(|_| RetrieverError::ReqwestTextRetrieval),
        _ => Err(RetrieverError::ResponseNotOkReqwest(response.status())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TERMS: [&str; 7] = [
        "gönderi kutusu",     // valid multi-word turkish with turkish character
        "mobile phone",        // valid multi-word english
        "sağır",             // valid single word turkish with turkish character
        "mailbox",             // valid single word english
        "asda",                // suggestion single word
        "asdfsdfafdasfdafd",   // invalid single word
        "asdf sdfafdasfd afd", // invalid single word
    ];

    #[tokio::test]
    async fn test_term_retrieval_is_ok() {
        for term in TEST_TERMS {
            let result = search_term(term).await;
            assert!(result.is_ok());
        }
    }
}
