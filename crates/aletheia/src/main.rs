use csv::Writer;
use scraper::Html;
use scraper::Selector;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::fs::OpenOptions;

async fn test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://www.amazon.com/Gossner-Foods-Milk-32oz-Pack/dp/B00U4D276I/ref=sr_1_73?_encoding=UTF8&c=ts&dchild=1&keywords=Dairy+Milks&qid=1625493404&s=grocery&sr=1-73&ts_id=6520438011");
    let response = reqwest::get(&url).await?.text().await?;
    // println!("body: {:?}", &response);
    let document = Html::parse_document(&response);
    let selector = Selector::parse("#priceblock_ourprice").unwrap();
    for element in document.select(&selector) {
        println!("element: {:?}", element.inner_html())
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles: std::vec::Vec<_> = Vec::new();
    let job = tokio::spawn(async move { test().await });
    handles.push(job);

    let mut results = Vec::new();
    for job in handles {
        results.push(job.await);
    }

    Ok(())
}
