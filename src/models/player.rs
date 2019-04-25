use crate::schema::players;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable, Insertable)]
#[primary_key(login)]
pub struct Player {
    pub login: String,
    pub nickname: String,
}

impl Player {
    pub fn insert_or_replace(&self, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::replace_into(players::table)
            .values(self)
            .execute(conn)
    }
}
