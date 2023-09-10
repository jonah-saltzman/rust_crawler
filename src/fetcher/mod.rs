use reqwest;
use url::{ParseError, Url};
use anyhow::{Result, Context};
pub struct URL {
    pub url: Url,
    pub(crate) parent_index: Option<usize>,
}

impl URL {
    pub fn new(url: String, parent: Option<&Page>) -> Result<Self> {
        match Url::parse(&url) {
            Ok(parsed_url) => Ok(URL {
                url: parsed_url,
                parent_index: parent.map_or(None, |parent_page| parent_page.index)
            }),
            Err(ParseError::RelativeUrlWithoutBase) => {
                if let Some(parent_page) = parent {
                    parent_page.url.url.join(&url).map(|joined_url| URL {
                        url: joined_url,
                        parent_index: parent_page.index
                    }).context("Failed to join urls")
                } else {
                    Err(anyhow::anyhow!(ParseError::RelativeUrlWithoutBase))
                }
            }
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    pub async fn fetch(self) -> Result<Page> {
        let response_content = reqwest::get(self.url.as_str()).await?.text().await?;
        let parent_index = self.parent_index;
        Ok(Page {
            url: self,
            content: response_content,
            referrer_index: parent_index,
            index: None,
            links: vec![],
        })
    }
}

pub struct Page {
    pub url: URL,
    pub content: String,
    pub(crate) referrer_index: Option<usize>,
    pub(crate) index: Option<usize>,
    pub(crate) links: Vec<usize>,
}
