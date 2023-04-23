use dialoguer::{theme::ColorfulTheme, Select};
use rustureng::parser::{parse_html_content, TranslationResult};
use rustureng::retriever::{self, RetrieverError};
// use rustureng::retriever_reqwest::{self as retriever, RetrieverError};
use std::env;

const WORD: &str = "test";
const INTERACTIVE: bool = true;

#[tokio::main]
async fn main() -> Result<(), RetrieverError> {
    let term = get_search_term();
    let mut translation_result = query(&term).await?;
    if INTERACTIVE {
        translation_result = get_valid_result_from_suggestions(translation_result).await;
    }
    translation_result.display();
    Ok(())
}

fn get_search_term() -> String {
    let args: Vec<String> = env::args().map(|x| x.to_lowercase()).collect();
    match args.len() {
        1 => WORD.to_string(),
        _ => args[1..].join(" "),
    }
}

async fn query(term: &str) -> Result<TranslationResult, RetrieverError> {
    let content = retriever::search_term(term).await?;
    Ok(parse_html_content(&content).await)
}

async fn get_valid_result_from_suggestions(result: TranslationResult) -> TranslationResult {
    if let TranslationResult::Suggestions(suggestions) = result {
        if let Some(term) = choose_from_suggestions(&suggestions) {
            query(&term)
                .await
                .unwrap_or(TranslationResult::Suggestions(suggestions))
        } else {
            TranslationResult::Suggestions(suggestions)
        }
    } else {
        result
    }
}

fn choose_from_suggestions(suggestions: &[String]) -> Option<String> {
    let valid_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Maybe the correct one is: ")
        .default(0)
        .items(suggestions)
        .interact_opt()
        .unwrap();

    valid_selection.map(|idx| suggestions[idx].to_string())
}
