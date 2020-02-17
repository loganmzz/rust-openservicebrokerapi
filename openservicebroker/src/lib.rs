use actix_web::{HttpRequest, HttpResponse};

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

pub async fn get_catalog(_req: HttpRequest) -> HttpResponse<Catalog> {
    HttpResponse::Ok().message_body(Catalog::new())
}

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
            assert_eq!(body.services().len(), 0);
        } else {
            assert!(false, "Expected body type, but other was found");
        }
    }
}
