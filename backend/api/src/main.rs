use actix_web::{
    middleware,
    get, post, web,
    App, HttpResponse, HttpServer, Responder
};
use sqlx::{
    sqlite::{
        SqlitePool,
        SqlitePoolOptions,
    },
};

use db::models::transaction::Transaction;

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
            )
    })
    .bind(("127.0.0.1", 9091))?
    .run()
    .await
}
