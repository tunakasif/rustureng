use chttp::http::StatusCode;
use chttp::prelude::*;
use std::io::Error as IOError;
use url::{ParseError, Url};

const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[derive(Debug)]
pub enum RusTurengError {
    ChttpBuilder,
    ChttpResponse,
    ChttpTextRetrieval(IOError),
    ResponseNotOk(StatusCode),
}

pub async fn search_term(term: &str) -> Result<String, RusTurengError> {
    let url = format!("{BASE_URL}{term}");
    let url = Url::parse(&url).map_err(|err| RusTurengError::UrlParse(err))?;

    let mut response = Request::builder()
        .method("GET")
        .uri(url.as_str())
        .header("User-Agent", MY_USER_AGENT)
        .body(())
        .map_err(|_| RusTurengError::ChttpBuilder)?
        .send_async()
        .await
        .map_err(|_| RusTurengError::ChttpResponse)?;

    match response.status() {
        StatusCode::OK => response
            .text_async()
            .await
            .map_err(RusTurengError::ChttpTextRetrieval),
        other => Err(RusTurengError::ResponseNotOk(other)),
    }
}
