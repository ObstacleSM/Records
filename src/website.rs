use crate::app_state::AppState;
use crate::records_api;
use actix_web::{error, web, Error, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct LatestQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub fn latest_records_route(
    parameters: web::Query<LatestQuery>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let offset = if let Some(o) = parameters.offset {
            o
        } else {
            0
        };

        let limit = if let Some(l) = parameters.limit {
            if l == 0 || l > 100 {
                100
            } else {
                l
            }
        } else {
            100
        };

        let conn: &MysqlConnection = &state.pool.get().unwrap();
        let result = records_api::latest_records(conn, offset, limit);

        match result {
            Ok(records) => Ok(records),
            Err(_) => Err(error::BlockingError::Error(())),
        }
    })
    // then we can send the response
    .then(move |res| match res {
        Ok(body) => Ok(HttpResponse::Ok().json(body)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn map_records_route(
    map_id: web::Path<String>,
    parameters: web::Query<LatestQuery>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let offset = if let Some(o) = parameters.offset {
            o
        } else {
            0
        };

        let limit = if let Some(l) = parameters.limit {
            if l == 0 || l > 100 {
                100
            } else {
                l
            }
        } else {
            100
        };

        let conn: &MysqlConnection = &state.pool.get().unwrap();
        let result = records_api::map_records(conn, offset, limit, &map_id);

        match result {
            Ok(records) => Ok(records),
            Err(e) => {
                eprint!("{}", e.to_string());
                Err(error::BlockingError::Error(()))
            }
        }
    })
    // then we can send the response
    .then(move |res| match res {
        Ok(None) => Ok(HttpResponse::NotFound().into()),
        Ok(Some(body)) => Ok(HttpResponse::Ok().json(body)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn player_records_route(
    player_id: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn: &MysqlConnection = &state.pool.get().unwrap();
        let result = records_api::player_records(conn, &player_id);

        match result {
            Ok(records) => Ok(records),
            Err(e) => {
                eprint!("{}", e.to_string());
                Err(error::BlockingError::Error(()))
            }
        }
    })
    .then(move |res| match res {
        Ok(None) => Ok(HttpResponse::NotFound().into()),
        Ok(Some(body)) => Ok(HttpResponse::Ok().json(body)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
