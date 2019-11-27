use std::cell::RefCell;

use actix_web::{HttpRequest, HttpResponse, web};
use futures::Future;

use crate::api::Api;
use crate::error::RigError;

pub mod handlers;
#[cfg(test)]
pub mod handlers_tests;
pub mod handler_factory;


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


pub trait Handler {
    fn handle(&self, req: &Request, context: &Exchange) -> Box<dyn Future<Item=HttpResponse, Error=RigError>>;
}


pub struct Exchange<'a> {
    api: RefCell<Option<Api>>,
    pub handler_chain: HandlerChain<'a>,
    context: RefCell<Context>,
}

impl Default for Exchange<'_> {
    fn default() -> Self {
        Exchange {
            api: RefCell::new(Option::None),
            handler_chain: Default::default(),
            context: RefCell::new(Context::default()),
        }
    }
}

pub struct Context {
    pub destination: Option<String>
}

impl Default for Context {
    fn default() -> Self {
        Context { destination: Option::None }
    }
}


pub struct HandlerChain<'a> {
    current: RefCell<usize>,
    handlers: Vec<&'a dyn Handler>,
}

impl Default for HandlerChain<'_> {
    fn default() -> Self {
        HandlerChain {
            current: RefCell::new(0),
            handlers: vec![],
        }
    }
}

impl Handler for HandlerChain<'_> {
    fn handle(&self, req: &Request, context: &Exchange) -> Box<dyn Future<Item=HttpResponse, Error=RigError>> {
        return self.next().handle(req, context);
    }
}


impl<'a> HandlerChain<'a> {
    pub fn first(&self) -> &dyn Handler {
        return self.handlers[0];
    }

    pub fn next(&self) -> &dyn Handler {
        let mut mut_current = self.current.borrow_mut();
        let next = *mut_current + 1;
        *mut_current = next;
        return self.handlers[next];
    }

    pub fn append(&mut self, handler: &'a dyn Handler) -> &mut Self {
        self.handlers.push(handler);
        return self;
    }
}
