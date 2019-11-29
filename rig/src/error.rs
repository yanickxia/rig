use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
pub enum RigError {
    #[display(fmt = "unknown error")]
    Unknown,

    #[display(fmt = "not found path")]
    NotFoundPath,

    #[display(fmt = "proxy request error")]
    AgentRequest,

    #[display(fmt = "proxy response error")]
    AgentResponse,
}

impl ResponseError for RigError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RigError::NotFoundPath => {
                HttpResponse::NotFound().finish()
            }
            _ => {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}