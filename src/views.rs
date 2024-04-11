use actix_web::{Responder, HttpResponse};
use tera::{Tera, Context};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::io;
use std::process;

lazy_static! {
    pub static ref TEMPLATES: Mutex<Tera> = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        
        Mutex::new(tera)
    };
}

pub async fn index() -> impl Responder {
    let context = Context::new();
    let string = TEMPLATES.lock().unwrap().render("index.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(string)
}

pub async fn tracker() -> impl Responder {
    let context = Context::new();
    let string = TEMPLATES.lock().unwrap().render("tracker.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(string)
}

pub async fn node() -> impl Responder {
    let context = Context::new();
    let string = TEMPLATES.lock().unwrap().render("node.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(string)
}

pub async fn nodes() -> impl Responder {
    let context = Context::new();
    let string = TEMPLATES.lock().unwrap().render("nodes.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(string)
}

pub async fn credits() -> impl Responder {
    let context = Context::new();
    let string = TEMPLATES.lock().unwrap().render("credits.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(string)
}

pub fn reload() {
    println!("To Exit Enter QUIT:");
    loop {
        println!("Press Enter To Reload Templates:");
        // Create a mutable string to store user input
        let mut input = String::new();

        // Read user input from the command line
        match io::stdin().read_line(&mut input) {
            Ok(_) => {    
                let input = input.trim().to_ascii_uppercase();
                if input == "QUIT" {
                    process::exit(0);
                } else {
                    TEMPLATES.lock().unwrap().full_reload();
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}