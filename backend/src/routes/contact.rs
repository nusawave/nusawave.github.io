use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use crate::models::contact::ContactForm;

#[post("/api/contact")]
pub async fn contact(form: web::Json<ContactForm>) -> impl Responder {
    // 🧾 Simpan ke file log
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("messages.log")
    {
        let now = Utc::now().to_rfc3339();
        let _ = writeln!(
            file,
            "[{}] From: {} ({}) | Service: {} | Msg: {}",
            now, form.name, form.email, form.service, form.message
        );
    }

    // 💬 Kirim ke Telegram
    let send_result = send_to_telegram(&form).await;

    match send_result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": format!("Thank you for contacting us, {}. We’ll get back to you soon!☺️", form.name)
        })),
        Err(e) => {
            eprintln!("❌ Failed to send Telegram message: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Failed to send message. Please try again later."
            }))
        }
    }
}

async fn send_to_telegram(form: &ContactForm) -> Result<(), String> {
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| "Missing TELEGRAM_BOT_TOKEN env var".to_string())?;
    let chat_id = std::env::var("TELEGRAM_CHAT_ID")
        .map_err(|_| "Missing TELEGRAM_CHAT_ID env var".to_string())?;

    let text = format!(
        "📩 *New NusaWave Message*\n\n👤 *Name:* {}\n📧 *Email:* {}\n📱 *Phone:* {}\n🏢 *Company:* {}\n⚙️ *Service:* {}\n💬 *Message:* {}",
        form.name, form.email, form.phone, form.company, form.service, form.message
    );

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let response = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "Markdown"
        }))
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Telegram API error {}: {}", status, body));
    }

    Ok(())
}
