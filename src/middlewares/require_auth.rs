use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response, Extension, Json};
use serde_json::json;

use crate::{ services::auth_service::verify_token, AppState};



pub async fn require_auth(
    Extension(app_state): Extension<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let token = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Missing or invalid token","success":false })),
            )
        })?;
        let result=verify_token(token, app_state).await;
        match result {
            Ok(user) => {
                req.extensions_mut().insert(user);
                Ok(next.run(req).await)
            }
            Err(e) => {
               
                Err((StatusCode::UNAUTHORIZED, Json(json!({ "message": e.message,"success":false,}))))
            }
        }

}
