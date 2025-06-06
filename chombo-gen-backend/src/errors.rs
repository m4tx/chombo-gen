use std::error::Error;
use std::fmt::{Display, Formatter};

use chombo_gen_common::errors::ServiceErrorResponse;
use cot::StatusCode;
use cot::error::handler::RequestError;
use cot::json::Json;
use cot::response::{IntoResponse, WithStatus};
use schemars::JsonSchema;
use tokio::task::JoinError;
use tracing::error;

#[derive(Debug, Clone, JsonSchema)]
pub enum ServiceError {
    Internal(String),
    BadRequest(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(msg) => {
                write!(f, "{msg}")
            }
            Self::BadRequest(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

impl Error for ServiceError {}

impl ServiceError {
    fn get_http_status(&self) -> StatusCode {
        match self {
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<ServiceError> for cot::Error {
    fn from(error: ServiceError) -> Self {
        let status_code = error.get_http_status();
        cot::Error::with_status(error, status_code)
    }
}

pub async fn error_handler(error: RequestError) -> WithStatus<Json<ServiceErrorResponse>> {
    Json(ServiceErrorResponse {
        message: error.to_string(),
    })
    .with_status(error.status_code())
}

impl From<JoinError> for ServiceError {
    fn from(error: JoinError) -> Self {
        error!("{error:?}");
        Self::Internal(error.to_string())
    }
}
