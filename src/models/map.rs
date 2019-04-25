use crate::schema::maps;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Insertable, Deserialize, Serialize, Debug)]
#[primary_key(maniaplanet_map_id)]
pub struct Map {
    #[serde(alias = "maniaplanetMapId")]
    pub maniaplanet_map_id: String,
    pub name: String,
    #[serde(alias = "playerId")]
    pub player_id: String,
}

impl Map {
    pub fn insert_or_replace(&self, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::replace_into(maps::table).values(self).execute(conn)
    }
}
