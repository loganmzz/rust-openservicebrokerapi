#[cfg(test)]
mod tests {
    use actix_web::{http, test, dev::ResponseBody};
    use actix_rt;

    #[actix_rt::test]
    async fn test_get_catalog() {
        let req = test::TestRequest::get()
                                    .uri("/v2/catalog")
                                    .to_http_request();
        let res = super::get_catalog(req).await;
        assert_eq!(res.status(), http::StatusCode::OK);
        if let ResponseBody::Body(body) = res.body() {
            assert_eq!(body.services.len(), 0);
        } else {
            assert!(false, "Expected body type, but other was found");
        }
    }
}
