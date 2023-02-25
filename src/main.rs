use rustureng::parser::parse_html_content;
use rustureng::retriever::{self, RetrieverError};
use std::{env, io::Write};

const WORD: &str = "telefon";
const WRITE_TO_FILE: bool = false;

#[tokio::main]
async fn main() -> Result<(), RetrieverError> {
    let args: Vec<String> = env::args().collect();
    let term = match args.len() {
        1 => WORD.to_string(),
        _ => args[1..].join(" "),
    };

    let content = retriever::search_term(&term).await?;
    if WRITE_TO_FILE {
        save_string_to_file("content.html", &content).await;
    }
    let translation_result = parse_html_content(&content).await;
    println!("{translation_result:#?}");
    Ok(())
}

async fn save_string_to_file(file_name: &str, content: &str) {
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
