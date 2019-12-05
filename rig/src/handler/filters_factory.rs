use std::collections::HashMap;

use crate::api::Definition;
use crate::api::FilterType::Method;
use crate::config::settings;
use crate::handler::Filter;
use crate::handler::filters::{ComposeFilter, FixtureFilter, MethodFilter};

pub struct FilterFactory {}

impl FilterFactory {
    pub fn new(definition: &Definition) -> Box<dyn Filter> {
        let filters = definition.filters.iter()
            .map(|f| {
                match f.name {
                    Method => { Box::new(MethodFilter::new(f.method.as_str())) as Box<dyn Filter>}
                    _ => { Box::new(FixtureFilter::default()) as Box<dyn Filter> }
                }
            })
            .collect::<Vec<Box<dyn Filter>>>();

        Box::new(ComposeFilter::new(filters, definition))
    }
}
