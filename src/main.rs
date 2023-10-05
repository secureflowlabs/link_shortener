pub mod api;
pub mod db;
pub mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Ok(())
}
