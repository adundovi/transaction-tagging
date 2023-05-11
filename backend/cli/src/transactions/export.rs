use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, ConnectOptions};
use futures::executor::block_on;
use std::str::FromStr;
use std::fs::File;
use std::io;
use std::error::Error;
use chrono::NaiveDate;

use db::models::transaction::Transaction;

fn csv_export(transactions: Vec<Transaction>) -> Result<(), Box<dyn Error>> {
    let mut wtr =
        csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_writer(io::stdout());

    for t in transactions.iter() {
        wtr.serialize(t);
    }

    Ok(())
}

async fn export_transactions() -> Result<(), Box<dyn Error>> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
    
    let transactions = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions;")
        .fetch_all(&mut conn).await?;
    
    csv_export(transactions)
}

pub fn f(_args: &clap::ArgMatches) {
    let db = export_transactions();
    block_on(db);
}
