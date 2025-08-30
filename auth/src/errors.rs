use core::error;

use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error,Debug)]
pub enum CustomErrors{
    #[error("Empty Payload")]
    EmptyPayload,

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("No User Found")]
    NoUserFound,


    #[error("Invalid User")]
    InvalidUser
}

impl IntoResponse for CustomErrors{
    fn into_response(self) -> axum::response::Response {
        match self{
            CustomErrors::EmptyPayload => (StatusCode::BAD_REQUEST,"Empty Payload").into_response(),
            CustomErrors::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error").into_response(),
            CustomErrors::NoUserFound => (StatusCode::NOT_FOUND,"No User Found").into_response(),
            CustomErrors::InvalidUser => (StatusCode::UNAUTHORIZED,"Invalid User").into_response(),
        }
    }
}