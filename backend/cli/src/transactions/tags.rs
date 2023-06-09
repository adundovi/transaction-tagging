use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, ConnectOptions};
use sqlx::sqlite::SqliteRow;
use futures::executor::block_on;
use std::str::FromStr;
use std::fs::File;
use std::error::Error;
use chrono::NaiveDate;
use futures::TryStreamExt;
use sqlx::Row;

use db::models::transaction::Transaction;
use db::models::tag::Tag;

async fn add_tag(id: i32, tag: String) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
        
    let tag_id: i32 = 
        sqlx::query("SELECT * FROM tags WHERE tag = $1")
        .bind(&tag)
        .map(|row: SqliteRow| {
            row.try_get("id")
        })
        .fetch_one(&mut conn).await??;

    let _ = sqlx::query("INSERT OR IGNORE INTO trans_tags_relations
                (transaction_id, tag_id) VALUES ($1, $2)")
            .bind(id)
            .bind(tag_id)
            .execute(&mut conn).await?;
   
    Ok(())
}

async fn remove_tag(id: i32, tag: String) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
    
    let tag_id: i32 = 
        sqlx::query("SELECT * FROM tags WHERE tag = $1")
        .bind(&tag)
        .map(|row: SqliteRow| {
            row.try_get("id")
        })
        .fetch_one(&mut conn).await??;
   
    let _ = sqlx::query("DELETE FROM trans_tags_relations WHERE transaction_id = $1 AND tag_id = $2")
        .bind(id)
        .bind(tag_id)
        .execute(&mut conn).await?;

    Ok(())
}

async fn list_tags(id: i32) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
   
    #[derive(sqlx::FromRow)]
    struct Tag { tag: String }

    let tags = sqlx::query_as::<_, Tag>("
            SELECT t.tag FROM transactions as tr
                INNER JOIN trans_tags_relations as r
                    ON r.transaction_id = tr.id
                INNER JOIN tags as t
                    ON t.id == r.tag_id WHERE tr.id = $1")
        .bind(id)
        .fetch_all(&mut conn).await?;
  
    println!("Tags:");
    for t in tags {
        println!("\t{}", t.tag.clone());
    }

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

