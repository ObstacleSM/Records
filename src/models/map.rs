use crate::escape::Escape;
use crate::models::player::Player;
use crate::schema::{maps, players};
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
        let author_exists: Option<Player> = players::table
            .find(&self.player_id)
            .get_result(conn)
            .optional()?;

        if let None = author_exists {
            let player = Player {
                login: self.player_id.clone(),
                nickname: self.player_id.clone(),
            };
            diesel::insert_into(players::table)
                .values(player)
                .execute(conn)?;
        }

        let map_exists: Option<Map> = maps::table
            .find(&self.maniaplanet_map_id)
            .get_result(conn)
            .optional()?;

        let escaped_name = format!("{}", Escape(&self.name));

        match map_exists {
            Some(map) => diesel::update(&map)
                .set((
                    maps::name.eq(escaped_name),
                    maps::player_id.eq(&self.player_id),
                ))
                .execute(conn),
            _ => diesel::insert_into(maps::table)
                .values((
                    maps::name.eq(escaped_name),
                    maps::maniaplanet_map_id.eq(&self.maniaplanet_map_id),
                    maps::player_id.eq(&self.player_id),
                ))
                .execute(conn),
        }
    }
}
