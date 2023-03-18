use scraper::{Html, Selector};
// use rustureng::parser::parse_html_content;
use rustureng::retriever::{self, RetrieverError};
// use rustureng::retriever_reqwest::{self as retriever, RetrieverError};
use std::{env, io::Write};

const WORD: &str = "test";
const WRITE_TO_FILE: bool = false;

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

    let translation_result = parse(&content).await;
    println!("{:#?}", translation_result.len());
    Ok(())
}

async fn parse(content: &str) -> Vec<Vec<String>> {
    let document = Html::parse_document(content);
    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_with_a_selector = Selector::parse("td > a").unwrap();

    let tables = document.select(&table_selector);
    tables
        .map(|table| {
            table
                .select(&tr_selector)
                .filter(|tr| tr.select(&td_with_a_selector).count() > 0)
                .map(|tr| tr.text().collect::<Vec<_>>().join(""))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>()
}

async fn save_string_to_file(file_name: &str, content: &str) {
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
