use url::{ParseError, Url};

const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const MY_USER_AGENT: &str = "MyAgent";

#[derive(Debug)]
pub enum RetrieverError {
    UrlParse(ParseError),
    UreqClient(ureq::Error),
    UreqTextRetrieval(ureq::Error),
}

pub async fn search_term(term: &str) -> Result<String, RetrieverError> {
    let url = format!("{BASE_URL}{term}");
    let url = Url::parse(&url).map_err(RetrieverError::UrlParse)?;

    let body: String = ureq::get(url.as_str())
        .header("User-Agent", MY_USER_AGENT)
        .call()
        .map_err(RetrieverError::UreqClient)?
        .body_mut()
        .read_to_string()
        .map_err(RetrieverError::UreqTextRetrieval)?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TERMS: [&str; 7] = [
        "gönderi kutusu",      // valid multi-word turkish with turkish character
        "mobile phone",        // valid multi-word english
        "sağır",               // valid single word turkish with turkish character
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
