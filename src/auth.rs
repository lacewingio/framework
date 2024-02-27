use axum::{
    async_trait,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: u32,
}

// From a request extract our authentication token.
#[async_trait]
impl<B> FromRequestParts<B> for AuthenticatedUser
    where
        B: Send,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &B) -> Result<Self, Self::Rejection>  {
        if let Some(user_id) = parts.headers.get("x-user-id") {
            if let Ok(user_id) = user_id.to_str() {
                if let Ok(user_id) = user_id.parse::<u32>() {
                    return Ok(AuthenticatedUser { user_id });
                }
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "x-user-id not found or unparseable as u32",
        )
            .into_response())
    }
}