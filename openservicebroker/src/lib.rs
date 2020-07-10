use actix_web::{HttpRequest, HttpResponse};
use std::borrow::Cow;

pub mod model;

struct CatalogProvider {
    catalog: model::Catalog,
}

impl CatalogProvider {
    fn from_static(catalog: model::Catalog) -> CatalogProvider {
        CatalogProvider {
            catalog,
        }
    }

    fn get_catalog(&self) -> Cow<model::Catalog> {
        Cow::Borrowed(&self.catalog)
    }
}

pub async fn get_catalog(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(model::Catalog::new())
}

#[cfg(test)]
mod tests {
    use super::{model, get_catalog};
    use actix_web::{http, test, dev::{ResponseBody, Body}};
    use actix_rt;

    fn build_catalog() -> model::Catalog {
        let mut catalog = model::Catalog::new();


        let mut mysql = model::Service::new();
        *mysql.id_mut() = "mysql".to_owned();
        *mysql.name_mut() = "MySQL".to_owned();

        let mut mysql_free = model::ServicePlan::new();
        *mysql_free.id_mut() = "mysql_free".to_owned();
        *mysql_free.name_mut() = "MySQL (Free)".to_owned();
        mysql.plans_mut().push(mysql_free);

        let mut mysql_small = model::ServicePlan::new();
        *mysql_small.id_mut() = "mysql_small".to_owned();
        *mysql_small.name_mut() = "MySQL (Small)".to_owned();
        mysql.plans_mut().push(mysql_small);

        catalog.services_mut().push(mysql);


        let mut pgsql = model::Service::new();
        *pgsql.id_mut() = "pgsql".to_owned();
        *pgsql.name_mut() = "PostgreSQL".to_owned();

        let mut pgsql_free = model::ServicePlan::new();
        *pgsql_free.id_mut() = "pgsql_free".to_owned();
        *pgsql_free.name_mut() = "PostgreSQL (Free)".to_owned();
        pgsql.plans_mut().push(pgsql_free);

        let mut pgsql_small = model::ServicePlan::new();
        *pgsql_small.id_mut() = "pgsql_small".to_owned();
        *pgsql_small.name_mut() = "PostgreSQL (Small)".to_owned();
        pgsql.plans_mut().push(pgsql_small);

        catalog.services_mut().push(pgsql);


        catalog
    }

    #[actix_rt::test]
    async fn test_get_catalog() {
        let req = test::TestRequest::get()
                                    .uri("/v2/catalog")
                                    .to_http_request();
        let res = get_catalog(req).await;
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

    #[test]
    fn catalog_provider_static() {
        let provider = super::CatalogProvider::from_static(build_catalog());

        let catalog  = provider.get_catalog();
        let mut services = catalog.services().iter();


        if let Some(mysql) = services.next() {
            assert_eq!("mysql", mysql.id(), "mysql.id");
            assert_eq!("MySQL", mysql.name(), "mysql.name");

            let mut plans = mysql.plans().iter();

            if let Some(mysql_free) = plans.next() {
                assert_eq!("mysql_free", mysql_free.id(), "mysql.plans.free.id");
                assert_eq!("MySQL (Free)", mysql_free.name(), "mysql.plans.free.name");
            } else {
                panic!("Missing MySQL Free plan");
            }

            if let Some(mysql_small) = plans.next() {
                assert_eq!("mysql_small", mysql_small.id(), "mysql.plans.small.id");
                assert_eq!("MySQL (Small)", mysql_small.name(), "mysql.plans.small.name");
            }

            assert!(plans.next().is_none(), "mysql.plans.end");
        } else {
            panic!("Missing MySQL service");
        }


        if let Some(pgsql) = services.next() {
            assert_eq!("pgsql", pgsql.id(), "pgsql.id");
            assert_eq!("PostgreSQL", pgsql.name(), "pgsql.name");

            let mut plans = pgsql.plans().iter();

            if let Some(pgsql_free) = plans.next() {
                assert_eq!("pgsql_free", pgsql_free.id(), "pgsql.plans.free.id");
                assert_eq!("PostgreSQL (Free)", pgsql_free.name(), "pgsql.plans.free.name");
            } else {
                panic!("Missing PostgreSQL Free plan");
            }

            if let Some(pgsql_small) = plans.next() {
                assert_eq!("pgsql_small", pgsql_small.id(), "pgsql.plans.small.id");
                assert_eq!("PostgreSQL (Small)", pgsql_small.name(), "pgsql.plans.small.name");
            }

            assert!(plans.next().is_none(), "pgsql.plans.end");
        } else {
            panic!("Missing PostgreSQL service");
        }


        assert!(services.next().is_none(), "services.end");
    }
}
