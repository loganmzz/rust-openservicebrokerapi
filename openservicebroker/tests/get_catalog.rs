use openservicebroker as osb;

use actix_web::{test, App, http::StatusCode, body::{Body, ResponseBody}};

#[actix_rt::test]
async fn ok() {
    let catalog = osb::service::SingleCatalogProvider::new(osb::model::Catalog::new());
    let mut app = test::init_service(
        App::new()
            .service(osb::new_scope("", Box::new(catalog)))
    ).await;
    let req = test::TestRequest::get().uri("/v2/catalog").to_request();
    let catalog: osb::model::Catalog = test::read_response_json(&mut app, req).await;
    assert_eq!(catalog.services().len(), 0);
}

#[actix_rt::test]
async fn missing() {
    let catalog = osb::service::JsonFileCatalogProvider::new("tests/missing_catalog.json");
    let mut app = test::init_service(
        App::new()
            .service(osb::new_scope("", Box::new(catalog)))
    ).await;
    let req = test::TestRequest::get().uri("/v2/catalog").to_request();
    let mut res = test::call_service(&mut app, req).await;
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    match res.take_body() {
        ResponseBody::Body(body)  => match body {
            Body::Empty => (),
            _           => panic!("Unexpected body type ({:?})", body),
        },
        ResponseBody::Other(body) => panic!("Found response body of type other ({:?})", body),
    };
}
