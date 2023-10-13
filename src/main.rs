use crate::db::DbResult;
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::http::Client;
use surrealdb::engine::remote::http::Http;

pub mod api;
pub mod db;
pub mod error;

#[tokio::main]
async fn main() -> DbResult<()> {
    static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

    DB.connect::<Http>("localhost:8000").await?;
    /*
    dotenv::dotenv().ok();
    println!("loaded env");

    let host = std::env::var("ADDR")?;
    let port = std::env::var("PORT")?;
    let db_url = format!("{host}:{port}");

    println!("created DB_URL: {db_url}");

    let target = url::Url::parse("https://orf.at")?;
    println!("created target URL: {target}");
    let meta = LinkMeta::new(HashMap::new()).clone();
    println!("created link meta: {:?}", meta.clone());
    let link = Link::new(target, meta)?;
    println!("created link: {:?}", link.clone());
    let db = Surreal::new::<Ws>(db_url).await?;
    println!("created db connection: {:?}", db.clone());
    let id = uuid::Uuid::new_v4().to_string();
    println!("created document ID: {}", id);
    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    println!("Setup done, signed in as root");

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;

    println!("Creating link");

    let created_link: Vec<Link> = db.create(id)
        .content(link).await?;

    dbg!(created_link);
    println!("FINISHED");
*/
    Ok(())
}
