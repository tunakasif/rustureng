use scraper::{Html, Selector};

#[derive(Debug)]
pub enum TranslationResult {
    Valid(Vec<String>),
    Suggestions(Vec<String>),
    TermNotFound,
}

pub async fn parse_html_content(content: &str) -> TranslationResult {
    let document = Html::parse_document(content);
    let table_selector = Selector::parse("table").unwrap();
    let h1_selector = Selector::parse("h1").unwrap();

    let tables = document.select(&table_selector);

    match tables.count() {
        0 => {
            let term_not_found_h1_exists = document.select(&h1_selector).any(|h1| {
                "term not found" == h1.text().collect::<Vec<_>>().join("").trim().to_lowercase()
            });

            match term_not_found_h1_exists {
                true => TranslationResult::TermNotFound,
                _ => TranslationResult::Suggestions(get_suggestions(&document).await),
            }
        }
        _ => TranslationResult::Valid(get_results(&document).await),
    }
}

async fn get_results(document: &Html) -> Vec<String> {
    let selector = Selector::parse(r#"table > tbody > tr > td > a"#).unwrap();
    let entries = document.select(&selector);
    entries
        .into_iter()
        .enumerate()
        .take(20)
        .filter(|(i, _)| i % 2 == 1) // get every other entry
        .map(|(_, entry)| entry.text().collect::<Vec<_>>().join("")) // get the text
        .collect()
}

async fn get_suggestions(document: &Html) -> Vec<String> {
    let selector = Selector::parse("ul.suggestion-list > li > a").unwrap();
    document
        .select(&selector)
        .map(|li| li.text().collect::<Vec<_>>().join(""))
        .collect::<Vec<_>>()
}
