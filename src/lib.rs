use anyhow::{Result, Ok};
use fetcher::Page;

pub mod fetcher;
pub struct CrawlerConfig {
    pub start_url: String,
    pub max_depth: Option<u32>,
    pub max_links: Option<u32>,
    pub user_agent: Option<String>,
    pub max_concurrent_requests: Option<u32>
}

pub struct WebCrawler {
    config: CrawlerConfig,
    pub pages: Vec<Page>
}

impl WebCrawler {
    pub fn new(config: CrawlerConfig) -> Self {
        WebCrawler { config, pages: vec![] }
    }

    pub async fn start(&mut self) -> Result<()> {
        let start_url = fetcher::URL::new(self.config.start_url.clone(), None)?;
        let page = start_url.fetch().await?;
        self.pages.push(page);
        let len = self.pages.len();
        if let Some(last_page) = self.pages.last_mut() {
            last_page.index = Some(len - 1);
        }
        Ok(())
    }
}