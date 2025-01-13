

use axum::{

    extract::FromRequest,
    http::StatusCode,
    response::{ IntoResponse, Response}, Json,
   
};
use serde::de::DeserializeOwned;
use serde_json::json;
use serde_json::Value;
use validator::{Validate, ValidationErrors};

pub struct ValidatedJson<T>(pub T);


impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: Validate + DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, &()).await.map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON: {}", err),
            )
                .into_response()
        })?;
        value.validate().map_err(|err| validation_error_response(err))?;

        Ok(ValidatedJson(value))
    }
}

fn validation_error_response(errors: ValidationErrors) -> Response {
    let error_details = errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let messages: Vec<Value> = errors
                .iter()
                .filter_map(|e| e.message.clone())
                .map(|msg| Value::String(msg.to_string()))
                .collect();
            (field.to_string(), Value::Array(messages))
        })
        .collect::<serde_json::Map<String, Value>>();

    let error_response = json!({
        "success": false,
        "message": "Validation failed",
        "errors": error_details,
    });
    

    (StatusCode::BAD_REQUEST, axum::Json(error_response)).into_response()
}
