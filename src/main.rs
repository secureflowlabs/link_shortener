mod db;
mod error;
mod api;

use api::*;
use error::*;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use sqlx::sqlite::SqlitePool;
use crate::db::link::ShortenedURL;

async fn redirect(pool: web::Data<SqlitePool>,
            req: web::Json<ShortenRequest>,) -> impl Responder {
    let result = sqlx::query_scalar::<_, String>(
        r#"SELECT short_url FROM links where original_url = ?"#
    )
        .bind(req.url.clone())
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(target_url) => {
            let header = ("Location", target_url);

            HttpResponse::TemporaryRedirect()
                .append_header(header)
                .finish()
        },
        Err(e) => {
            HttpResponse::NotFound().json(format!("URL not found: {e}"))
        },
    }

}

async fn shorten(
    pool: web::Data<SqlitePool>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {

    let Ok(short_url) = generate_short_url() else {
        return HttpResponse::UnprocessableEntity().json("URL doesn't work")
    };

    let result = sqlx::query_as::<_, ShortenedURL>(
        r#"INSERT INTO urls (original_url, short_url) VALUES (?, ?) RETURNING id, original_url, short_url"#
    )
        .bind(&req.url)
        .bind(short_url)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(row) => HttpResponse::Ok().json(ShortenResponse {
            short_url: row.short_url,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tokio::main]
async fn main() -> LinkShortenerResult<()> {
    dotenv::dotenv().ok();

    let db_path = std::env::var("DATABASE_URL")?;
    // Initialize the SQLite database pool
    let db_pool = SqlitePool::connect(db_path.as_str()).await.unwrap();
    let url = format!("{}:{}", std::env::var("LOCAL_IP")?, std::env::var("LOCAL_PORT")?);

    // Create the `urls` table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS urls (
            id bigint,
            original_url TEXT NOT NULL,
            short_url TEXT NOT NULL
        )
    "#,
    )
        .execute(&db_pool)
        .await
        .unwrap();

    println!("Starting HTTP server");

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
