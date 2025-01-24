use std::sync::Arc;
use axum::{routing:: Router, Extension};
use routes::api_routes;
use socket::random_match::random_match_socket;
use sqlx::postgres;
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
   pub db_pool:Arc<postgres::PgPool>,
   pub broadcaster:Arc<tokio::sync::broadcast::Sender<models::random_match::RandomMatch>>,
}

impl  AppState {
  
    fn db_pool(&self)->postgres::PgPool{
        self.db_pool.as_ref().clone()
    }
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_pool=database::establish_connection().await;
    let (broadcaster,_)=tokio::sync::broadcast::channel(100);
    let app_state=AppState{db_pool:Arc::new(db_pool),broadcaster:Arc::new(broadcaster)};
    let random_match_socket=random_match_socket(app_state.clone());
    let router = Router::new().nest("/api/v1", api_routes(app_state.clone())).layer(Extension(app_state)).layer(random_match_socket);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
