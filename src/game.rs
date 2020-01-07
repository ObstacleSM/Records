use crate::app_state::AppState;
use crate::models;
use crate::records_api;
use crate::xml;
use actix_web::{error, web, Error, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "response")]
pub struct HasFinishedResult {
    #[serde(rename = "newBest")]
    pub is_new_best: bool,
    pub login: String,
    pub old: i32,
    pub new: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasFinishedPayload {
    pub time: i32,
    #[serde(alias = "respawnCount")]
    pub respawn_count: i32,
    #[serde(alias = "mapId")]
    pub map_id: String,
    #[serde(alias = "playerId")]
    pub player_id: String,
}

fn string_to_xml_response(
    res: Result<String, error::BlockingError<error::BlockingError<()>>>,
) -> Result<HttpResponse, Error> {
    match res {
        Ok(body) => Ok(xml::xml_response(body)),
        Err(e) => {
            eprintln!("Error while sending xml: {}", e.to_string());
            Ok(HttpResponse::InternalServerError().into())
        }
    }
}

pub fn has_finished_route(
    payload: web::Json<HasFinishedPayload>,
    state: web::Data<Arc<AppState>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let banned_players = ["51x7yn1n3"];
        let is_banned = banned_players
            .iter()
            .any(|&banned_player| payload.player_id == banned_player);

        if is_banned {
            return Err(error::BlockingError::Error(()));
        }

        let conn: &MysqlConnection = &state.pool.get().unwrap();

        let finished = records_api::has_finished(
            conn,
            payload.time,
            payload.respawn_count,
            &payload.player_id,
            &payload.map_id,
        );

        let result = match finished {
            Ok((is_new_best, old, new)) => HasFinishedResult {
                is_new_best,
                old,
                new,
                login: String::from(&payload.player_id),
            },
            Err(_) => return Err(error::BlockingError::Error(())),
        };

        match serde_xml_rs::to_string(&result) {
            Ok(body) => Ok(body),
            Err(_) => Err(error::BlockingError::Error(())),
        }
    })
    // then we can send the response
    .then(string_to_xml_response)
}

#[derive(Deserialize)]
pub struct OverviewQuery {
    #[serde(alias = "mapId")]
    pub map_id: String,
    #[serde(alias = "playerId")]
    pub player_id: String,
}

pub fn overview_route(
    parameters: web::Query<OverviewQuery>,
    state: web::Data<Arc<AppState>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let conn: &MysqlConnection = &state.pool.get().unwrap();
        let result = records_api::overview(conn, &parameters.player_id, &parameters.map_id);

        match result {
            Ok(records) => Ok(xml::to_string(records)),
            Err(e) => {
                eprintln!("Error: {}", e.to_string());
                Err(error::BlockingError::Error(()))
            }
        }
    })
    // then we can send the response
    .then(string_to_xml_response)
}

pub fn ok_stub() -> HttpResponse {
    HttpResponse::Ok().body("<response><id>ok</id></response>")
}

pub fn player_replace_or_create(
    data: web::Json<models::player::Player>,
    state: web::Data<Arc<AppState>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let conn: &MysqlConnection = &state.pool.get().unwrap();
        match data.insert_or_replace(conn) {
            Ok(_size) => Ok(String::from("<response><id>ok</id></response>")),
            Err(_) => Err(error::BlockingError::Error(())),
        }
    })
    // then we can send the response
    .then(string_to_xml_response)
}

pub fn map_replace_or_create(
    data: web::Json<models::map::Map>,
    state: web::Data<Arc<AppState>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let conn: &MysqlConnection = &state.pool.get().unwrap();
        match data.insert_or_replace(conn) {
            Ok(_size) => Ok(String::from("<response><id>ok</id></response>")),
            Err(_) => Err(error::BlockingError::Error(())),
        }
    })
    // then we can send the response
    .then(string_to_xml_response)
}
