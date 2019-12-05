use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum RigError {
    #[display(fmt = "unknown error")]
    Unknown,

    #[display(fmt = "not found path")]
    NotFoundPath,

    #[display(fmt = "not matched filters")]
    NotMatchedFilters,

    #[display(fmt = "proxy request error")]
    AgentRequest,

    #[display(fmt = "proxy response error")]
    AgentResponse,

    #[display(fmt = "wrap error")]
    WrapError(Box<dyn ResponseError>),
}

impl ResponseError for RigError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RigError::NotFoundPath => {
                HttpResponse::NotFound().finish()
            }
            RigError::WrapError(res) => {
                res.error_response()
            }
            _ => {
                HttpResponse::BadRequest().finish()
            }
        }
    }
}