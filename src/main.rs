use std::{env, sync::Arc};
use axum::{routing:: Router, Extension};
use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection},  PgConnection};
use routes::api_routes;
use socket::random_match::random_match_socket;
use sqlx::postgres::{self, PgPoolOptions};
pub mod routes;
pub mod controllers;
pub mod models;
pub mod services;
pub mod dtos;
pub mod database;
pub mod utils;
pub mod middlewares;
pub mod socket;
#[derive(Clone)]
pub struct AppState {
   pub db_connection:Arc<Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
   pub db_pool:Arc<postgres::PgPool>
}

impl  AppState {
    fn db(&self)->PooledConnection<ConnectionManager<PgConnection>>{
        self.db_connection.as_ref().get().unwrap()
    }
    fn db_pool(&self)->postgres::PgPool{
        self.db_pool.as_ref().clone()
    }
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_connection=database::establish_connection();
    let db_url=env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool= PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url).await.unwrap();
    let app_state=AppState{db_connection:Arc::new(db_connection), db_pool:Arc::new(db_pool)};
    let random_match_socket=random_match_socket(app_state.clone());
    let router = Router::new().nest("/api/v1", api_routes(app_state.clone())).layer(Extension(app_state)).layer(random_match_socket);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
