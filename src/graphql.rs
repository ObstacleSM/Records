use crate::models::map::Map;
use crate::models::player::Player;
use crate::models::record::Record;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{EmptyMutation, FieldResult, RootNode};
use std::convert::TryInto;

use crate::app_state::Pool;
use crate::schema;

#[derive(Clone)]
pub struct DbContext(pub Pool);

impl juniper::Context for DbContext {}

pub struct QueryRoot;

#[juniper::object(Context = DbContext,)]
impl QueryRoot {
    fn players(&self, context: &DbContext) -> FieldResult<Vec<Player>> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::players::table.load(conn)?)
    }

    fn player(&self, context: &DbContext, login: String) -> FieldResult<Option<Player>> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::players::table
            .find(&login)
            .get_result(conn)
            .optional()?)
    }

    fn maps(&self, context: &DbContext) -> FieldResult<Vec<Map>> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::maps::table.load(conn)?)
    }

    fn map(&self, context: &DbContext, id: String) -> FieldResult<Option<Map>> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::maps::table.find(&id).get_result(conn).optional()?)
    }

    fn record(
        &self,
        context: &DbContext,
        login: String,
        map_id: String,
    ) -> FieldResult<Option<Record>> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::records::table
            .find((&map_id, &login))
            .get_result(conn)
            .optional()?)
    }

    fn records(
        &self,
        context: &DbContext
    ) -> FieldResult<Vec<Record>> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::records::table
            .order_by(schema::records::updated_at.desc())
            .limit(100)
            .load(conn)?)
    }
}

#[juniper::object(Context = DbContext,)]
impl Player {
    fn login(&self) -> &str {
        self.login.as_str()
    }

    fn nickname(&self) -> &str {
        self.nickname.as_str()
    }

    fn maps(&self, context: &DbContext) -> FieldResult<Vec<Map>> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::maps::table
            .filter(schema::maps::player_id.eq(&self.login))
            .load(conn)?)
    }
}

#[juniper::object(Context = DbContext,)]
impl Map {
    fn id(&self) -> &str {
        self.maniaplanet_map_id.as_str()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn author(&self, context: &DbContext) -> FieldResult<Player> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::players::table
            .find(&self.player_id)
            .get_result(conn)?)
    }

    fn records(&self, context: &DbContext) -> FieldResult<Vec<Record>> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::records::table
            .filter(schema::records::map_id.eq(&self.maniaplanet_map_id))
            .order_by(schema::records::time)
            .limit(100)
            .load(conn)?)
    }
}

#[juniper::object(Context = DbContext,)]
impl Record {
    fn player(&self, context: &DbContext) -> FieldResult<Player> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::players::table
            .find(&self.player_id)
            .get_result(conn)?)
    }

    fn map(&self, context: &DbContext) -> FieldResult<Map> {
        let conn: &MysqlConnection = &context.0.get().unwrap();
        Ok(schema::maps::table.find(&self.map_id).get_result(conn)?)
    }

    fn rank(&self) -> i32 {
        // can panic
        self.rank.try_into().unwrap()
    }

    fn time(&self) -> i32 {
        self.time
    }

    fn respawn_count(&self) -> i32 {
        self.respawn_count
    }

    fn try_count(&self) -> i32 {
        self.try_count
    }

    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<DbContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
