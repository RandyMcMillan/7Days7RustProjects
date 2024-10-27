use reqwest;
use scraper::{Html, Selector};
use crate::models::Article;
 
pub async fn scrape_articles() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://example.com").await?.text().await?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("article").unwrap();
    let articles = document.select(&selector).filter_map(|article| {
        let title = article.select(&Selector::parse("h2").unwrap()).next()?.text().collect::<String>();
        let link = article.select(&Selector::parse("a").unwrap()).next()?.value().attr("href")?.to_string();
        Some(Article { title, link })
    }).collect();
    Ok(articles)
}