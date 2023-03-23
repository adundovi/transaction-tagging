use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, ConnectOptions};
use futures::executor::block_on;
use std::str::FromStr;
use std::fs::File;
use std::error::Error;
use chrono::NaiveDate;

use db::models::transaction::Transaction;

async fn list_transactions() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
    
    let transactions = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions;")
        .fetch_all(&mut conn).await?;
    
    for t in transactions.iter() {
        println!("{}\t{}\t{}\t{}",
            t.id,
            t.transaction_reference,
            t.value_date,
            t.sender_receiver_name.clone().unwrap_or_default()
        );
    }

    Ok(())
}

pub fn f(_args: &clap::ArgMatches) {
    let db = list_transactions();
    block_on(db);
}
