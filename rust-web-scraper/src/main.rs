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