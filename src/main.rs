#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Hello, world!");
    const URL: &str = "https://jsonplaceholder.typicode.com/todos?userId=1";
    let res = reqwest::get(URL).await?.text().await?;
    println!("Response: {:#?}", res);
    Ok(())
}
