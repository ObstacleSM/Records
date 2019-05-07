#[macro_use]
extern crate diesel;

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::Future;
use serde_derive::{Deserialize, Serialize};

use std::env;

pub mod models;
pub mod records_api;
pub mod schema;
pub mod xml;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// Some fields need to be renamed to maintain backward-compatibility

#[derive(Serialize)]
#[serde(rename = "response")]
pub struct HasFinishedResult<'a> {
    #[serde(rename = "newBest")]
    pub is_new_best: bool,
    pub login: &'a str,
    pub old: i32,
    pub new: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct HasFinishedPayload {
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

fn has_finished_route(
    payload: web::Json<HasFinishedPayload>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let conn: &MysqlConnection = &pool.get().unwrap();
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
                login: &payload.player_id,
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
struct OverviewQuery {
    #[serde(alias = "mapId")]
    pub map_id: String,
    #[serde(alias = "playerId")]
    pub player_id: String,
}

fn overview_route(
    parameters: web::Query<OverviewQuery>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let conn: &MysqlConnection = &pool.get().unwrap();
        let result = records_api::overview(conn, &parameters.player_id, &parameters.map_id);

        match result {
            Ok(records) => Ok(xml::to_string(records)),
            Err(e) =>  {
                eprintln!("Error: {}", e.to_string());
                Err(error::BlockingError::Error(()))
            }
        }
    })
    // then we can send the response
    .then(string_to_xml_response)
}

#[derive(Deserialize)]
struct LatestQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

fn latest_records_route(
    parameters: web::Query<LatestQuery>,
    pool: web::Data<Pool>,
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

        let conn: &MysqlConnection = &pool.get().unwrap();
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

fn map_records_route(
    map_id: web::Path<String>,
    parameters: web::Query<LatestQuery>,
    pool: web::Data<Pool>,
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

        let conn: &MysqlConnection = &pool.get().unwrap();
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

fn player_records_route(
    player_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let conn: &MysqlConnection = &pool.get().unwrap();
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

fn ok_stub() -> HttpResponse {
    HttpResponse::Ok().body("<response><id>ok</id></response>")
}

fn player_replace_or_create(
    data: web::Json<models::player::Player>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let conn: &MysqlConnection = &pool.get().unwrap();
        match data.insert_or_replace(conn) {
            Ok(_size) => Ok(String::from("<response><id>ok</id></response>")),
            Err(_) => Err(error::BlockingError::Error(())),
        }
    })
    // then we can send the response
    .then(string_to_xml_response)
}

fn map_replace_or_create(
    data: web::Json<models::map::Map>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // First we block during the access to the database
    web::block(move || {
        let conn: &MysqlConnection = &pool.get().unwrap();
        match data.insert_or_replace(conn) {
            Ok(_size) => Ok(String::from("<response><id>ok</id></response>")),
            Err(_) => Err(error::BlockingError::Error(())),
        }
    })
    // then we can send the response
    .then(string_to_xml_response)
}

fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();


    // Create the database connection pool
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create the MysqlConnection pool.");

    HttpServer::new(move || {
        use actix_web::http;
        use middleware::cors::Cors;

        App::new()
            .data(pool.clone())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("https://www.obstacle.ovh")
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/api/Records/player-finished")
                    .route(web::post().to_async(has_finished_route)),
            )
            .service(
                web::resource("/api/Records/overview").route(web::get().to_async(overview_route)),
            )
            .service(web::resource("/api/Users/Login").route(web::post().to(ok_stub)))
            .service(
                web::resource("/api/Players/replaceOrCreate")
                    .route(web::post().to_async(player_replace_or_create)),
            )
            .service(
                web::resource("/api/Maps/replaceOrCreate")
                    .route(web::post().to_async(map_replace_or_create)),
            )
            .service(
                web::resource("/api/latest-records")
                    .route(web::get().to_async(latest_records_route)),
            )
            .service(
                web::resource("/api/map-records/{id}")
                    .route(web::get().to_async(map_records_route)),
            )
            .service(
                web::resource("/api/player-records/{id}")
                    .route(web::get().to_async(player_records_route)),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
}
