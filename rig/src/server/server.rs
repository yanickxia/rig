use std::borrow::Borrow;
use std::net::SocketAddr;
use std::ops::Deref;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, web,
};
use futures::Future;
use futures::future::ok;
use log::{debug, info};

use crate::config::settings;
use crate::error::RigError;
use crate::handler::{Exchange, Handler, HandlerChain, Request};
use crate::handler::handlers::{AgentRequestHandler, DirectDispatcher, RouterHandler};
use crate::handler::handlers_factory::HandlerFactory;

fn forward(req: HttpRequest,
           body: web::Bytes,
           handler_factory: web::Data<HandlerFactory>,
) -> impl Future<Item=HttpResponse, Error=RigError> {

    let mut handler_chain = HandlerChain::default();
    let factory =handler_factory.get_ref();

    handler_chain
        .append(*factory.get(std::any::type_name::<RouterHandler>()).as_ref())
        .append(*factory.get(std::any::type_name::<DirectDispatcher>()).as_ref())
        .append(*factory.get(std::any::type_name::<AgentRequestHandler>()).as_ref());

    let mut exchange = Exchange::new(&handler_chain);
    exchange.handler_chain.first().handle(&Request::new(&req, &body), &exchange)
}

pub fn start_server() {
    let settings = &settings::SETTINGS;
    let port = settings.server.port;
    let addr = ("0.0.0.0:".to_owned() + port.to_string().as_str()).parse::<SocketAddr>().unwrap();
    info!("Listen at: {}", addr);

    HttpServer::new(
        || App::new()
            .data(HandlerFactory::default())
            .default_service(web::route().to_async(forward)))
        .bind(addr)
        .unwrap()
        .run()
        .unwrap()
}