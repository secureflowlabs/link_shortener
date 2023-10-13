mod db;
mod error;

use error::*;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use sqlx::{sqlite::SqlitePool, Pool};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    short_url: String,
}

fn redirect(pool: web::Data<SqlitePool>,
            req: web::Json<ShortenRequest>,) -> impl Responder {
    "Not implemented"
}

async fn shorten(
    pool: web::Data<SqlitePool>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {
    let result = sqlx::query_as!(
        db::link::ShortenedURL,
        "INSERT INTO urls (original_url, short_url) VALUES (?, ?) RETURNING id, original_url, short_url",
        &req.url,
        generate_short_url()
    )
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(row) => HttpResponse::Ok().json(ShortenResponse {
            short_url: row.short_url,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

fn generate_short_url() -> String {
    // Generate a unique short URL
    // You can use any logic to generate a short URL here
    // For simplicity, we'll use a UUID.
    uuid::Uuid::new_v4().to_string()
}

#[tokio::main]
async fn main() -> LinkShortenerResult<()> {
    dotenv::dotenv().ok();

    let db_path = std::env::var("DB_PATH")?;
    // Initialize the SQLite database pool
    let db_pool = SqlitePool::connect(db_path.as_str()).await.unwrap();
    let url = format!("{}:{}", std::env::var("LOCAL_IP")?, std::env::var("LOCAL_PORT")?);

    // Create the `urls` table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS urls (
            id varchar(10) PRIMARY KEY,
            original_url TEXT NOT NULL
        )
    "#,
    )
        .execute(&db_pool)
        .await
        .unwrap();

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .service(web::resource("/shorten").route(web::post().to(shorten)))
            .service(web::resource("/{short_url}").route(web::get().to(redirect)))
    })
        .bind(url)?
        .run()
        .await?)

}
