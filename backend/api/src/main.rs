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
use serde::{Serialize, Deserialize};

use futures_util::TryFutureExt;

use db::models::transaction::{Transaction, NewCSVTransaction};
use utils::csv::load_csv;

pub type Pool = SqlitePool;

#[get("/transactions")]
async fn get_transactions(
    pool: web::Data<Pool>) -> Result<HttpResponse, actix_web::Error> {
    
    let mut conn = pool.try_acquire().ok_or(actix_web::error::ErrorBadGateway("Cannot connect to DB exhausted"))?;

    let transactions = Transaction::get_all(&mut conn).await
            .map_err(|_|  actix_web::error::ErrorBadGateway("Query"))?;

    Ok(HttpResponse::Ok().json(&transactions))
}

#[get("/transactions/{id}")]
async fn get_transaction_by_id(
    pool: web::Data<Pool>, chunks: web::Path<(u32,)>) -> Result<HttpResponse, actix_web::Error> {
    let chunks = chunks.into_inner();

    let mut conn = pool.try_acquire().ok_or(actix_web::error::ErrorBadGateway("Free connection to DB exhausted"))?;

    let transaction = Transaction::get_by_id(&mut conn, chunks.0)
        .map_err(|_|  actix_web::error::ErrorBadGateway("Query")).await?;

    Ok(HttpResponse::Ok().json(&transaction))
}

#[derive(Serialize, Deserialize)]
pub struct CommentUpdate {
    comment: Option<String>,
}

#[post("/transactions/{id}/comment")]
async fn post_transaction_comment_by_id(
    pool: web::Data<Pool>, chunks: web::Path<(u32,)>, data: web::Json<CommentUpdate>) -> Result<HttpResponse, actix_web::Error> {
    let chunks = chunks.into_inner();

    let mut conn = pool.try_acquire().ok_or(actix_web::error::ErrorBadGateway("Free connection to DB exhausted"))?;

    let transaction = Transaction::set_comment_by_id(&mut conn, chunks.0, &data.comment)
        .map_err(|_|  actix_web::error::ErrorBadGateway("Query")).await?;

    Ok(HttpResponse::Ok().json(&transaction))
}

#[post("/transactions/upload")]
async fn post_csv_with_transactions(
    mut multipart: Multipart, pool: web::Data<Pool>) -> Result<HttpResponse, actix_web::Error> {

    let mut conn: PoolConnection<Sqlite> = pool.try_acquire().ok_or(actix_web::error::ErrorBadGateway("Free connection to DB exhausted"))?;
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
        .max_connections(25)
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
                .service(get_transaction_by_id)
                .service(post_transaction_comment_by_id)
            )
    })
    .bind(("127.0.0.1", 9091))?
    .run()
    .await
}
