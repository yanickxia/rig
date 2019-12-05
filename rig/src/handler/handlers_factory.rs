use std::collections::HashMap;

use crate::api::{Api, Definition};
use crate::config::settings;
use crate::handler::filters_factory::FilterFactory;
use crate::handler::Handler;
use crate::handler::handlers::{AgentRequestHandler, ComposeHandler, DirectDispatcher};

pub struct HandlerFactory {}

impl HandlerFactory {
    pub fn new(definition: &Definition) -> Box<dyn Handler> {
        let mut compose_handler = ComposeHandler::default();
        compose_handler.append(Box::new(DirectDispatcher::default()));
        compose_handler.append(Box::new(AgentRequestHandler::default()));

        Box::new(compose_handler)
    }
}