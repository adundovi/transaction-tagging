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

async fn new_comment(id: i32, comment: String) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
        
    let _ = sqlx::query("UPDATE transactions SET comment = $1 WHERE id = $2")
            .bind(comment)
            .bind(id)
            .execute(&mut conn).await?;
   
    Ok(())
}

async fn remove_comment(id: i32, tag: String) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
    
    let tag_id: i32 = 
        sqlx::query("SELECT * FROM tags WHERE tag = $1")
        .bind(&tag)
        .map(|row: SqliteRow| {
            row.try_get("id")
        })
        .fetch_one(&mut conn).await??;
   
    let _ = sqlx::query("DELETE FROM trans_comments_relations WHERE transaction_id = $1 AND tag_id = $2")
        .bind(id)
        .bind(tag_id)
        .execute(&mut conn).await?;

    Ok(())
}

async fn list_comments(id: i32) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
   
    let comment: String = sqlx::query("SELECT comment FROM transactions WHERE id = $1")
        .bind(id)
        .map(|row: SqliteRow| {
            row.try_get("comment")
        })
        .fetch_one(&mut conn).await??;
  
    println!("{}", comment.clone());

    Ok(())
}

pub fn f(args: &clap::ArgMatches) {
    match args.subcommand() {
        Some(("new",  args)) => {
            let id = match args.value_of("ID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let comment = match args.value_of("COMMENT") {
                Some(t) => t.parse::<String>().ok(),
                None => None,
            };
            if id.is_some() && comment.is_some() {
                let db = new_comment(id.unwrap(), comment.unwrap());
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
                let db = remove_comment(id.unwrap(), tag.unwrap());
                match block_on(db) {
                    Ok(_) => (),
                    Err(error) => panic!("Query error {:?}", error)
                };
            }
       },
       Some(("show",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => {
                        let db = list_comments(i);
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

