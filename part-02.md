## Let's design catalog

The Open Service Broker API is based around three major concepts: service, instance and binding. Listing provided services is made through `/v2/catalog` endpoint. Let's first design ~~entities~~ structs ([spec](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/spec.md#service-offering-object)):

```rust
struct Catalog {
    services: Vec<Service>,
}

struct Service {
    name: String,
    id: String,
    description: String,
    tags: Vec<String>,
    requires: Vec<String>,
    bindable: bool,
    instances_retrievable: Option<bool>,
    bindings_retrievable: Option<bool>,
    allow_context_updates: Option<bool>,
    metadata: HashMap<String, String>,
    // dashboard_client: Option<DashboardClient>,
    plan_updateable: Option<bool>,
    plans: Vec<ServicePlan>,
}

struct ServicePlan {
    id: String,
    name: String,
    description: String,
    metadata: HashMap<String, String>,
    free: Option<bool>,
    bindable: Option<bool>,
    plan_updateable: Option<bool>,
    // schemas: Option<Schemas>,
    maximum_polling_duration: Option<u64>,
    // maintenance_info: Option<MaintenanceInfo>,
}
```

_Notes:_

* Accessor and mutator aren't shown there. They can be added as structs are coded, but can also be added during refactoring steps.
* _[Service metadata](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/profile.md#service-metadata) is supposed to be free-form object but to keep it simple, it's implemented as simple key-value string pairs._
* _[`DashboardClient`](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/profile.md#dashboard-client-object) is not implemented._
* _[`Schemas`](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/spec.md#schemas-object) is not implemented._
* _[`MaintenanceInfo`](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/spec.md#maintenance-info-object) is not implemented._
* _DON'T FORGET TO RE-RUN TEST !_

OK, that's a nice struct design but don't forget JSON support ! As previously, just add `#[derive(Serialize, Deserialize)]` from [Serde](https://serde.rs/) to each struct.

In order to make things cleaner, let's move all those structs into a `model` sub-module.


## Let's define Catalog

Many sources/ways to define Catalog should be supported. Let's begin providing an implementation based on a "static" configuration. As usual, a test must be added first:

```rust
// src/lib.rs
mod tests {
    use super::{model, CatalogProvider};

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

    #[test]
    fn catalog_provider_static() {
        let provider = CatalogProvider::from_static(build_catalog());

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
```

What may `fn CatalogProvider::get_catalog()` look like ? In fact, test usage works with both owned or borrowed instances. So, how to choose between both ? There is no universal response. Providing borrowed instances doesn't prevent user to get their owned instance if they [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html). However, it means borrow lifetime is bound to provider one and must be owned by it. Such, if a new catalog instance is created at each call, provider must still keep ownership of the instance.

So, no flexible solutions ? Yes, there are. To be honest ownership or borrowing are not strictly required, that's [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) ! Such structs like [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) or [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) implement it. This is this last one that will be used for its flexibility:

```rust
// src/lib.rs
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
```

But now, `Catalog` must support [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html):

```rust
// src/model.rs
#[derive(Clone)]
struct Catalog;

#[derive(Clone)]
struct Service;

#[derive(Clone)]
struct ServicePlan;
```


## Let's load Catalog

For developer that can hardcode Catalog, it's fine but if they want to load catalog from their environment ? using a file for example ? Let's support that !

First, previous test will be reused:

```rust
// src/lib.rs
mod tests {

    fn check_catalog_provider(provider: CatalogProvider) {
        let catalog  = provider.get_catalog();
        // ...
    }

    #[test]
    fn catalog_provider_static() {
        let provider = CatalogProvider::from_static(build_catalog());
        check_catalog_provider(provider);
    }

    #[test]
    fn catalog_provider_file_json() {
        let provider = CatalogProvider::from_file_json("tests/default_catalog.json");
        check_catalog_provider(provider);
    }

}
```

And the test file:

```json
{
    "services": [
        {
            "id": "mysql",
            "name": "MySQL",
            "description": "",
            "tags": [],
            "requires": [],
            "bindable": true,
            "metadata": {},
            "plans": [
                {
                    "id": "mysql_free",
                    "name": "MySQL (Free)",
                    "description": "",
                    "metadata": {}
                },
                {
                    "id": "mysql_small",
                    "name": "MySQL (Small)",
                    "description": "",
                    "metadata": {}
                }
            ]
        },
        {
            "id": "pgsql",
            "name": "PostgreSQL",
            "description": "",
            "tags": [],
            "requires": [],
            "bindable": true,
            "metadata": {},
            "plans": [
                {
                    "id": "pgsql_free",
                    "name": "PostgreSQL (Free)",
                    "description": "",
                    "metadata": {}
                },
                {
                    "id": "pgsql_small",
                    "name": "PostgreSQL (Small)",
                    "description": "",
                    "metadata": {}
                }
            ]
        }
    ]
}
```

And the implementation:

```rust
// src/lib.rs
impl CatalogProvider {
    // ...
    fn from_file_json(path: &str) -> CatalogProvider {
        let file = std::fs::File::open(path).expect(&format!("File '{}' not found", path));
        let catalog: model::Catalog = serde_json::from_reader(file).expect(&format!("Invalid JSON file '{}'", path));
        Self::from_static(catalog)
    }
}
```

_Note: Error management in this implementation doesn't reflect good practices. Better solution coming soon !_


Then just refactor and move `CatalogProvider` to a `service` module.


## Let's integrate provider

`fn get_catalog()` handler is still always returning a fresh empty catalog on each call. First, adapt tests:

```rust
// src/lib.rs
mod tests {
    use actix_web::web;

    async fn test_get_catalog() {
        // ...
        let provider = service::CatalogProvider::from_static(model::Catalog::new());
        let res = get_catalog(req, web::Data::new(provider)).await;
        // ...
    }
}


// tests/get_catalog.rs
async fn main() {
    // ...
        App::new()
            .data(osb::service::CatalogProvider::from_static(osb::model::Catalog::new()))
    // ...
}

// src/bin/dummy-servicebroker.rs
async fn main() -> std::io::Result<()> {
    // ...
        App::new()
            .data(osb::service::CatalogProvider::from_static(osb::model::Catalog::new()))
    // ...
}
```

Then, adapt implementation:

```rust
// src/lib.rs
pub async fn get_catalog(_req: HttpRequest, data: web::Data<service::CatalogProvider>) -> HttpResponse {
    HttpResponse::Ok().json(data.get_catalog())
}
```

Re-run test to check everything works fine, then adapt `dummy-servicebroker` binary to load default catalog !

Let's refactor a little bit more by defining a [`Scope`](https://docs.rs/actix-web/2.0.0/actix_web/struct.Scope.html):

```rust
// src/lib.rs
pub fn new_scope(path: &str, catalog: service::CatalogProvider) -> actix_web::Scope {
    actix_web::Scope::new(path)
                     .data(catalog)
                     .route("/v2/catalog", web::get().to(get_catalog))
}

// tests/get_catalog.rs
        App::new()
            .service(
                osb::new_scope(
                    "",
                    osb::service::CatalogProvider::from_static(osb::model::Catalog::new())
                )
            )


// src/bin/dummy-servicebroker.rs
        App::new()
            .service(
                osb::new_scope(
                    "",
                    osb::service::CatalogProvider::from_file_json("tests/default_catalog.json")
                )
            )
```
