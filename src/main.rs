use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::fs;
use std::path;

const FOLDER: &str = "templates";

async fn index() -> impl Responder {
    // Read the content of index.html from the templates folder
    match fs::read_to_string(path::Path::new(FOLDER).join("index.html")) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html")
            .body(content),
        Err(_) => HttpResponse::InternalServerError().body("Error reading file"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
