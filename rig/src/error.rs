use derive_more::Display;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, Display, PartialEq)]
pub enum RigError {
    #[display(fmt = "unknown error")]
    Unknown,

    #[display(fmt = "not found path")]
    NotFoundPath,

    #[display(fmt = "proxy request fail")]
    AgentRequest,

    #[display(fmt = "proxy response fail")]
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