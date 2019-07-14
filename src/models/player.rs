use crate::escape::Escape;
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
        let exists: Option<Player> = players::table
            .find(&self.login)
            .get_result(conn)
            .optional()?;

        let escaped_nick = format!("{}", Escape(&self.nickname));

        match exists {
            Some(player) => diesel::update(&player)
                .set(players::nickname.eq(escaped_nick))
                .execute(conn),
            _ => diesel::insert_into(players::table)
                .values((
                    players::login.eq(&self.login),
                    players::nickname.eq(escaped_nick),
                ))
                .execute(conn),
        }
    }
}
