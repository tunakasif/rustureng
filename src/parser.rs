use scraper::{Html, Selector};

#[derive(Debug)]
pub enum TranslationResult {
    Valid(Vec<Vec<String>>),
    Suggestions(Vec<String>),
    TermNotFound,
}

pub async fn parse_html_content(content: &str) -> TranslationResult {
    let document = Html::parse_document(content);
    let translation_result = get_results(&document).await;

    if translation_result.is_empty() {
        let h1_selector = Selector::parse("h1").unwrap();
        let term_not_found_h1_exists = document.select(&h1_selector).any(|h1| {
            "term not found" == h1.text().collect::<Vec<_>>().join("").trim().to_lowercase()
        });

        match term_not_found_h1_exists {
            true => TranslationResult::TermNotFound,
            _ => TranslationResult::Suggestions(get_suggestions(&document).await),
        }
    } else {
        TranslationResult::Valid(translation_result)
    }
}

async fn get_results(document: &Html) -> Vec<Vec<String>> {
    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_with_a_selector = Selector::parse("td > a").unwrap();

    document
        .select(&table_selector)
        .map(|table| {
            table
                .select(&tr_selector)
                .filter(|tr| tr.select(&td_with_a_selector).count() > 0)
                .map(|tr| tr.text().collect::<Vec<_>>().join(""))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>()
}

async fn get_suggestions(document: &Html) -> Vec<String> {
    let selector = Selector::parse("ul.suggestion-list > li > a").unwrap();
    document
        .select(&selector)
        .map(|li| li.text().collect::<Vec<_>>().join(""))
        .collect::<Vec<_>>()
}
