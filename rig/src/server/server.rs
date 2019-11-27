use std::net::SocketAddr;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, web,
};
use futures::Future;
use futures::future::ok;
use log::info;

use crate::config::settings;
use crate::error::RigError;
use crate::handler::{Exchange, Handler, HandlerChain, Request};
use crate::handler::handlers::{AgentRequestHandler, DirectDispatcher, RouterHandler};

fn forward(req: HttpRequest,
           body: web::Bytes,
           router_handler: web::Data<RouterHandler>,
           direct_dispatcher: web::Data<DirectDispatcher>,
           agent_request_handler: web::Data<AgentRequestHandler>,
) -> impl Future<Item=HttpResponse, Error=RigError> {
    let mut exchange = Exchange::default();
    let mut handler_chain = HandlerChain::default();
    handler_chain.append(router_handler.get_ref())
        .append(direct_dispatcher.get_ref())
        .append(agent_request_handler.get_ref());
    exchange.handler_chain = handler_chain;
    exchange.handler_chain.first().handle(&Request::new(&req, &body), &exchange)
}

pub fn start_server() {
    let settings = &settings::SETTINGS;
    let port = settings.server.port;
    let addr = ("0.0.0.0:".to_owned() + port.to_string().as_str()).parse::<SocketAddr>().unwrap();

    info!("Listen at: {}", addr);

    HttpServer::new(
        || App::new()
            .data(DirectDispatcher::default())
            .data(AgentRequestHandler::default())
            .data(RouterHandler::new(&settings::APIS))
            .default_service(web::route().to_async(forward)))
        .bind(addr)
        .unwrap()
        .run()
        .unwrap()
}