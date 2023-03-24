use rustureng::retriever::{self, RetrieverError};
use scraper::{Html, Selector};
// use rustureng::retriever_reqwest::{self as retriever, RetrieverError};
use std::{env, io::Write};

const WORD: &str = "test";
const WRITE_TO_FILE: bool = false;

#[allow(dead_code)]
struct ValidTranslationEntry {
    pub number: usize,
    pub word: String,
    pub translation: String,
    pub part_of_speech: String,
}

#[allow(dead_code)]
impl ValidTranslationEntry {
    fn new(number: usize, word: String, translation: String, part_of_speech: String) -> Self {
        Self {
            number,
            word,
            translation,
            part_of_speech,
        }
    }
}

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

    parse_html_content(&content).await;
    Ok(())
}

async fn parse_html_content(content: &str) {
    let document = Html::parse_document(content);
    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let td_with_a_selector = Selector::parse("td > a").unwrap();

    let translation_result = document
        .select(&table_selector)
        .map(|table| {
            table
                .select(&tr_selector)
                .filter(|tr| tr.select(&td_with_a_selector).count() > 0)
                // .map(|tr| tr.text().collect::<Vec<_>>().join(""))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let a = translation_result[1][2]
        .select(&td_selector)
        .map(|td| {
            if td.inner_html().contains("<i>") {
                td.inner_html()
            } else {
                td.text().collect::<Vec<_>>().join("")
            }
        })
        .map(|x| x.trim().to_string())
        .filter(|td| !td.is_empty())
        .collect::<Vec<_>>();
    println!("{:#?}", a);
}

async fn save_string_to_file(file_name: &str, content: &str) {
    let mut file = std::fs::File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
