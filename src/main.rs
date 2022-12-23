const URL: &str = "https://tureng.com/en/turkish-english/kedi";
const USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:108.0) Gecko/20100101 Firefox/108.0";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .user_agent(USER_AGENT)
        .build()?;

    let res = client.get(URL).send().await?;
    println!("Status: {}", res.status());

    Ok(())
}
