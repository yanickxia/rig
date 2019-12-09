use std::cell::RefCell;

use rand::prelude::*;

use crate::api::Definition;
use crate::handler::{Exchange, Filter, Request};

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
///
/// base on http method
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
    fn filter(&self, req: &Request, _exchange: &mut Exchange) -> bool {
        req.req.method().eq(self.method.as_str())
    }
}

/// FixtureFilter
///
/// fixture filter always true
pub struct FixtureFilter {}

impl Filter for FixtureFilter {
    fn filter(&self, _req: &Request, _exchange: &mut Exchange) -> bool {
        true
    }
}

impl Default for FixtureFilter {
    fn default() -> Self {
        FixtureFilter {}
    }
}

/// Random filter
pub struct RandomFilter {
    percentage: u8,
}

impl RandomFilter {
    fn new(percentage: u8) -> Self {
        RandomFilter {
            percentage,
        }
    }
}

impl Filter for RandomFilter {
    fn filter(&self, _req: &Request, _exchange: &mut Exchange) -> bool {
        let random: u8 = rand::random();
        random % 100 <= self.percentage
    }
}