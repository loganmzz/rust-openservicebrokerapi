use actix_web::{web, HttpRequest, HttpResponse};

pub mod model;
pub mod service;

pub fn new_scope(path: &str, catalog: Box<dyn service::CatalogProvider>) -> actix_web::Scope {
    actix_web::Scope::new(path)
                     .app_data(web::Data::new(catalog))
                     .route("/v2/catalog", web::get().to(get_catalog))
}

pub async fn get_catalog(_req: HttpRequest, data: web::Data<Box<dyn service::CatalogProvider>>) -> HttpResponse {
    match data.get_catalog() {
        Ok(catalog) => HttpResponse::Ok().json(catalog),
        Err(error)  => {
            eprintln!("ERROR: {:?}", error);
            HttpResponse::InternalServerError().finish()
        },
    }

}

#[cfg(test)]
mod tests {
    use super::{model, service, get_catalog};
    use actix_web::{body::{MessageBody}, http, test, web};

    #[actix_web::test]
    async fn test_get_catalog() {
        let req = test::TestRequest::get()
                                    .uri("/v2/catalog")
                                    .to_http_request();
        let provider = service::SingleCatalogProvider::new(model::Catalog::new());
        let res = get_catalog(req, web::Data::new(Box::new(provider))).await;
        assert_eq!(res.status(), http::StatusCode::OK);
        let bytes = res.into_body().try_into_bytes().expect("Unable to get body bytes");
        let catalog: model::Catalog = match serde_json::from_slice(&bytes) {
            Result::Ok(value) => value,
            Result::Err(e) => panic!("{:?}", e),
        };
        assert_eq!(catalog.services().len(), 0);
    }

    #[actix_web::test]
    async fn test_get_catalog_missing() {

        let req = test::TestRequest::get()
                                    .uri("/v2/catalog")
                                    .to_http_request();
        let provider = service::JsonFileCatalogProvider::new("tests/missing_catalog.json");
        let res = get_catalog(req, web::Data::new(Box::new(provider))).await;
        assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        let bytes = res.into_body().try_into_bytes().expect("Expected body type, but other was found");
        if bytes.len() != 0 {
            panic!("Unexpected body ({:?})", bytes);
        }
    }
}
