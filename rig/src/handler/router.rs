use std::convert::TryFrom;

use actix_router::{Path, ResourceDef, Router};
use log::{debug, error, info};

use crate::api::{Api, Definition};
use crate::error::RigError;
use crate::handler::{CONTINUE, error_response, Exchange, Filter, FutureResponse, Handler, Request};
use crate::handler::filters::ComposeFilter;
use crate::handler::filters_factory::FilterFactory;
use crate::handler::handlers_factory::HandlerFactory;

pub struct Processor {
    api: Api,
    handlers: Vec<(Box<dyn Filter>, Box<dyn Handler>)>,
}


pub struct RouterHandler {
    routers: Router<Processor>
}

/// Router handler
impl RouterHandler {
    pub fn new(apis: &Vec<Api>) -> Self {
        let mut router_builder = Router::<Processor>::build();

        apis.iter().for_each(
            |it| {
                let id = u16::try_from(it.id).unwrap();
                let processor = Processor { api: it.clone(), handlers: RouterHandler::new_handlers(it) };
                router_builder.path(it.path.as_str(), processor).0.set_id(id);
            });

        RouterHandler {
            routers: router_builder.finish()
        }
    }

    fn new_handlers(api: &Api) -> Vec<(Box<dyn Filter>, Box<dyn Handler>)> {
        api.handlers.iter()
            .map(|def| RouterHandler::new_handler(def))
            .collect()
    }

    fn new_handler(definition: &Definition) -> (Box<dyn Filter>, Box<dyn Handler>) {
        (FilterFactory::new(definition), HandlerFactory::new(definition))
    }
}


impl Default for RouterHandler {
    fn default() -> Self {
        let router_builder = Router::<Processor>::build();
        RouterHandler {
            routers: router_builder.finish()
        }
    }
}


impl Handler for RouterHandler {
    fn handle(&self, req: &Request, exchange: &mut Exchange) -> Option<FutureResponse> {
        let mut path = Path::new(req.req.path());
        let found_route = self.routers.recognize(&mut path);

        debug!("router handler process, path: {}, matched: {}", req.req.path(), found_route.is_some());

        if found_route.is_none() {
            return error_response(RigError::NotFoundPath);
        }

        exchange.api = Option::Some(found_route.unwrap().0.api.clone());

        let handler =
            match found_route.unwrap().0.handlers.iter()
                .find(|it| it.0.filter(req, exchange))
                .map(|it| it.1.as_ref())
                {
                    Some(h) => { h }
                    None => {
                        return error_response(RigError::NotMatchedFilters);
                    }
                };

        handler.handle(req, exchange)
    }
}