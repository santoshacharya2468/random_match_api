use axum::{response::IntoResponse, Extension};

use crate::{dtos::api_response::ApiResponse, models::auth_user::AuthUser, services::random_match_service::RandomMatchService};


pub async  fn get_status(Extension(math_service):Extension<RandomMatchService>,
Extension(user): Extension<AuthUser>
) -> impl IntoResponse{
    let result= math_service.get_status(user.id).await;
    match result{
        Ok(user)=>ApiResponse::ok(user),
        Err(e)=>ApiResponse::bad_request(e.message)
    }
}

pub async fn enter_match(Extension(math_service):Extension<RandomMatchService>,
Extension(user): Extension<AuthUser>) -> impl IntoResponse{
    let result= math_service.create_random_match(user.id).await;
    match result{
        Ok(user)=>ApiResponse::ok(user),
        Err(e)=>ApiResponse::bad_request(e.message)
    }
    
}
pub async  fn exit_match(
    Extension(math_service):Extension<RandomMatchService>,
    Extension(user): Extension<AuthUser>
) -> impl IntoResponse {
    let result= math_service.exit_match(user.id).await;
    match result{
        Ok(user)=>ApiResponse::ok(user),
        Err(e)=>ApiResponse::bad_request(e.message)
    }
}

pub async  fn update_match_filter() -> impl IntoResponse {
    ApiResponse::created("Random match filter updated".to_string())  
}