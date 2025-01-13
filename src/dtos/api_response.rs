use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};




#[derive(Debug,Serialize,Deserialize)]
pub struct ApiResponse<T>{
    status_code: u16,
    message: String,
    data: Option<T>
}
impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::OK.as_u16(),
            message: "Success".to_string(),
            data: Some(data),
        }
    }
    pub fn created(data: T) -> Self {
        ApiResponse {
            status_code: StatusCode::CREATED.as_u16(),
            message: "Resource created".to_string(),
            data: Some(data),
        }
    }
    pub fn bad_request(message: String) -> Self {
        ApiResponse {
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            message,
            data: None, 
        }
    }
    pub fn forbidden(message: String) -> Self {
        ApiResponse {
            status_code: StatusCode::FORBIDDEN.as_u16(),
            message,
            data: None, 
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::OK);
        let body = Json(self); 
        (status_code, body).into_response()
 
    }
}
