use scraper::{Html, Selector};
use std::fmt;

#[derive(Debug)]
pub enum TranslationResult {
    Valid(Vec<Vec<ValidTranslationEntry>>),
    Suggestions(Vec<String>),
    TermNotFound,
}

#[derive(Debug, Default)]
pub struct ValidTranslationEntry {
    pub index: usize,
    pub category: String,
    pub from: String,
    pub to: String,
    pub parts_of_speech: Option<String>,
}

impl fmt::Display for ValidTranslationEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base = format!(
            "{}. {}: {} -> {}",
            self.index, self.category, self.from, self.to
        );
        match self.parts_of_speech {
            Some(ref pos) => write!(f, "{} ({})", base, pos),
            None => write!(f, "{}", base),
        }
    }
}

impl ValidTranslationEntry {
    pub fn new(
        index: usize,
        category: String,
        from: String,
        to: String,
        parts_of_speech: Option<String>,
    ) -> Self {
        Self {
            index,
            category,
            from,
            to,
            parts_of_speech,
        }
    }
}

pub async fn parse_html_content(content: &str) -> TranslationResult {
    let document = Html::parse_document(content);
    let translation_result = get_results(&document);

    if translation_result.is_empty() {
        let h1_selector = Selector::parse("h1").unwrap();
        let term_not_found_h1_exists = document.select(&h1_selector).any(|h1| {
            "term not found" == h1.text().collect::<Vec<_>>().join("").trim().to_lowercase()
        });

        match term_not_found_h1_exists {
            true => TranslationResult::TermNotFound,
            _ => TranslationResult::Suggestions(get_suggestions(&document)),
        }
    } else {
        TranslationResult::Valid(translation_result)
    }
}

fn get_results(document: &Html) -> Vec<Vec<ValidTranslationEntry>> {
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
                    let mut entry_pos: Option<String> = None;
                    let single_result_vec = tr
                        .select(&td_selector)
                        .map(|td| get_result_from_td(&td))
                        .map(|(word, pos)| {
                            if let Some(p) = pos {
                                entry_pos = Some(p.trim().to_owned());
                            }
                            word.trim().to_string()
                        })
                        .filter(|td| !td.is_empty())
                        .collect::<Vec<_>>();

                    ValidTranslationEntry::new(
                        single_result_vec[0].parse::<usize>().unwrap(),
                        single_result_vec[1].to_owned(),
                        single_result_vec[2].to_owned(),
                        single_result_vec[3].to_owned(),
                        entry_pos,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_suggestions(document: &Html) -> Vec<String> {
    let selector = Selector::parse("ul.suggestion-list > li > a").unwrap();
    document
        .select(&selector)
        .map(|li| li.text().collect::<Vec<_>>().join(""))
        .collect::<Vec<_>>()
}

fn get_result_from_td(td: &scraper::ElementRef) -> (String, Option<String>) {
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
        (word, Some(pos))
    } else {
        (td.text().collect::<Vec<_>>().join(""), None)
    }
}
