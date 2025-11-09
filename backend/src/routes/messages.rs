use actix_web::{get, HttpResponse, Responder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::models::contact::ContactMessage;

#[get("/api/messages")]
pub async fn list_messages() -> impl Responder {
    let file_path = "messages.jsonl";

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            return HttpResponse::Ok().json(Vec::<ContactMessage>::new());
        }
    };

    let reader = BufReader::new(file);
    let mut messages = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(msg) = serde_json::from_str::<ContactMessage>(&line) {
                messages.push(msg);
            }
        }
    }

    HttpResponse::Ok().json(messages)
}
