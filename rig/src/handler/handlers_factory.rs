use std::collections::HashMap;
use std::sync::{Arc, RwLock,Mutex};

use crate::config::settings;
use crate::handler::{Handler, HandlerChain};
use crate::handler::handlers::{AgentRequestHandler, DirectDispatcher, RouterHandler};

pub struct HandlerFactory {
    handlers: HashMap<&'static str, Box<dyn Handler>>
}

impl HandlerFactory {
    pub fn get(&self, name: &str) -> Box<&dyn Handler> {
        Box::new(self.handlers.get(name).unwrap().as_ref())
    }
}

impl Default for HandlerFactory {
    fn default() -> Self {
        let mut handlers: HashMap<&'static str, Box<dyn Handler>> = HashMap::new();

        let _ = handlers.insert(std::any::type_name::<DirectDispatcher>(), Box::new(DirectDispatcher::default()));
        let _ = handlers.insert(std::any::type_name::<AgentRequestHandler>(), Box::new(AgentRequestHandler::default()));
        let _ = handlers.insert(std::any::type_name::<RouterHandler>(), Box::new(RouterHandler::new(&settings::APIS)));

        HandlerFactory {
            handlers
        }
    }
}

