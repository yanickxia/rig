use std::cell::RefCell;

use actix_web::client::Client;
use actix_web::HttpResponse;
use futures::{Future, Stream};
use log::{debug, error, info};

use crate::error::RigError;
use crate::handler::{CONTINUE, Exchange, FutureResponse, Handler, Request};

/// HandlerChain
pub struct ComposeHandler {
    handlers: Vec<Box<dyn Handler>>,
}

impl Default for ComposeHandler {
    fn default() -> Self {
        ComposeHandler {
            handlers: vec![],
        }
    }
}

impl Handler for ComposeHandler {
    fn handle(&self, req: &Request, context: &mut Exchange) -> Option<FutureResponse> {
        for h in self.handlers.iter() {
            match h.handle(req, context) {
                Some(resp) => {
                    return Option::Some(resp);
                }
                None => {
                    continue;
                }
            }
        }
        unreachable!("never be in here");
    }
}


impl ComposeHandler {
    pub fn first(&self) -> &dyn Handler {
        return self.handlers[0].as_ref();
    }

    pub fn append(&mut self, handler: Box<dyn Handler>) -> &mut Self {
        self.handlers.push(handler);
        return self;
    }
}


/// DirectDispatcher
pub struct DirectDispatcher {}

impl Default for DirectDispatcher {
    fn default() -> Self {
        DirectDispatcher {}
    }
}

impl Handler for DirectDispatcher {
    fn handle(&self, req: &Request, exchange: &mut Exchange) -> Option<FutureResponse> {
        let mut dest = exchange.context
            .definition.as_ref().unwrap().dispatcher.destination.clone();
        exchange.resolved_path_variables
            .as_ref()
            .unwrap()
            .iter()
            .for_each(|it| {
                let (k, v) = it;
                let replace_key = "{".to_owned() + k + "}";
                dest = dest.replace(replace_key.as_str(), v);
            });

        let dest = match req.req.uri().query() {
            Some(query) => { dest + "?" + query }
            None => dest
        };

        debug!("DirectDispatcher Final Dest: {}", dest);
        exchange.context.destination = Some(dest);

        CONTINUE
    }
}

/// proxy request handler
pub struct AgentRequestHandler {}

impl Default for AgentRequestHandler {
    fn default() -> Self {
        AgentRequestHandler {}
    }
}

impl Handler for AgentRequestHandler {
    fn handle(&self, req: &Request, exchange: &mut Exchange) -> Option<FutureResponse> {
        let destination = exchange.context.destination.as_ref().unwrap().clone();

        Option::Some(Box::new(
            req.client
                .request(req.req.method().clone(), destination)
                .send_body(req.body.clone())
                .map_err(|e| {
                    error!("send request error {}", e);
                    RigError::WrapError(Box::new(e))
                })
                .map(|mut res| {
                    let mut client_resp = HttpResponse::build(res.status());
                    // Remove `Connection` as per
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
                    for (header_name, header_value) in
                        res.headers().iter().filter(|(h, _)| *h != "connection")
                        {
                            client_resp.header(header_name.clone(), header_value.clone());
                        }
                    res.body()
                        .into_stream()
                        .concat2()
                        .map(move |b| client_resp.body(b))
                        .map_err(|e| {
                            error!("payload error {}", e);
                            RigError::WrapError(Box::new(e))
                        })
                })
                .flatten()))
    }
}