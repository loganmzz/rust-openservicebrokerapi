use actix_web::{HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    services: Vec<()>,
}

impl Catalog {
    fn new() -> Catalog {
        Catalog { services: vec![] }
    }

    pub fn services(&self) -> &Vec<()> {
        &self.services
    }
}

pub async fn get_catalog(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(Catalog::new())
}

#[cfg(test)]
mod tests {
    use actix_web::{http, test, dev::{ResponseBody, Body}};
    use actix_rt;

    #[actix_rt::test]
    async fn test_get_catalog() {
        let req = test::TestRequest::get()
                                    .uri("/v2/catalog")
                                    .to_http_request();
        let res = super::get_catalog(req).await;
        assert_eq!(res.status(), http::StatusCode::OK);
        let bytes = if let ResponseBody::Body(Body::Bytes(body)) = res.body() {
            body
        } else {
            panic!("Expected body type, but other was found");
        };
        let catalog: super::Catalog = match serde_json::from_slice(&bytes) {
            Result::Ok(value) => value,
            Result::Err(e) => panic!("{:?}", e),
        };
        assert_eq!(catalog.services().len(), 0);
    }
}
