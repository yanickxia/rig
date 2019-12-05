use futures::Future;

use crate::api::{Api, Definition};
use crate::error::RigError;
use crate::handler::{Exchange, Filter, FutureResponse, Handler, Request};

/// ComposeFilter
pub struct ComposeFilter {
    filters: Vec<Box<dyn Filter>>,
    definition: Definition,
}

impl Filter for ComposeFilter {
    fn filter(&self, req: &Request, exchange: &mut Exchange) -> bool {
        if self.filters.is_empty() {
            exchange.context.definition = Some(self.definition.clone());
            return true;
        }
        return self.filters.iter()
            .any(|f| f.filter(req, exchange));
    }
}

impl ComposeFilter {
    pub fn new(filters: Vec<Box<dyn Filter>>, definition: &Definition) -> Self {
        ComposeFilter {
            filters,
            definition: definition.clone(),
        }
    }
}

/// Method Filter
pub struct MethodFilter {
    method: String,
}

impl MethodFilter {
    pub fn new(method: &str) -> Self {
        MethodFilter {
            method: method.to_string()
        }
    }
}

impl Filter for MethodFilter {
    fn filter(&self, req: &Request, exchange: &mut Exchange) -> bool {
        req.req.method().eq(self.method.as_str())
    }
}

pub struct FixtureFilter {}

impl Filter for FixtureFilter {
    fn filter(&self, req: &Request, exchange: &mut Exchange) -> bool {
        true
    }
}

impl Default for FixtureFilter {
    fn default() -> Self {
        FixtureFilter {}
    }
}

