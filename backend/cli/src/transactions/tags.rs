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

async fn add_tag(id: i32, tag: String) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
   
    let old_tags: (String,) = sqlx::query_as("SELECT tags FROM transactions WHERE id = $1;")
        .bind(id)
        .fetch_one(&mut conn).await?;

    let update_tags = format!("{};{}", old_tags.0, tag);

    let _ = sqlx::query("UPDATE transactions SET tags = $1 WHERE id = $2;")
        .bind(update_tags)
        .bind(id)
        .execute(&mut conn).await?;

    Ok(())
}

async fn remove_tag(id: i32, tag: String) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
   
    let old_tags: (String,) = sqlx::query_as("SELECT tags FROM transactions WHERE id = $1;")
        .bind(id)
        .fetch_one(&mut conn).await?;

    let update_tags: String = old_tags.0
        .split(";")
        .into_iter()
        .filter(|t| *t != tag).collect::<Vec<&str>>().join(";");

    let _ = sqlx::query("UPDATE transactions SET tags = $1 WHERE id = $2;")
        .bind(update_tags)
        .bind(id)
        .execute(&mut conn).await?;

    Ok(())
}

async fn list_tags(id: i32) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
    
    let t = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = $1;")
        .bind(id)
        .fetch_one(&mut conn).await?;
   
    println!("Tags: {}", t.tags.clone().unwrap_or_default());

    Ok(())
}

pub fn f(args: &clap::ArgMatches) {
    match args.subcommand() {
        Some(("add",  args)) => {
            let id = match args.value_of("ID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let tag = match args.value_of("TAG") {
                Some(t) => t.parse::<String>().ok(),
                None => None,
            };
            if id.is_some() && tag.is_some() {
                let db = add_tag(id.unwrap(), tag.unwrap());
                match block_on(db) {
                    Ok(_) => (),
                    Err(error) => panic!("Query error {:?}", error)
                };
            }
        },
        Some(("remove",  args)) => {
            let id = match args.value_of("ID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let tag = match args.value_of("TAG") {
                Some(t) => t.parse::<String>().ok(),
                None => None,
            };
            if id.is_some() && tag.is_some() {
                let db = remove_tag(id.unwrap(), tag.unwrap());
                match block_on(db) {
                    Ok(_) => (),
                    Err(error) => panic!("Query error {:?}", error)
                };
            }
       },
       Some(("list",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => {
                        let db = list_tags(i);
                        block_on(db);
                    },
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
            };
       },
       Some((&_, _)) => print!("No subcommand selected"),
       None => print!("No subcommand selected"),
    }
}

