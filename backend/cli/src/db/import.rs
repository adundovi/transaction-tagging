use std::str::FromStr;
use std::fs::File;
use std::error::Error;
use chrono::NaiveDate;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, ConnectOptions, Executor};
use futures::executor::block_on;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use utils::{date_serializer, currency_serializer_option, currency_serializer};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct NewTransaction {
    pub transaction_no: i32,
    #[serde(with = "date_serializer")]
    pub value_date: NaiveDate,
    #[serde(with = "date_serializer")]
    pub execution_date: NaiveDate,
    pub description: Option<String>,
    pub iban_sender: String,
    #[serde(with = "currency_serializer_option")]
    pub send_amount: Option<f64>,
    #[serde(with = "currency_serializer_option")]
    pub receive_amount: Option<f64>,
    #[serde(with = "currency_serializer")]
    pub account_balance: f64,
    pub sender_reference_number: Option<String>,
    pub receiver_reference_number: Option<String>,
    pub sender_receiver_name: Option<String>,
    pub sender_receiver_place: Option<String>,
    pub transaction_reference: String,
}

pub fn load_csv<T: DeserializeOwned>(filename: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let mut items: Vec<T> = Vec::new();

    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b'\t')
                    .has_headers(false)
                    .double_quote(true)
                    .escape(Some(b'\\'))
                    .from_reader(file);
    for record in rdr.deserialize() {
        let p: T = match record {
            Ok(result) => result,
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };
        items.push(p);
    }
    Ok(items)
}

async fn insert_transactions_from_csv(filename: &str) -> Result::<u32, sqlx::Error> {
    let new_transactions: Vec<NewTransaction> = load_csv(filename).unwrap();

    let mut conn = SqliteConnectOptions::from_str("sqlite://sqlite.db")?
        .connect().await?;
    
    for t in new_transactions.iter() {
        sqlx::query("
                INSERT OR IGNORE INTO transactions
                (
                    value_date,
                    execution_date,
                    description,
                    iban_sender,
                    send_amount,
                    receive_amount,
                    account_balance,
                    sender_reference_number,
                    receiver_reference_number,
                    sender_receiver_name,
                    sender_receiver_place,
                    transaction_reference,
                    tags,
                    comment,
                    url
                )
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, '', '', '')")
            .bind(&t.value_date.format("%Y-%m-%d").to_string())
            .bind(&t.execution_date.format("%Y-%m-%d").to_string())
            .bind(&t.description)
            .bind(&t.iban_sender)
            .bind(t.send_amount)
            .bind(t.receive_amount)
            .bind(t.account_balance)
            .bind(&t.sender_reference_number)
            .bind(&t.receiver_reference_number)
            .bind(&t.sender_receiver_name)
            .bind(&t.sender_receiver_place)
            .bind(&t.transaction_reference)
            .execute(&mut conn).await?;
    }

    Ok(32)
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
