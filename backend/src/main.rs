pub mod routes;
pub mod models;
pub mod utils;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::init_routes;
use utils::logger::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();

    println!("🚀 Connecting to PostgreSQL...");
    let db_pool = connect_db().await;

    println!("🚀 Starting NusaWave API at http://127.0.0.1:8000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(db_pool.clone()))
            .configure(init_routes)
    })
    .bind(("0.0.0.0", std::env::var("PORT").unwrap_or("8000".to_string()).parse::<u16>().unwrap()))?
    .run()
    .await
}
