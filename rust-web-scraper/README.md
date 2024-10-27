#### Introduction
Today, we're going to build a web scraper using Rust with the Actix Web framework to fetch data from websites. We'll create an endpoint that, when hit, scrapes a predefined website for information and returns it in JSON format. This project will introduce you to asynchronous programming, HTTP requests, and HTML parsing in Rust.

#### Difficulty
🍂 **Intermediate**

#### Prerequisites
- Basic understanding of Rust
- Familiarity with HTTP requests
- Concept of asynchronous programming

#### Project Structure
Let's set up our project:

```sh
mkdir rust-web-scraper
cd rust-web-scraper
cargo init --lib
cargo add actix-web reqwest scraper tokio serde serde_json
```

Our folder structure:

```
rust-web-scraper/
│
├── src/
│   ├── main.rs
│   ├── scraper.rs
│   └── models.rs
│
├── Cargo.toml
└── README.md
```

#### Step 1: Setting up `Cargo.toml`

```toml
[package]
name = "web_scraper"
version = "0.1.0"
edition = "2018"

[dependencies]
actix-web = "4.0.0-beta.9"
reqwest = { version = "0.11", features = ["json"] }
scraper = "0.12"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### Step 2: `models.rs` - Define the structure of our scraped data

```rust
use serde::Serialize;

#[derive(Serialize)]
pub struct Article {
    pub title: String,
    pub link: String,
}
```

#### Step 3: `scraper.rs` - Implement the scraping logic

```rust
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
```

#### Step 4: `main.rs` - Set up the web server and route

```rust
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde_json::json;
mod scraper;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_articles))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_articles() -> impl Responder {
    match scraper::scrape_articles().await {
        Ok(articles) => HttpResponse::Ok().json(json!({ "articles": articles })),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve articles"),
    }
}
```

#### Step 5: Usage

To run your scraper:

```sh
cargo run
```

Then, in your web browser, navigate to `http://127.0.0.1:8080/` or use any HTTP client like `curl` to get the scraped data:

```sh
curl http://127.0.0.1:8080/
```

#### Explanation

- **Dependencies**: We use `actix-web` for the web server, `reqwest` for making HTTP requests, `scraper` for parsing HTML, `tokio` for async runtime, and `serde` for JSON serialization.
- **Scraping Logic**: The `scrape_articles` function fetches the page content, parses it into an HTML document, and extracts article titles and links using CSS selectors.
- **Web Server**: We set up an Actix Web server that listens on port 8080 for GET requests to the root path, which triggers the scraping and returns JSON.

#### Conclusion

This project teaches you how to create a web service that can scrape data from websites, handle asynchronous operations, and serve JSON data. It's a fantastic way to learn about Rust's concurrency model and web development capabilities:

- **Extend the Project**: You can enhance it by adding more complex scraping logic, error handling, or by making the site to scrape configurable.
- **Security Considerations**: Always respect the terms of service of websites you're scraping, implement rate limiting, and avoid overloading the target server.

By following this guide, you've not only built a functional web scraper but also gained insights into Rust's ecosystem for web technologies.