use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::sql_types::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable, Insertable)]
#[primary_key(map_id, player_id)]
pub struct Record {
    pub rank: u32,
    pub time: i32,
    pub respawn_count: i32,
    pub try_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub player_id: String,
    pub map_id: String,
}

#[derive(Clone, QueryableByName, Deserialize, Serialize)]
#[serde(rename = "records")]
pub struct RankedRecord {
    #[sql_type = "Unsigned<Integer>"]
    pub rank: u32,
    #[sql_type = "VarChar"]
    #[serde(rename = "playerId")]
    pub player_id: String,
    #[sql_type = "VarChar"]
    pub nickname: String,
    #[sql_type = "Integer"]
    pub time: i32,
}
