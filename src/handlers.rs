use actix_web::{HttpResponse, Responder, web};
use sqlx::SqlitePool;
use crate::api::{generate_short_url, ShortenRequest, ShortenResponse};
use crate::db::{GET_REDIRECT_URL, INSERT_LINK};
use crate::db::link::ShortenedURL;

pub async fn redirect(pool: web::Data<SqlitePool>,
                  req: web::Path<String>,) -> impl Responder {
    tracing::info!("Call to redirect received");

    let data = req.into_inner();
    let result = sqlx::query_scalar::<_, String>(GET_REDIRECT_URL)
        .bind(data)
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

pub async fn shorten(
    pool: web::Data<SqlitePool>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {

    let Ok(short_url) = generate_short_url() else {
        return HttpResponse::UnprocessableEntity().json("URL doesn't work")
    };

    let result = sqlx::query_as::<_, ShortenedURL>(INSERT_LINK)
        .bind(short_url)
        .bind(&req.url)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(row) => HttpResponse::Ok().json(ShortenResponse {
            short_url: row.id,
        }),
        Err(e) => {
            tracing::error!("Can't insert URL: {e}");
            HttpResponse::InternalServerError().finish()
        },
    }
}
