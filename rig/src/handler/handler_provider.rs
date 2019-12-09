use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};
use log::{error, info};

use crate::config::settings;
use crate::handler::router::RouterHandler;

lazy_static! {
    pub static ref HANDLER_PROVIDER : HandlerProvider = HandlerProvider::new();
}

pub struct HandlerProvider {
    pub current: Arc<RwLock<RouterHandler>>,
}

impl HandlerProvider {
    pub fn new_router_handler() -> RouterHandler {
        let apis = &settings::APIS;
        RouterHandler::new(&apis)
    }

    pub fn new() -> Self {
        HandlerProvider {
            current: Arc::new(RwLock::new(HandlerProvider::new_router_handler())),
        }
    }
}

pub fn scheduler_refresh_router_handler() -> JoinHandle<()> {
    let settings = &settings::SETTINGS;
    let interval: u64 = settings.tasks.api.interval;
    return thread::spawn(move || {
        loop {
            let current_arc = HANDLER_PROVIDER.current.clone();
            let lock = current_arc.write();
            match lock {
                Ok(mut rh) => {
                    info!("update route handler at {:?}", Utc::now());
                    *rh = HandlerProvider::new_router_handler();
                }
                Err(e) => {
                    error!("update router handler fail: {}", e)
                }
            }

            thread::sleep(Duration::from_secs(interval));
        }
    });
}