use tokio;
use anyhow::{Result, Ok};
extern crate web_crawler;

#[tokio::main]
async fn main() -> Result<()> {
    let options = web_crawler::CrawlerConfig {
        start_url: "https://google.com".to_owned(),
        max_depth: None,
        max_links: None,
        user_agent: None,
        max_concurrent_requests: None
    };
    let mut crawler = web_crawler::WebCrawler::new(options);
    crawler.start().await?;
    println!("crawled {} pages", crawler.pages.len());
    for page in crawler.pages.iter() {
        println!("{}\n{}", page.url.url.as_ref(), page.content);
    }
    Ok(())
}
