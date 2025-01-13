use crate::{
    dtos::{api_response::ApiResponse, auth_tokens::AuthTokens, social_login_requet::SocialLoginRequest,},
    middlewares::json_extractor::ValidatedJson,
    services::auth_service::AuthService
};
use axum::Extension;

pub async fn socail_login(
    Extension(auth_service): Extension<AuthService>,
    ValidatedJson(login_request): ValidatedJson<SocialLoginRequest>,
) -> ApiResponse<AuthTokens> {
    let result = auth_service.social_login(login_request).await;
    match result {
        Ok(user) => ApiResponse::ok(user),
        Err(e) => ApiResponse::bad_request(e.message),
    }
}

pub async fn register() -> ApiResponse<String> {
    ApiResponse::created("Hello Register".to_string())
}
