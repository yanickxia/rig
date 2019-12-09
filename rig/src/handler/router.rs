use std::convert::TryFrom;

use actix_router::{Path, Router};
use log::{debug, error, info};

use crate::api::{Api, Definition};
use crate::error::RigError;
use crate::handler::{error_response, Exchange, Filter, FutureResponse, Handler, Request};
use crate::handler::filters_factory::FilterFactory;
use crate::handler::handlers_factory::HandlerFactory;
use crate::http::path::PathParser;

pub struct Processor {
    pub api: Api,
    path_parser: PathParser,
    handlers: Vec<(Box<dyn Filter>, Box<dyn Handler>)>,
}


pub struct RouterHandler {
    routers: Vec<Processor>
}

/// Router handler
impl RouterHandler {
    pub fn new(apis: &Vec<Api>) -> Self {
        RouterHandler {
            routers: apis.iter().map(
                |it| {
                    Processor {
                        api: it.clone(),
                        path_parser: PathParser::new(it.path.as_str()),
                        handlers: RouterHandler::new_handlers(it),
                    }
                }).collect()
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
        RouterHandler {
            routers: vec![]
        }
    }
}


impl Handler for RouterHandler {
    fn handle(&self, req: &Request, exchange: &mut Exchange) -> Option<FutureResponse> {
        let request_path = req.req.path();
        let found_result = self.routers.iter()
            .find(|processor| {
                let resolved = processor.path_parser.parse(request_path);
                if resolved.0 {
                    exchange.resolved_path_variables = Some(resolved.1);
                    return true;
                }
                return false;
            });

        debug!("router handler process, path: {}, matched: {}", request_path, found_result.is_some());

        if found_result.is_none() {
            return error_response(RigError::NotFoundPath);
        }

        exchange.api = Some(found_result.unwrap().api.clone());

        let handler =
            match found_result.unwrap().handlers.iter()
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