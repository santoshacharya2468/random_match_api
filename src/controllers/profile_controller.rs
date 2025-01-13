use axum::Extension;


use crate::{dtos::api_response::ApiResponse, middlewares::json_extractor::ValidatedJson, models::auth_user::{AuthUser, UpdateAuthUser}, services::profile_service::ProfileService};



pub async  fn get_profile(Extension(user): Extension<AuthUser>) -> ApiResponse<AuthUser> {    
    ApiResponse::ok(user)
}

pub async  fn update_profile(Extension(user): Extension<AuthUser>,
Extension(profile_service): Extension<ProfileService>,
ValidatedJson(payload):ValidatedJson<UpdateAuthUser>) -> ApiResponse<AuthUser> { 

    profile_service.update_profile(user.id,payload).await.map(ApiResponse::ok).unwrap_or(ApiResponse::bad_request("Invalid payload".to_string()))
}