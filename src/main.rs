use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    chat_ids: Vec<String>,
    content: String,
}

/// extract `Info` using serde
fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("{} {}", info.content,info.chat_ids[0]))
}

fn main() {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}