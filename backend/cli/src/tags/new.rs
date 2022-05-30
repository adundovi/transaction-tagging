use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, ConnectOptions};
use futures::executor::block_on;
use std::str::FromStr;
use std::fs::File;
use std::error::Error;
use chrono::NaiveDate;
use futures::TryStreamExt;
use sqlx::Row;

use db::models::transaction::Transaction;
use db::models::tag::Tag;

async fn new_tag(tag: String) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
   
    let _ = sqlx::query("INSERT OR IGNORE INTO tags
                (
                    tag,
                    description,
                    url
                )
                VALUES
                    ($1, '', '')")
            .bind(&tag)
            .execute(&mut conn).await?;

    Ok(())
}

pub fn f(args: &clap::ArgMatches) {
    let name = match args.value_of("NAME") {
        Some(t) => t.parse::<String>().ok(),
        None => None,
    };
    if name.is_some() {
        let db = new_tag(name.unwrap());
        match block_on(db) {
            Ok(_) => (),
            Err(error) => panic!("Query error {:?}", error)
        };
    }
}

