use isahc::{http::Error as IsachHttpError, Error as IsahcError};
use isahc::{http::StatusCode, prelude::*, Request};
use std::io::Error as IOError;
use url::{ParseError, Url};

const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[derive(Debug)]
pub enum RetrieverError {
    UrlParse(ParseError),
    IsahcBuilder(IsachHttpError),
    IsahcResponse(IsahcError),
    IsahcTextRetrieval(IOError),
    ResponseNotOk(StatusCode),
}

pub async fn search_term(term: &str) -> Result<String, RetrieverError> {
    let url = format!("{BASE_URL}{term}");
    let url = Url::parse(&url).map_err(RetrieverError::UrlParse)?;

    let mut response = Request::builder()
        .method("GET")
        .uri(url.as_str())
        .header("User-Agent", MY_USER_AGENT)
        .body(())
        .map_err(RetrieverError::IsahcBuilder)?
        .send_async()
        .await
        .map_err(RetrieverError::IsahcResponse)?;

    match response.status() {
        StatusCode::OK => response
            .text()
            .await
            .map_err(RetrieverError::IsahcTextRetrieval),
        other => Err(RetrieverError::ResponseNotOk(other)),
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
