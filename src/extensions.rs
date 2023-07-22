use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};

use crate::{constants, context::Context};

pub struct ExtractContext(pub Context);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let request_id = match parts.headers.get(constants::REQUEST_ID) {
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    axum::Json(serde_json::json!({
                        "message": format!("missing request id header: {}",constants::REQUEST_ID)
                    })),
                ))
            }
            Some(v) => v.to_str().unwrap().to_owned(),
        };

        Ok(ExtractContext(Context { request_id }))
    }
}
