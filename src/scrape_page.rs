use reqwest::Url;

pub struct Page {
    pub url: Url,
    pub words: Vec<String>,
    pub links: Vec<Url>,
}

pub fn scrape_page(body: &str, page_url: Url) -> Page {
    let document = scraper::Html::parse_document(body);
    let div_selector = scraper::Selector::parse("div").unwrap();
    let words = document
        .select(&div_selector)
        .flat_map(|e| e.text())
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.to_string())
        .collect();
    let anchor_selector = scraper::Selector::parse("a").unwrap();
    let links = document
        .select(&anchor_selector)
        .filter_map(|e| e.value().attr("href"))
        .filter(|url| !url.starts_with("#"))
        .map(|url| {
            if url.starts_with("/") {
                page_url.join(url)
            } else {
                reqwest::Url::parse(url)
            }
        })
        .filter_map(|url_result| url_result.ok())
        .collect();
    let url = page_url;
    Page { url, words, links }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_PAGE_URL: &str = "https://en.wikipedia.org/wiki/Library_of_Alexandria";
    static TEST_PAGE_CONTENTS: &str = include_str!("../test_data/Library_of_Alexandria.html");

    #[test]
    fn scraped_page_returns_passed_url() {
        let test_page_url = Url::parse(TEST_PAGE_URL).unwrap();
        let page = scrape_page(TEST_PAGE_CONTENTS, test_page_url.clone());
        assert!(page.url == test_page_url);
    }

    #[test]
    fn scraped_page_returns_words_from_text() {
        let test_page_url = Url::parse(TEST_PAGE_URL).unwrap();
        let words = scrape_page(TEST_PAGE_CONTENTS, test_page_url.clone()).words;
        assert!(words.iter().any(|word| word == "reign"));
        assert!(words.iter().any(|word| word == "dwindled"));
        assert!(words.iter().any(|word| word == "decree"));
        assert!(words.iter().any(|word| word == "Philadelphus"));
        assert!(words.iter().any(|word| word == "Bibliotheca"));
    }

    #[test]
    fn scraped_page_returns_word_from_section_headings() {
        let test_page_url = Url::parse(TEST_PAGE_URL).unwrap();
        let words = scrape_page(TEST_PAGE_CONTENTS, test_page_url.clone()).words;
        assert!(words.iter().any(|word| word == "POLC"));
    }

    #[test]
    fn scraped_page_returns_word_from_link_text() {
        let test_page_url = Url::parse(TEST_PAGE_URL).unwrap();
        let words = scrape_page(TEST_PAGE_CONTENTS, test_page_url.clone()).words;
        assert!(words.iter().any(|word| word == "HCL"));
    }

    #[test]
    fn scraped_page_does_not_return_non_text_words() {
        let test_page_url = Url::parse(TEST_PAGE_URL).unwrap();
        let words = scrape_page(TEST_PAGE_CONTENTS, test_page_url.clone()).words;
        assert!(!words.iter().any(|word| word == "screen"));
        assert!(!words.iter().any(|word| word == "span"));
        assert!(!words.iter().any(|word| word == "Lua"));
        assert!(!words.iter().any(|word| word == "<ul"));
        assert!(!words.iter().any(|word| word == "</nav>"));
    }

    #[ignore] // Text inside a <style> which is inside a <div> is wrongly included
    #[test]
    fn scraped_page_does_not_words_from_style_element() {
        let test_page_url = Url::parse(TEST_PAGE_URL).unwrap();
        let words = scrape_page(TEST_PAGE_CONTENTS, test_page_url.clone()).words;
        assert!(!words
            .iter()
            .any(|word| word == ".hatnote{font-style:italic}.mw-parser-output"));
    }

    #[test]
    fn scraped_page_returns_links_from_page() {
        let test_page_url = Url::parse(TEST_PAGE_URL).unwrap();
        let links = scrape_page(TEST_PAGE_CONTENTS, test_page_url.clone()).links;
        assert!(links.iter().any(|url| url
            == &Url::parse("https://foundation.wikimedia.org/wiki/Privacy_policy").unwrap()));
        assert!(links
            .iter()
            .any(|url| url == &Url::parse("https://en.wikipedia.org/wiki/Help:Contents").unwrap()));
        assert!(links
            .iter()
            .any(|url| url == &Url::parse("https://en.wikipedia.org/wiki/Callimachus").unwrap()));
    }
}
