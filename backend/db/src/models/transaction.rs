#[cfg(not(target_arch = "wasm32"))]
use sqlx::{
    FromRow,
    types::chrono::NaiveDate
};
#[cfg(target_arch = "wasm32")]
use chrono::NaiveDate;

use serde::{Deserialize, Serialize};

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

