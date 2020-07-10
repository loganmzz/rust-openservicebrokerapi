use actix_web::{web, HttpRequest, HttpResponse};

pub mod model;
pub mod service;

pub fn new_scope(path: &str, catalog: service::CatalogProvider) -> actix_web::Scope {
    actix_web::Scope::new(path)
                     .data(catalog)
                     .route("/v2/catalog", web::get().to(get_catalog))
}

pub async fn get_catalog(_req: HttpRequest, data: web::Data<service::CatalogProvider>) -> HttpResponse {
    HttpResponse::Ok().json(data.get_catalog())
}

#[cfg(test)]
mod tests {
    use super::{model, service, get_catalog};
    use actix_web::{http, test, dev::{ResponseBody, Body}, web};
    use actix_rt;

    #[actix_rt::test]
    async fn test_get_catalog() {
        let req = test::TestRequest::get()
                                    .uri("/v2/catalog")
                                    .to_http_request();
        let provider = service::CatalogProvider::from_static(model::Catalog::new());
        let res = get_catalog(req, web::Data::new(provider)).await;
        assert_eq!(res.status(), http::StatusCode::OK);
        let bytes = if let ResponseBody::Body(Body::Bytes(body)) = res.body() {
            body
        } else {
            panic!("Expected body type, but other was found");
        };
        let catalog: model::Catalog = match serde_json::from_slice(&bytes) {
            Result::Ok(value) => value,
            Result::Err(e) => panic!("{:?}", e),
        };
        assert_eq!(catalog.services().len(), 0);
    }
}
