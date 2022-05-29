use actix_web::{
    middleware,
    get, post, web,
    App, HttpResponse, HttpServer
};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use futures_util::TryStreamExt;
use sqlx::{
    pool::PoolConnection,
    Sqlite,
    sqlite::{
        SqlitePool,
        SqlitePoolOptions,
    },
};

use db::models::transaction::{Transaction, NewCSVTransaction};
use utils::csv::load_csv;

pub type Pool = SqlitePool;

#[get("/transactions")]
async fn get_transactions(
    pool: web::Data<Pool>) -> Result<HttpResponse, actix_web::Error> {
    
    let conn = pool.get_ref();

    let transactions = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions ORDER BY value_date DESC, id DESC;"
        )
        .fetch_all(conn).await
        .map_err(|_|  actix_web::error::ErrorBadGateway("Query"))?;

    Ok(HttpResponse::Ok().json(&transactions))
}

#[post("/transactions/upload")]
async fn post_csv_with_transactions(
    mut multipart: Multipart, pool: web::Data<Pool>) -> Result<HttpResponse, actix_web::Error> {

    let mut conn: PoolConnection<Sqlite> = pool.try_acquire().ok_or(actix_web::error::ErrorBadGateway("Query"))?;
    let mut bytes = web::BytesMut::new();

    while let Some(mut field) = multipart.try_next().await? {
        let content_disposition = field.content_disposition();
        let _field_name = content_disposition.get_name().unwrap(); // csvFile
    
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            bytes.extend_from_slice(&data);
        }
    }
    
    let new_transactions: Vec<NewCSVTransaction> = load_csv(&bytes).unwrap();
    for t in new_transactions.iter() {
        Transaction::insert(&mut conn, &t).await.map_err(|_| actix_web::error::ErrorBadGateway("Error inserting"))?;
    }

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://sqlite.db").await
        .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::Other, "Cannot connect to DB"))?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                .service(get_transactions)
                .service(post_csv_with_transactions)
            )
    })
    .bind(("127.0.0.1", 9091))?
    .run()
    .await
}
