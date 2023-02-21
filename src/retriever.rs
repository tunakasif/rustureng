#[allow(unused_imports)]
use reqwest::{header::HeaderMap, header::USER_AGENT, Client};
use reqwest::{Error, Response};
// use std::future::Future;

const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[derive(Debug)]
pub enum RusTurengError {
    Reqwest(Error),
    ResponseNotOk(Response),
}

impl From<Error> for RusTurengError {
    fn from(err: Error) -> Self {
        RusTurengError::Reqwest(err)
    }
}

impl From<Response> for RusTurengError {
    fn from(resp: Response) -> Self {
        RusTurengError::ResponseNotOk(resp)
    }
}

pub async fn search_term(term: &str) -> Result<String, RusTurengError> {
    let url: String = format!("{BASE_URL}{term}");
    let mut header_map: HeaderMap = HeaderMap::new();
    header_map.insert(USER_AGENT, MY_USER_AGENT.parse().unwrap());
    get_content(&url, header_map).await
}

async fn get_content(url: &str, header_map: HeaderMap) -> Result<String, RusTurengError> {
    let response = Client::new().get(url).headers(header_map).send().await?;
    if response.status().is_success() {
        let content = response.text().await?;
        Ok(content)
    } else {
        Err(RusTurengError::ResponseNotOk(response))
    }
}
