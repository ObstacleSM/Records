use crate::graphql::Schema;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct AppState {
    pub pool: Pool,
    pub schema: Schema,
}
