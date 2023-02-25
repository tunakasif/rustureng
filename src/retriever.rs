use chttp::http::StatusCode;
use chttp::prelude::*;
use std::io::Error as IOError;
use url::{ParseError, Url};

const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[derive(Debug)]
pub enum RetrieverError {
    UrlParse(ParseError),
    ChttpBuilder(chttp::http::Error),
    ChttpResponse(chttp::Error),
    ChttpTextRetrieval(IOError),
    ResponseNotOk(StatusCode),
}

pub async fn search_term(term: &str) -> Result<String, RetrieverError> {
    let url = format!("{BASE_URL}{term}");
    let url = Url::parse(&url).map_err(|err| RetrieverError::UrlParse(err))?;

    let mut response = Request::builder()
        .method("GET")
        .uri(url.as_str())
        .header("User-Agent", MY_USER_AGENT)
        .body(())
        .map_err(|err| RetrieverError::ChttpBuilder(err))?
        .send_async()
        .await
        .map_err(|err| RetrieverError::ChttpResponse(err))?;

    match response.status() {
        StatusCode::OK => response
            .text_async()
            .await
            .map_err(RetrieverError::ChttpTextRetrieval),
        other => Err(RetrieverError::ResponseNotOk(other)),
    }
}
