use anyhow::Result;
use tokio;
extern crate web_crawler;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    start_url: String,

    #[arg(short = 'd', long = "depth")]
    max_depth: Option<u32>,

    #[arg(short = 'l', long = "links")]
    max_links: Option<u32>,

    #[arg(short = 'u', long = "agent")]
    user_agent: Option<String>,

    #[arg(short = 'm', long = "concurrency")]
    max_concurrent_requests: Option<u32>
}

#[tokio::main]
async fn main() -> Result<()> {

    let cli = Cli::parse();

    let options = web_crawler::CrawlerConfig {
        start_url: cli.start_url.clone(),
        max_depth: cli.max_depth.unwrap_or(10),
        max_links: cli.max_links.unwrap_or(5),
        user_agent: cli.user_agent.unwrap_or("WebCrawler".to_owned()),
        max_concurrent_requests: cli.max_concurrent_requests.unwrap_or(10),
    };

    let mut crawler = web_crawler::WebCrawler::new(options);
    crawler.start().await?;
    println!("crawled {} pages", crawler.pages().len());
    for page in crawler.pages() {
        println!("{}\n{}", page.url.url.as_ref(), page.content);
    }

    Ok(())
}
