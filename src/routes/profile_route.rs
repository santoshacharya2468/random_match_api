use axum::{middleware, routing::{get, patch}, Extension, Router};
use crate::{controllers::profile_controller::{get_profile,update_profile}, middlewares::require_auth::require_auth, services::profile_service:: ProfileService, AppState};
pub fn profile_router(app_state:AppState) -> Router {
    let profile_service=ProfileService{app_state:app_state.clone()};
    Router::new()
    .route("/me", get(get_profile))
    .route("/me", patch(update_profile))
    .layer(Extension(profile_service))
    .layer(middleware::from_fn(require_auth))
    .layer(Extension(app_state))
}