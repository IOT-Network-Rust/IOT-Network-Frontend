use actix_web::error::HttpError;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_files::NamedFile;
use std::fs;
use std::future::IntoFuture;
use std::path::{self, PathBuf};
use reqwest;

const FOLDER: &str = "templates";
const BACKEND: &str = "127.0.0.1:8080";

pub async fn data() -> impl Responder {
    // Read the content of data.html from the templates folder
    match fs::read_to_string(path::Path::new(FOLDER).join("data.html")) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html")
            .body(content),
        Err(_) => HttpResponse::InternalServerError().body("Error reading file"),
    }
}

pub async fn fetch_device_data(req: HttpRequest) -> impl Responder {
    let device_id: String = req.match_info().query("device_id").parse().unwrap();
    let url = format!("http://{}/files/{}.db", BACKEND, device_id);
    println!("{}", url);
    match reqwest::get(url).await {
        Ok(response) => {
            let data = response.text().await.unwrap();
            HttpResponse::Ok()
                        .content_type("text/html")
                        .body(data)
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Error reading file")
        }
    }
}

pub async fn fetch_device_catalog() -> impl Responder {
    let url = format!("http://{}/files/device_catalog.db", BACKEND);

    match reqwest::get(url).await {
        Ok(response) => {
            let data = response.text().await.unwrap();
            HttpResponse::Ok()
                        .content_type("text/html")
                        .body(data)
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Error reading file")
        }
    }
}
