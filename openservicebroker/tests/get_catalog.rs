use openservicebroker as osb;

use actix_web::{test, App};

#[actix_rt::test]
async fn main() {
    let mut app = test::init_service(
        App::new()
            .service(
                osb::new_scope(
                    "",
                    osb::service::CatalogProvider::from_static(osb::model::Catalog::new())
                )
            )
    ).await;
    let req = test::TestRequest::get().uri("/v2/catalog").to_request();
    let catalog: osb::model::Catalog = test::read_response_json(&mut app, req).await;
    assert_eq!(catalog.services().len(), 0);
}
