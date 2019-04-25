use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::sql_types::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable, Insertable)]
#[primary_key(map_id, player_id)]
pub struct Record {
    pub time: i32,
    pub respawn_count: i32,
    pub try_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub player_id: String,
    pub map_id: String,
}

// Rank Record is sent to the in-game records widget called the overview or small records widget
#[derive(Clone, QueryableByName, Deserialize, Serialize)]
#[serde(rename = "records")]
pub struct RankedRecord {
    #[sql_type = "BigInt"]
    pub rank: i64,
    #[sql_type = "VarChar"]
    #[serde(rename = "playerId")]
    pub player_id: String,
    #[sql_type = "VarChar"]
    pub nickname: String,
    #[sql_type = "Integer"]
    pub time: i32,
}

// PlayerRecord is one of the player's record, it contains all the informations needed to display a record by itself
#[derive(Clone, QueryableByName, Deserialize, Serialize)]
pub struct PlayerRecord {
    #[sql_type = "BigInt"]
    pub rank: i64,
    #[sql_type = "Integer"]
    pub time: i32,
    #[sql_type = "Integer"]
    pub respawn_count: i32,
    #[sql_type = "Integer"]
    pub try_count: i32,
    #[sql_type = "Datetime"]
    pub created_at: NaiveDateTime,
    #[sql_type = "Datetime"]
    pub updated_at: NaiveDateTime,
    #[sql_type = "VarChar"]
    pub player_id: String,
    #[sql_type = "VarChar"]
    pub map_id: String,
    #[sql_type = "VarChar"]
    pub map_name: String,
}
