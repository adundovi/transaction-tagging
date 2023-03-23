#[cfg(not(target_arch = "wasm32"))]
use sqlx::{
    FromRow,
    Type,
    Sqlite,
    pool::PoolConnection
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromRow, Type))]
pub struct Tag {
    pub id: i64,
    pub tag: String,
    pub description: Option<String>,
    pub url: Option<String>
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct NewTag {
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
/*
impl Default for Tag {
    fn default() -> Self { None }
}*/

impl Default for Tag {
    fn default() -> Self {
        Tag {
            id: -1,
            tag: "Undefined".to_string(),
            description: None,
            url: None
        }
    }
}

            
impl Tag {
    
    pub fn new() -> Tag {
        Default::default()
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn insert(conn: &mut PoolConnection<Sqlite>, t: &NewTag) ->  Result::<(), sqlx::Error> {
        sqlx::query("
                INSERT OR IGNORE INTO tags
                (
                    tag,
                    description,
                    url
                )
                VALUES
                    ($1, $2, $3)")
            .bind(&t.tag)
            .bind(&t.description)
            .bind(&t.url)
            .execute(conn).await?;
        
        Ok(())
    }

}
