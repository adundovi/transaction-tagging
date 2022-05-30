#[cfg(not(target_arch = "wasm32"))]
use sqlx::{
    FromRow,
    types::chrono::NaiveDate,
    Sqlite,
    pool::PoolConnection
};
#[cfg(target_arch = "wasm32")]
use chrono::NaiveDate;
#[cfg(target_arch = "wasm32")]
use reqwasm::http::Request;

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
    pub comment: Option<String>,
    pub url: Option<String>,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            id: -1,
            value_date: NaiveDate::from_ymd(1970, 1, 1),
            execution_date: NaiveDate::from_ymd(1970, 1, 1),
            description: None,
            iban_sender: "NaN".to_string(),
            send_amount: None,
            receive_amount: None,
            account_balance: 0f64,
            sender_reference_number: None,
            receiver_reference_number: None,
            sender_receiver_name: None,
            sender_receiver_place: None,
            transaction_reference: "NaN".to_string(),
            comment: None,
            url: None,
        }
    }
}

impl Transaction {
    
    pub fn new() -> Transaction {
        Default::default()
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_all(conn: &mut PoolConnection<Sqlite>) -> Result::<Vec<Transaction>, sqlx::Error> {
        let result = sqlx::query_as::<_, Transaction>(
            "SELECT * FROM transactions ORDER BY value_date DESC, id DESC")
            .fetch_all(conn).await?;
        Ok(result)
    }
    
    #[cfg(target_arch = "wasm32")]
    pub async fn get_all(api: &str) -> Result<Vec<Transaction>, reqwasm::Error> {
        let url = format!("{}/transactions", api);
        let resp = Request::get(&url).send().await?;
        let body = resp.json::<Vec<Transaction>>().await?;
        Ok(body)
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn get_by_id(conn: &mut PoolConnection<Sqlite>, id: u32) -> Result::<Transaction, sqlx::Error> {
        let t = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = $1;")
            .bind(id)
            .fetch_one(conn).await?;
        Ok(t)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_by_id(api: &str, id: u32) -> Result<Transaction, reqwasm::Error> {
        let url = format!("{}/transactions/{}", api, id);
        let resp = Request::get(&url).send().await?;
        let t = resp.json::<Transaction>().await?;
        Ok(t)
    }
    
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
                    comment,
                    url
                )
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NULL, NULL)")
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

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn set_comment_by_id(conn: &mut PoolConnection<Sqlite>, id: u32, comment: &Option<String>) -> Result::<(), sqlx::Error> {
        let _ = sqlx::query("UPDATE transactions SET comment = $2 WHERE id = $1;")
            .bind(id)
            .bind(comment)
            .execute(conn).await?;
        Ok(())
    }

}
