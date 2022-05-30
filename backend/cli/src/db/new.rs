use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};
use sqlx::Connection;
use sqlx::ConnectOptions;
use std::str::FromStr;
use futures::executor::block_on;

async fn create() -> Result::<SqliteConnection, sqlx::Error> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .create_if_missing(true)
        .connect().await?;

    // YYYY-MM-DD HH:MM:SS.SSS
    sqlx::query("CREATE TABLE IF NOT EXISTS transactions (
                id              INTEGER PRIMARY KEY NOT NULL,
                value_date      TEXT NOT NULL,
                execution_date  TEXT NOT NULL,
                description     TEXT,
                iban_sender     TEXT NOT NULL,
                send_amount     REAL,
                receive_amount  REAL,
                account_balance REALO NOT NULL,
                sender_reference_number TEXT,
                receiver_reference_number   TEXT,
                sender_receiver_name    TEXT,
                sender_receiver_place   TEXT,
                transaction_reference   TEXT NOT NULL,
                comment         TEXT,
                url             TEXT,
                UNIQUE(transaction_reference, iban_sender)
            )").execute(&mut conn).await?;
    
    sqlx::query("CREATE TABLE IF NOT EXISTS tags (
                id              INTEGER PRIMARY KEY NOT NULL,
                tag             TEXT NOT NULL,
                description     TEXT,
                url             TEXT,
                UNIQUE(tag)
            )").execute(&mut conn).await?;
    
    sqlx::query("CREATE TABLE IF NOT EXISTS trans_tags_relations (
                id              INTEGER PRIMARY KEY NOT NULL,
                transaction_id  INTEGER REFERENCES transactions(id),
                tag_id          INTEGER REFERENCES tags(id),
                UNIQUE(transaction_id, tag_id)
            )").execute(&mut conn).await?;

    Ok(conn)
}

pub fn f(args: &clap::ArgMatches) {
    let db = create();
    block_on(db);
}
