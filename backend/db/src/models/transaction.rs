#[cfg(not(target_arch = "wasm32"))]
use sqlx::{
    FromRow,
    types::chrono::NaiveDate,
    Sqlite,
    pool::PoolConnection
};
#[cfg(target_arch = "wasm32")]
use chrono::NaiveDate;

use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use utils::{date_serializer, currency_serializer_option, currency_serializer};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct NewCSVTransaction {
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
            
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromRow))]
pub struct Transaction {
    pub id: i64,
    pub value_date: NaiveDate,
    pub execution_date: NaiveDate,
    pub description: Option<String>,
    pub iban_sender: String,
    pub send_amount: Option<f64>,
    pub receive_amount: Option<f64>,
    pub account_balance: f64,
    pub sender_reference_number: Option<String>,
    pub receiver_reference_number: Option<String>,
    pub sender_receiver_name: Option<String>,
    pub sender_receiver_place: Option<String>,
    pub transaction_reference: String,
    pub tags: Option<String>,
    pub comment: Option<String>,
    pub url: Option<String>,
}


impl Transaction {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn insert(conn: &mut PoolConnection<Sqlite>, t: &NewCSVTransaction) ->  Result::<(), sqlx::Error> {
        
        
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
            .execute(conn).await?;
        
        Ok(())
    }
}