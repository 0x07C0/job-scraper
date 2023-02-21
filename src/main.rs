use futures::StreamExt;
use job_scraper::{IndeedScraper, IndeedState};
use voyager::{CrawlerConfig, Collector};

#[tokio::main]
async fn main() {
  let conf = CrawlerConfig::default().allow_domain("ua.indeed.com");
  let mut collector = Collector::new(IndeedScraper::default(), conf);
  collector.crawler_mut().visit_with_state("https://ua.indeed.com/jobs?q=rust", IndeedState::Search(0));
  while let Some(res) = collector.next().await {
    if let Ok(job) = res {
      println!(
        "Job found:\nTitle: {}\nLink: {}\nDescription: {}\n",
        job.title,
        job.url,
        job.description
      );
    }
  }
}
