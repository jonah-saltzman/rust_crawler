use anyhow::{Result, Ok};
use fetcher::Page;

pub mod fetcher;
pub struct CrawlerConfig {
    pub start_url: String,
    pub max_depth: u32,
    pub max_links: u32,
    pub user_agent: String,
    pub max_concurrent_requests: u32
}

pub struct WebCrawler {
    config: CrawlerConfig,
    pages: Vec<Page>
}

impl WebCrawler {
    pub fn new(config: CrawlerConfig) -> Self {
        WebCrawler { config, pages: vec![] }
    }

    pub async fn start(&mut self) -> Result<()> {
        let start_url = fetcher::URL::new(self.config.start_url.clone(), None)?;
        let page = start_url.fetch().await?;
        self.pages.push(page);
        if let Some(last_page) = self.pages.last_mut() {
            last_page.index = Some(0);
        }
        Ok(())
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}