use actix_web::{Responder, HttpResponse};
use tera::{Tera, Context};
use lazy_static::lazy_static;


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

pub async fn index() -> impl Responder {
    let context = Context::new();
    let string = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok()
    .content_type("text/html")
    .body(string)
}

pub async fn tracker() -> impl Responder {
    let context = Context::new();
    let string = TEMPLATES.render("tracker.html", &context).unwrap();
    HttpResponse::Ok()
    .content_type("text/html")
    .body(string)
}

pub async fn credits() -> impl Responder{
    let context = Context::new();
    let string = TEMPLATES.render("credits.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(string)
}