use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};
use sqlx::Connection;
use sqlx::ConnectOptions;
use std::str::FromStr;
use futures::executor::block_on;

use db::models::transaction::Transaction;

async fn convert_from_hrk_to_eur() -> Result::<(), sqlx::Error> {

    let mut pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite://sqlite.db").await?;
    let mut conn = pool.acquire().await?;
    
    let transactions = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions;")
        .fetch_all(&mut conn).await?;
    

    for t in transactions.iter() {
        let t.send_amount;
    }

    Ok(())
}

pub fn f(args: &clap::ArgMatches) {
    let db = convert_from_hrk_to_eur();
    block_on(db);
}
