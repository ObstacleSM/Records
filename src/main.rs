#[macro_use]
extern crate diesel;

// database
pub mod app_state;
pub mod graphql;
pub mod models;
pub mod schema;

// records related functions
pub mod records_api;

// utils
pub mod escape;
pub mod xml;

// routes used in game
pub mod game;
// routes used in website
pub mod website;

use crate::app_state::*;
use crate::game::*;
use crate::graphql::*;
use crate::website::*;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, http, HttpResponse, HttpServer};
use middleware::cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::env;

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    state: web::Data<Arc<AppState>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let ctx = DbContext(state.pool.clone());
        let res = data.execute(&state.schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
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

    let app_state = Arc::new(AppState {
        pool,
        schema: create_schema(),
    });

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
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
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
