#[cfg(not(target_arch = "wasm32"))]
use sqlx::{
    FromRow,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromRow))]
pub struct Tag {
    pub id: i64,
    pub tag: String,
    pub description: Option<String>,
    pub url: Option<String>
}

/*
    sqlx::query("CREATE TABLE IF NOT EXISTS trans_tags_relations (
                id              INTEGER PRIMARY KEY NOT NULL,
                transaction_id  INTEGER REFERENCES transactions(id),
                tag_id          INTEGER REFERENCES tags(id),
*/
