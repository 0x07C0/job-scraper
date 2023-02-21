use std::borrow::Cow;

use anyhow::{Ok, Result};
use voyager::{scraper::Selector, Scraper};

pub struct IndeedScraper {
  job_selector: Selector,
  title_selector: Selector,
  url_selector: Selector,
  description_selector: Selector
}

impl Default for IndeedScraper {
  fn default() -> Self {
    Self {
      job_selector: Selector::parse(".jobsearch-ResultsList li").unwrap(),
      title_selector: Selector::parse(".jcs-JobTitle").unwrap(),
      url_selector: Selector::parse(".jcs-JobTitle").unwrap(),
      description_selector: Selector::parse(".jobsearch-JobComponent-description").unwrap()
    }
  }
}

#[derive(Debug)]
pub struct IndeedJob {
  pub title: Cow<'static, str>,
  pub url: Cow<'static, str>,
  pub description: Cow<'static, str>
}

#[derive(Debug)]
pub enum IndeedState {
  Search(usize),
  Job(IndeedJob)
}

impl Scraper for IndeedScraper {
  type Output = IndeedJob;
  type State = IndeedState;

  fn scrape(
          &mut self,
          response: voyager::Response<Self::State>,
          crawler: &mut voyager::Crawler<Self>,
      ) -> Result<Option<Self::Output>> {
    let html = response.html();
    if let Some(state) = response.state {
      match state {
        IndeedState::Search(_page) => {
          for job in html.select(&self.job_selector) {
            let title = job.select(&self.title_selector).next().unwrap().text().collect();
            let url = job.select(&self.url_selector).next().unwrap().value().attr("href").unwrap().to_owned();
            let job = IndeedJob {
              title,
              url: url.clone().into(),
              description: "".into()
            };

            crawler.visit_with_state(url, IndeedState::Job(job))
          }
        },
        IndeedState::Job(mut job) => {
          if let Some(desc) = html.select(&self.description_selector).next() {
            job.description = desc.text().collect()
          }
          return Ok(Some(job))
        }
      }
    }
    

    Ok(None)
  }
}