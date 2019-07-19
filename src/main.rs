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

use crate::app_state::*;
use crate::game::*;
use crate::graphql::*;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use futures::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::env;

#[allow(dead_code)]
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
    dotenv().ok();

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
                    .allowed_origin("https://www.obstacle.ovh")
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
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            //.service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:3000")?
    .run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use crate::models::player::Player;
    use crate::models::map::Map;

    fn create_app_state() -> Arc<AppState> {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).unwrap();

        Arc::new(AppState {
            pool,
            schema: create_schema(),
        })
    }

    #[test]
    fn test_player_finished_get() {
        let state = create_app_state();
        let mut app = test::init_service(
            App::new().data(Arc::clone(&state)).service(
                web::resource("/api/Records/player-finished")
                    .route(web::post().to_async(has_finished_route)),
            ),
        );
        let req = test::TestRequest::get()
            .uri("/api/Records/player-finished")
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert!(resp.status().is_client_error());
    }

    #[test]
    fn test_player_finished_post() {
        let state = create_app_state();
        let mut app = test::init_service(
            App::new().data(Arc::clone(&state))
                .service(
                web::resource("/api/Records/player-finished")
                    .route(web::post().to_async(has_finished_route)),
            ),
        );

        let payload = HasFinishedPayload {
            time: 72000,
            respawn_count: 32,
            map_id: String::from("NullId"),
            player_id: String::from("smokegun"),
        };

        let req = test::TestRequest::post()
            .uri("/api/Records/player-finished")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert!(resp.status().is_success());
    }

    #[test]
    fn test_overview_get() {
        let state = create_app_state();
        let mut app = test::init_service(
            App::new().data(Arc::clone(&state))
                .service(
                    web::resource("/api/Records/overview").route(web::get().to_async(overview_route)),
            ),
        );
        let req = test::TestRequest::get()
            .uri("/api/Records/overview?map_id=NullId&player_id=smokegun")
            .to_request();

        let resp = test::call_service(&mut app, req);

        assert!(resp.status().is_success());
    }

    #[test]
    fn test_player_replace_or_create_get() {
        let state = create_app_state();
        let mut app = test::init_service(
            App::new().data(Arc::clone(&state)).service(
                web::resource("/api/Players/replaceOrCreate")
                    .route(web::post().to_async(player_replace_or_create)),
            ),
        );
        let req = test::TestRequest::get()
            .uri("/api/Players/replaceOrCreate")
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert!(resp.status().is_client_error());
    }

    #[test]
    fn test_player_replace_or_create_post() {
        let state = create_app_state();
        let mut app = test::init_service(
            App::new().data(Arc::clone(&state))
                .service(
                web::resource("/api/Players/replaceOrCreate")
                    .route(web::post().to_async(player_replace_or_create)),
            ),
        );

        let payload = Player {
            login: String::from("smokegun"),
            nickname: String::from("smokegun"),
        };

        let req = test::TestRequest::post()
            .uri("/api/Players/replaceOrCreate")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert!(resp.status().is_success());
    }


    #[test]
    fn test_map_replace_or_create_get() {
        let state = create_app_state();
        let mut app = test::init_service(
            App::new().data(Arc::clone(&state)).service(
                web::resource("/api/Maps/replaceOrCreate")
                    .route(web::post().to_async(map_replace_or_create)),
            ),
        );
        let req = test::TestRequest::get()
            .uri("/api/Maps/replaceOrCreate")
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert!(resp.status().is_client_error());
    }

    #[test]
    fn test_map_replace_or_create_post() {
        let state = create_app_state();
        let mut app = test::init_service(
            App::new().data(Arc::clone(&state))
                .service(
                web::resource("/api/Maps/replaceOrCreate")
                    .route(web::post().to_async(map_replace_or_create)),
            ),
        );

        let payload = Map {
            maniaplanet_map_id: String::from("NullId"),
            name: String::from("NullId"),
            player_id: String::from("smokegun"),
        };

        let req = test::TestRequest::post()
            .uri("/api/Maps/replaceOrCreate")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req);
        assert!(resp.status().is_success());
    }
}
