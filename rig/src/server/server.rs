use std::net::SocketAddr;
use std::sync::{Arc, mpsc, RwLock};
use std::thread;
use std::time::Duration;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, web,
};
use actix_web::client::Client;
use futures::Future;
use futures::future::ok;
use log::info;

use crate::config::settings;
use crate::error::RigError;
use crate::handler::{Exchange, Handler, Request};
use crate::handler::handler_provider;
use crate::handler::router::RouterHandler;

fn forward(req: HttpRequest,
           body: web::Bytes,
           client: web::Data<Client>,
) -> impl Future<Item=HttpResponse, Error=RigError> {
    let handler = &handler_provider::HANDLER_PROVIDER;
    let route_handler = handler.current.read().unwrap();
    let mut exchange = Exchange::default();
    match route_handler.handle(&Request::new(&req, &body, client.get_ref()), &mut exchange) {
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
    let port = settings.server.port;
    let addr = ("0.0.0.0:".to_owned() + port.to_string().as_str()).parse::<SocketAddr>().unwrap();
    info!("Listen at: {}", addr);

    let _ = handler_provider::scheduler_refresh_router_handler();

    HttpServer::new(
        || App::new()
            .data(Client::default())
            .default_service(web::route().to_async(forward)))
        .bind(addr)
        .unwrap()
        .run()
        .unwrap();
}