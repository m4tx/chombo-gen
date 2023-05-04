use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Cursor;

use chombo_gen_common::errors::ServiceErrorResponse;
use log::error;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::tokio::task::JoinError;
use rocket::{response, Request, Response};

#[derive(Debug, Clone)]
pub enum ServiceError {
    Internal(String),
    BadRequest(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(msg) => {
                write!(f, "{}", msg)
            }
            Self::BadRequest(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl Error for ServiceError {}

impl ServiceError {
    fn get_http_status(&self) -> Status {
        match self {
            Self::Internal(_) => Status::InternalServerError,
            Self::BadRequest(_) => Status::BadRequest,
        }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ServiceError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let err_response = serde_json::to_string(&ServiceErrorResponse {
            message: self.to_string(),
        })
        .unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}

impl From<JoinError> for ServiceError {
    fn from(error: JoinError) -> Self {
        error!("{:?}", error);
        Self::Internal(error.to_string())
    }
}
