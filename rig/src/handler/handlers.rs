use std::convert::TryFrom;

use actix_router::{Path, ResourceDef, Router};
use actix_web::{HttpRequest, HttpResponse};
use actix_web::client::Client;
use awc::ClientResponse;
use futures::{Future, Stream};
use futures::future::{err, ok};
use log::{debug, info};

use crate::api::Api;
use crate::error::RigError;
use crate::handler::{Exchange, Handler, Request};

pub struct RouterHandler<'a> {
    routers: Router<&'a Api>
}

/// Router handler
impl<'a> RouterHandler<'a> {
    pub fn new(apis: &'a Vec<Api>) -> Self {
        let mut router_builder = Router::<&Api>::build();

        apis.iter().for_each(
            |it| {
                let v = u16::try_from(it.id).unwrap();
                router_builder.path(it.path.as_str(), it).0.set_id(v);
            });

        RouterHandler {
            routers: router_builder.finish()
        }
    }
}


impl Default for RouterHandler<'_> {
    fn default() -> Self {
        let router_builder = Router::<&Api>::build();
        RouterHandler {
            routers: router_builder.finish()
        }
    }
}


impl Handler for RouterHandler<'_> {
    fn handle(&self, req: &Request, context: &Exchange) -> Box<dyn Future<Item=HttpResponse, Error=RigError>> {
        let mut path = Path::new(req.req.path());
        let found_route = self.routers.recognize(&mut path);

        debug!("router handler process, path: {}, matched: {}", req.req.path(), found_route.is_some());

        if found_route.is_none() {
            return Box::new(err(RigError::NotFoundPath));
        }

        *context.api.borrow_mut() = Option::Some((*(found_route.unwrap().0)).clone());
        context.handler_chain.handle(req, context)
    }
}

/// DirectDispatcher
pub struct DirectDispatcher {
    pub env_destination: Option<String>,
}

impl Default for DirectDispatcher {
    fn default() -> Self {
        DirectDispatcher {
            env_destination: option_env!("rig_destination")
                .map(|it| it.to_string())
        }
    }
}

impl Handler for DirectDispatcher {
    fn handle(&self, req: &Request, exchange: &Exchange) -> Box<dyn Future<Item=HttpResponse, Error=RigError>> {
        let api = exchange.api.borrow_mut();
        let dest = (*api).as_ref().unwrap().path.as_str();
        let resource_def = ResourceDef::new(dest);
        let mut path = Path::new(req.req.path());
        let _ = resource_def.match_path(&mut path);

        let mut dest = match &self.env_destination {
            Some(it) => it.clone(),
            None => (*api).as_ref().unwrap().destination.destination.to_string(),
        };

        path.iter()
            .for_each(|it| {
                let (k, v) = it;
                let replace_key = "{".to_owned() + k + "}";
                dest = dest.replace(replace_key.as_str(), v);
            });

        debug!("DirectDispatcher Final Dest: {}", dest);
        exchange.context.borrow_mut().destination = Option::Some(dest);
        exchange.handler_chain.handle(req, exchange)
    }
}

/// proxy request handler
pub struct AgentRequestHandler {
    client: Client,
}

impl Default for AgentRequestHandler {
    fn default() -> Self {
        AgentRequestHandler { client: Default::default() }
    }
}

impl Handler for AgentRequestHandler {
    fn handle(&self, req: &Request, exchange: &Exchange) -> Box<dyn Future<Item=HttpResponse, Error=RigError>> {
        let context = exchange.context.borrow_mut();
        let destination = context.destination.as_ref().unwrap();

        Box::new(self.client.request(req.req.method().clone(), destination)
            .send_body(req.body.clone())
            .map_err(|_| RigError::AgentRequest)
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
                    .map_err(|e| RigError::AgentResponse)
            })
            .flatten())
    }
}