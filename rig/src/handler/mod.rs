use std::collections::HashMap;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::client::Client;
use futures::Future;
use futures::future::err;

use crate::api::{Api, Definition};
use crate::error::RigError;

pub mod handlers;
pub mod handlers_factory;
pub mod filters;
#[cfg(test)]
pub mod filters_test;
pub mod filters_factory;
pub mod router;
pub mod handler_provider;

const CONTINUE: Option<FutureResponse> = None;

pub struct Request<'a> {
    pub req: &'a HttpRequest,
    pub body: &'a web::Bytes,
    pub client: &'a Client,
}

impl<'a> Request<'a> {
    pub fn new(req: &'a HttpRequest, body: &'a web::Bytes, client: &'a Client) -> Self {
        Request {
            req,
            body,
            client,
        }
    }
}


/// FutureResponse for Box wrap response
type FutureResponse = Box<dyn Future<Item=HttpResponse, Error=RigError>>;


pub trait Handler: Sync + Send {
    fn handle(&self, req: &Request, exchange: &mut Exchange) -> Option<FutureResponse>;
}

pub trait Filter: Sync + Send {
    fn filter(&self, req: &Request, exchange: &mut Exchange) -> bool;
}


pub struct Exchange {
    pub api: Option<Api>,
    pub context: Context,
    pub resolved_path_variables: Option<HashMap<String, String>>,
}


impl Default for Exchange {
    fn default() -> Self {
        Exchange {
            api: None,
            context: Context::default(),
            resolved_path_variables: None,
        }
    }
}

pub struct Context {
    pub definition: Option<Definition>,
    pub destination: Option<String>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            definition: None,
            destination: None,
        }
    }
}

fn error_response(error: RigError) -> Option<FutureResponse> {
    Some(Box::new(err(error)))
}