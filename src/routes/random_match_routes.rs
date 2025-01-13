use axum::routing::{post, put,delete,get};
use axum::{middleware, Extension, Router};


use crate::middlewares::require_auth::require_auth;
use crate::services::random_match_service::RandomMatchService;
use crate::AppState;
use crate::controllers::random_match_controller::{enter_match, exit_match, update_match_filter,get_status};


pub fn random_match_router(app_state:AppState) -> Router { 
    let random_match_service=RandomMatchService{app_state:app_state}; 
    Router::new()
    .route("/status", get(get_status))
    .route("/join", post(enter_match))
    .route("/leave", delete(exit_match))
    .route("/change-filter", put(update_match_filter))
    .route_layer(middleware::from_fn(require_auth))
    .layer(Extension(random_match_service))
 }