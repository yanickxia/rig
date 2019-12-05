use std::net::SocketAddr;
use std::sync::{Arc, Mutex, RwLock};

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, web,
};
use futures::Future;
use futures::future::{err, ok};
use log::info;

use crate::api::Api;
use crate::config::settings;
use crate::error::RigError;
use crate::handler::{Exchange, Handler, Request};
use crate::handler::handlers::{AgentRequestHandler, ComposeHandler, DirectDispatcher};
use crate::handler::router::RouterHandler;

fn forward(req: HttpRequest,
           body: web::Bytes,
           share_handler: web::Data<Arc<RwLock<RouterHandler>>>,
) -> impl Future<Item=HttpResponse, Error=RigError> {
    let handler = share_handler.get_ref().clone();
    let route_handler = handler.read().unwrap();
    let mut exchange = Exchange::default();
    match route_handler.handle(&Request::new(&req, &body), &mut exchange) {
        Some(result) => {
            result
        }
        None => {
            Box::new(ok(HttpResponse::InternalServerError().finish()))
        }
    }
}

pub fn start_server() {
    let settings = &settings::SETTINGS;
    let api = &settings::APIS;
    let port = settings.server.port;
    let addr = ("0.0.0.0:".to_owned() + port.to_string().as_str()).parse::<SocketAddr>().unwrap();
    info!("Listen at: {}", addr);

    HttpServer::new(
        || App::new()
            .data(shared_handler())
            .default_service(web::route().to_async(forward)))
        .bind(addr)
        .unwrap()
        .run()
        .unwrap()
}

fn shared_handler() -> Arc<RwLock<RouterHandler>> {
    let apis = &settings::APIS;
    let router_handler = RouterHandler::new(&apis);
    Arc::new(RwLock::new(router_handler))
}