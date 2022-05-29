use std::str::FromStr;
use std::fs::{read, File};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Connection, ConnectOptions};
use futures::executor::block_on;

use db::models::transaction::{Transaction, NewCSVTransaction};
use utils::csv::load_csv;

async fn insert_transactions_from_csv(filename: &str) -> Result::<(), sqlx::Error> {
    let file = read(filename)?;
    let new_transactions: Vec<NewCSVTransaction> = load_csv(&file).unwrap();

    let mut pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite://sqlite.db").await?;
    let mut conn = pool.acquire().await?;
    
    for t in new_transactions.iter() {
        Transaction::insert(&mut conn, &t);
    }

    Ok(())
}

pub fn f(args: &clap::ArgMatches) {
    match args.value_of("FILE") {
        Some(f) => {
            let db = insert_transactions_from_csv(f);
            block_on(db);
        },
        None => { }
    }
}
