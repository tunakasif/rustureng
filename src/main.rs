const WORD: &str = "telefon";
const BASE_URL: &str = "https://tureng.com/en/turkish-english/";
const USER_AGENT: &str = "MyAgent";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}{}", BASE_URL, WORD);

    let res = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;

    println!("Status: {:#?}", res.status());

    Ok(())
}
