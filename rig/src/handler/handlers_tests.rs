extern crate downcast_rs;

use actix_web::{HttpRequest, HttpResponse};
use futures::Future;
use futures::future::ok;

use crate::error::RigError;
use crate::handler::{Exchange, Handler};

pub struct TestEndHandler {}

impl Handler for TestEndHandler {
    fn handle(&self, _req: &HttpRequest, _context: &Exchange) -> Box<dyn Future<Item=HttpResponse, Error=RigError>> {
        Box::new(ok(HttpResponse::Ok().finish()))
    }
}

impl Default for TestEndHandler {
    fn default() -> Self {
        TestEndHandler {}
    }
}


#[cfg(test)]
mod route_handler_test {
    use actix_web::test::*;
    use futures::future::Future;

    use crate::error::RigError;
    use crate::handler::{Exchange, Handler};
    use crate::handler::handlers::RouterHandler;
    use crate::handler::handlers_tests::TestEndHandler;

    #[test]
    fn test_notfound_route_handle() {
        let router_handler = RouterHandler::default();

        let request = TestRequest::default()
            .uri("/test")
            .to_http_request();

        let context = Exchange::default();

        let result = router_handler.handle(&request, &context).wait();

        assert_eq!(RigError::NotFoundPath, result.err().unwrap());
    }

    #[test]
    fn test_found_route() {
        let router_handler = RouterHandler::builder()
            .append("/test".to_string(), "GET".to_string())
            .finish();

        let request = TestRequest::default()
            .uri("/test")
            .to_http_request();

        let end_handler = TestEndHandler {};
        let mut context = Exchange::default();
        context.handler_chain.append(&end_handler);

        let _ = router_handler.handle(&request, &context).wait();
        assert!(context.api.borrow().is_some())
    }
}

#[cfg(test)]
mod direct_dispatcher_handler_test {
    use actix_web::test::*;
    use futures::future::Future;

    use crate::api::Api;
    use crate::error::RigError;
    use crate::handler::{Exchange, Handler};
    use crate::handler::handlers::DirectDispatcher;
    use crate::handler::handlers_tests::TestEndHandler;

    #[test]
    fn test_dispatcher() {
        let dispatcher = DirectDispatcher::new();
        let request = TestRequest::default()
            .uri("/test/123")
            .to_http_request();

        let end_handler = TestEndHandler {};
        let mut context = Exchange::default();
        context.handler_chain.append(&end_handler);
        *context.api.borrow_mut() = Option::Some(Api::builder()
            .path("/test/{version}")
            .dispatcher("/other/{version}")
            .finish());

        let _ = dispatcher.handle(&request, &context).wait();

        let c = &(*context.context.borrow_mut()).destination;

        assert_eq!(c.as_ref().unwrap().as_str(), "/other/123")

    }
}