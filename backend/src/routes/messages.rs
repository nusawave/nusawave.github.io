use actix_web::{get, HttpResponse, Responder};
use std::fs;

#[get("/api/messages")]
pub async fn list_messages() -> impl Responder {
    let file_path = "messages.log";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            let lines: Vec<_> = contents
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| serde_json::json!({ "entry": l }))
                .collect();

            HttpResponse::Ok().json(lines)
        }
        Err(_) => HttpResponse::Ok().json(Vec::<String>::new()),
    }
}