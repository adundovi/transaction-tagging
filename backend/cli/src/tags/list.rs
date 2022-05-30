use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, ConnectOptions};
use futures::executor::block_on;
use std::str::FromStr;
use std::fs::File;
use std::error::Error;
use chrono::NaiveDate;

use db::models::tag::Tag;

async fn list_tags() -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
    
    let transactions = sqlx::query_as::<_, Tag>("SELECT * FROM tags;")
        .fetch_all(&mut conn).await?;
    
    for t in transactions.iter() {
        println!("{}\t{}\t{}\t{}",
            t.id,
            t.tag,
            t.description.clone().unwrap_or_default(),
            t.url.clone().unwrap_or_default()
        );
    }

    Ok(())
}

pub fn f(_args: &clap::ArgMatches) {
    let db = list_tags();
    block_on(db);
}
