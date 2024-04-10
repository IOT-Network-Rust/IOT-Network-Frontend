use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use std::fs;
use std::path;

const FOLDER: &str = "templates";

const BACKEND: &str = "";

pub async fn data() -> impl Responder {
    // Read the content of data.html from the templates folder
    match fs::read_to_string(path::Path::new(FOLDER).join("data.html")) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html")
            .body(content),
        Err(_) => HttpResponse::InternalServerError().body("Error reading file"),
    }
}

pub async fn fetch_device(req: HttpRequest) -> impl Responder {
    let query_device_name = req.query_string().ge("device_name");
    match 7 {
        _ => HttpResponse::InternalServerError().body("Error reading file"),
    }
}

pub async fn fetch_device_names() -> impl Responder {
    let devices: Option<Vec<&str>> = Some(vec!["one", "two", "three"]);
    match devices {
        Some(data) => HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("{:?}", data)),
        None => HttpResponse::InternalServerError().body("Error reading file"),
    }
}
