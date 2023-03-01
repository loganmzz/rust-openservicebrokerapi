use openservicebroker as osb;

use actix_web::{test, App, http::StatusCode, body::{MessageBody}};

#[actix_web::test]
async fn ok() {
    let catalog = osb::service::SingleCatalogProvider::new(osb::model::Catalog::new());
    let mut app = test::init_service(
        App::new()
            .service(osb::new_scope("", Box::new(catalog)))
    ).await;
    let req = test::TestRequest::get().uri("/v2/catalog").to_request();
    let catalog: osb::model::Catalog = test::call_and_read_body_json(&mut app, req).await;
    assert_eq!(catalog.services().len(), 0);
}

#[actix_web::test]
async fn missing() {
    let catalog = osb::service::JsonFileCatalogProvider::new("tests/missing_catalog.json");
    let mut app = test::init_service(
        App::new()
            .service(osb::new_scope("", Box::new(catalog)))
    ).await;
    let req = test::TestRequest::get().uri("/v2/catalog").to_request();
    let res = test::call_service(&mut app, req).await;
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let bytes = res.into_body().try_into_bytes().expect("Expected body type, but other was found");
    if bytes.len() != 0 {
        panic!("Unexpected body ({:?})", bytes);
    }
}
