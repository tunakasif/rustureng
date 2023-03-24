use scraper::{Html, Selector};

#[derive(Debug)]
pub enum TranslationResult {
    Valid(Vec<Vec<Vec<String>>>),
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

async fn get_results(document: &Html) -> Vec<Vec<Vec<String>>> {
    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let td_with_a_selector = Selector::parse("td > a").unwrap();

    document
        .select(&table_selector)
        .map(|table| {
            table
                .select(&tr_selector)
                .filter(|tr| tr.select(&td_with_a_selector).count() > 0)
                .map(|tr| {
                    tr.select(&td_selector)
                        .map(|td| get_result_from_td(&td))
                        .map(|x| x.trim().to_string())
                        .filter(|td| !td.is_empty())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
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

fn get_result_from_td(td: &scraper::ElementRef) -> String {
    let i_selector = Selector::parse("i").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let inner_html = td.inner_html();

    if inner_html.contains("<i>") {
        let pos = td
            .select(&i_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");

        let inner_html_without_pos = match inner_html.rfind("<i>") {
            Some(index) => inner_html[..index].trim().to_string(),
            None => inner_html,
        };

        let word = Html::parse_fragment(&inner_html_without_pos)
            .select(&a_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        vec![word, pos].join(", ")
    } else {
        td.text().collect::<Vec<_>>().join("")
    }
}
