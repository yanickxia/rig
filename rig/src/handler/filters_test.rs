#[cfg(test)]
mod method_filter_test {
    use actix_web::test::TestRequest;

    use crate::handler::{Exchange, Filter, Request};

    use super::super::filters::MethodFilter;

    #[test]
    fn test() {
        let m = MethodFilter::new("GET");
        let request = TestRequest::get()
            .to_http_request();
        let req = Request { req: &request, body: &Default::default() };
        let mut exchange = Exchange::default();
        assert!(m.filter(&req, &mut exchange))
    }
}