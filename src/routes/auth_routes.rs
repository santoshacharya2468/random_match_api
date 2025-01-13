

use axum::{ routing:: post, Extension, Router};

use crate::{controllers::auth_controller::{ socail_login, register}, services::auth_service::AuthService, AppState};

pub fn auth_router(app_state:AppState) -> Router {
    let auth_service=AuthService{app_state:app_state.clone()};
    Router::new()
    .route("/social-login", post(socail_login))
    .route("/register", post(register))
    .layer(Extension(auth_service))
}