use std::any::Any;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use actix_web::{HttpRequest, HttpResponse, web};
use futures::Future;
use futures::future::err;

use crate::api::{Api, Definition, Dispatcher};
use crate::error::RigError;
use crate::handler::handlers::ComposeHandler;

pub mod handlers;
pub mod handlers_factory;
pub mod filters;
#[cfg(test)]
pub mod filters_test;
pub mod filters_factory;
pub mod router;

const CONTINUE: Option<FutureResponse> = None;

pub struct Request<'a> {
    pub req: &'a HttpRequest,
    pub body: &'a web::Bytes,
}

impl<'a> Request<'a> {
    pub fn new(req: &'a HttpRequest, body: &'a web::Bytes) -> Self {
        Request {
            req,
            body,
        }
    }
}


/// FutureResponse for Box wrap response
type FutureResponse = Box<dyn Future<Item=HttpResponse, Error=RigError>>;


pub trait Handler {
    fn handle(&self, req: &Request, exchange: &mut Exchange) -> Option<FutureResponse>;
}

pub trait Filter {
    fn filter(&self, req: &Request, exchange: &mut Exchange) -> bool;
}


pub struct Exchange {
    pub api: Option<Api>,
    pub context: Context,
}


impl Default for Exchange {
    fn default() -> Self {
        Exchange {
            api: None,
            context: Context::default(),
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