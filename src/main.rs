use select::document::Document;
use select::predicate::Name;

const URL: &str = "https://tureng.com/en/turkish-english/";
const WORD: &str = "kedi";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:108.0) Gecko/20100101 Firefox/108.0";

// User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:108.0) Gecko/20100101 Firefox/108.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Client with a "User-Agent" header
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());
    headers.insert("Accept", "text/html".parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    println!("Client: {:?}", client);

    // Make the GET request to "https://www.tureng.com" using the Client
    let request_url = format!("{}{}", URL, WORD);
    println!("Requesting {}", request_url);
    let resp = client.get(request_url).send().await?;
    println!("Response: {}", resp.status());

    // Check that the request was successful
    assert!(resp.status().is_success());

    // Read the response body as a string
    let body = resp.text().await?;

    // Create a Document from the response body string
    let document = Document::from(&*body);

    // Find all table entries and print them in a formatted manner
    for node in document.find(Name("td")) {
        println!("{}", node.text());
    }

    Ok(())
}
