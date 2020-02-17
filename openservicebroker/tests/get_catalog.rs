use openservicebroker as osb;

use actix_web::{test, web, App};

#[actix_rt::test]
async fn main() {
    let mut app = test::init_service(
        App::new()
            .route("/v2/catalog", web::get().to(osb::get_catalog)),
    ).await;
    let req = test::TestRequest::get().uri("/v2/catalog").to_request();
    let catalog: osb::Catalog = test::read_response_json(&mut app, req).await;
    assert_eq!(catalog.services().len(), 0);
}
