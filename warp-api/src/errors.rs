use auth::JWTError;
use domain::{DomainError, GetAdventureError};
use warp::hyper::StatusCode;

use crate::response::{ErrorMessage, ErrorResponse};

impl From<DomainError> for ErrorResponse {
    fn from(e: DomainError) -> ErrorResponse {
        ErrorResponse(
            ErrorMessage {
                message: e.to_string(),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            },
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl From<GetAdventureError> for ErrorResponse {
    fn from(e: GetAdventureError) -> ErrorResponse {
        match &e {
            GetAdventureError::NotFound { .. } => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::NOT_FOUND.as_u16(),
                },
                StatusCode::NOT_FOUND,
            ),
            GetAdventureError::DomainError(_) => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl From<JWTError> for ErrorResponse {
    fn from(e: JWTError) -> Self {
        match &e {
            JWTError::Invalid => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                },
                StatusCode::UNAUTHORIZED,
            ),
            JWTError::Missing => ErrorResponse(
                ErrorMessage {
                    message: e.to_string(),
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                },
                StatusCode::UNAUTHORIZED,
            ),
        }
    }
}
