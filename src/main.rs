use rustureng::parser::{parse_html_content, TranslationResult};
use rustureng::retriever::{self, RetrieverError};
// use rustureng::retriever_reqwest::{self as retriever, RetrieverError};
use std::{env, io::Write};

const WORD: &str = "test";
const WRITE_TO_FILE: bool = false;
const QUIET: bool = false;

#[tokio::main]
async fn main() -> Result<(), RetrieverError> {
    let args: Vec<String> = env::args().map(|x| x.to_lowercase()).collect();
    let term = match args.len() {
        1 => WORD.to_string(),
        _ => args[1..].join(" "),
    };

    let content = retriever::search_term(&term).await?;
    if WRITE_TO_FILE {
        save_string_to_file("content.html", &content).await;
    }

    let translation_result = parse_html_content(&content).await;
    if !QUIET {
        match translation_result {
            TranslationResult::Valid(results) => {
                for result in results {
                    for entry in result {
                        println!("{}", entry);
                    }
                    println!();
                }
            }
            TranslationResult::Suggestions(suggestions) => {
                println!("{:#?}", suggestions);
            }
            TranslationResult::TermNotFound => {
                println!("Term not found");
            }
        }
    }
    Ok(())
}

async fn save_string_to_file(file_name: &str, content: &str) {
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
