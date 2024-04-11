#![allow(unused)]
#![allow(unused_imports)]


use actix_web::{web, App, HttpServer};
mod data;
mod views;
use std::thread;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    thread::spawn(|| {views::reload()});

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(views::index))
            .route("/tracker", web::get().to(views::tracker))
            .route("/tracker/node", web::get().to(views::node))
            .route("/tracker/nodes", web::get().to(views::nodes))

            .route("/credits", web::get().to(views::credits))

            .route("/data/", web::get().to(data::fetch_device_names))
            .route("/data/get_device", web::get().to(data::data))
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
