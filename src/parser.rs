use scraper::{Html, Selector};
use std::fmt;

#[derive(Debug)]
pub enum TranslationResult {
    Valid(Vec<Vec<ValidTranslationEntry>>),
    Suggestions(Vec<String>),
    TermNotFound,
}

impl TranslationResult {
    pub fn display(&self) {
        match self {
            TranslationResult::Valid(results) => {
                for result in results {
                    for entry in result {
                        println!("{}", entry);
                    }
                    println!();
                }
            }
            TranslationResult::Suggestions(suggestions) => {
                println!("Suggestions:");
                for (i, s) in suggestions.iter().enumerate() {
                    println!("{:2}. {}", i + 1, s);
                }
            }
            TranslationResult::TermNotFound => {
                println!("Term not found");
            }
        }
    }
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
            "{:3}. {}: {} -> {}",
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
    let i_selector = Selector::parse("i").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    document
        .select(&table_selector)
        .map(|table| {
            table
                .select(&tr_selector)
                .filter(|tr| tr.select(&td_with_a_selector).count() > 0)
                .filter_map(|tr| get_result_from_tr(&tr, &td_selector, &i_selector, &a_selector))
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

fn get_result_from_tr(
    tr: &scraper::ElementRef,
    td_selector: &Selector,
    i_selector: &Selector,
    a_selector: &Selector,
) -> Option<ValidTranslationEntry> {
    let mut entry_pos: Option<String> = None;
    let single_result_vec = tr
        .select(td_selector)
        .map(|td| get_result_from_td(&td, i_selector, a_selector))
        .map(|(word, pos)| {
            if let Some(p) = pos {
                entry_pos = Some(p.trim().to_owned());
            }
            word.trim().to_string()
        })
        .filter(|td| !td.is_empty())
        .collect::<Vec<_>>();

    match single_result_vec[0].parse::<usize>() {
        Ok(index) => Some(ValidTranslationEntry::new(
            index,
            single_result_vec[1].to_owned(),
            single_result_vec[2].to_owned(),
            single_result_vec[3].to_owned(),
            entry_pos,
        )),
        Err(_) => None,
    }
}

fn get_result_from_td(
    td: &scraper::ElementRef,
    i_selector: &Selector,
    a_selector: &Selector,
) -> (String, Option<String>) {
    let inner_html = td.inner_html();
    if inner_html.contains("<i>") {
        let pos = td
            .select(i_selector)
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
            .select(a_selector)
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

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_TABLE_STR: &str = r#"
        <h1>test</h1>
        <table class="table table-hover table-striped searchResultsTable" id="englishResultsTable">
            <tbody>
                <tr>
                    <td class="rc0 hidden-xs">1</td>
                    <td class="hidden-xs">Common Usage</td>
                    <td class="tr ts" lang="tr"><a href="/en/turkish-english/kedi">kedi</a></td>
                    <td class="en tm" lang="en"><a href="/en/turkish-english/cat">cat</a><i>n. </i></td>
                    <td class="rc4 hidden-xs"><span class="glyphicon glyphicon-option-horizontal"></span></td>
                </tr>
            </tbody>
        </table>
        "#;

    #[test]
    fn test_get_result_from_tr() {
        let table_tr_selector = Selector::parse("table").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let i_selector = Selector::parse("i").unwrap();
        let a_selector = Selector::parse("a").unwrap();

        let table_html = Html::parse_document(SAMPLE_TABLE_STR);
        let sample_tr = table_html
            .select(&table_tr_selector)
            .map(|table| table.select(&tr_selector).next().unwrap())
            .next()
            .unwrap();

        let entry = get_result_from_tr(&sample_tr, &td_selector, &i_selector, &a_selector).unwrap();
        assert_eq!(entry.index, 1);
        assert_eq!(entry.from, "kedi".to_string());
        assert_eq!(entry.to, "cat".to_string());
        assert_eq!(entry.parts_of_speech, Some("n.".to_string()));
    }
}
