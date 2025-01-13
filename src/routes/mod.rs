use auth_routes::auth_router;
use profile_route::profile_router;
use random_match_routes::random_match_router;
use axum:: Router;
use crate::AppState;
pub mod auth_routes;
pub mod profile_route;
pub mod random_match_routes;
pub fn api_routes(app_state:AppState) -> Router {
     Router::new()
    .nest("/auth", auth_router(app_state.clone()))
    .nest("/profile", profile_router(app_state.clone())) 
    .nest("/random-match",random_match_router(app_state.clone()) )
}