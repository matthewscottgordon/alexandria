mod scrape_page;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let base_url = reqwest::Url::parse(&std::env::args().skip(1).next().unwrap()).unwrap();
    let response = client.get(base_url.clone()).send().await?;
    if response.status() == 200 {
        let page = scrape_page::scrape_page(&response.text().await?, base_url);
        for word in page.words {
            print!("{} ", word);
        }
        for link in page.links {
            println!("{}", link);
        }
    }
    Ok(())
}
