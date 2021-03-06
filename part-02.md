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

##  Let's handle the errors

Before going further with `CatalogProvider`, error handling must be improved. For the moment [`expect(..)`](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect) is used but it panics on failure, causing a fatal error (aka a "crash") of the application with no chance to:

* provide a detailed message to final user (not developer using library),
* let developers catch errors and handling them as they wish.

In Rust, [`Result` struct](https://doc.rust-lang.org/std/result/enum.Result.html) and [`Error` trait](https://doc.rust-lang.org/std/error/trait.Error.html) are used for catchable failures. A very simple improvement can be:

```rust
// src/service.rs
    pub fn from_file_json(path: &str) -> Result<CatalogProvider, Box<dyn Error + 'static>> {
        let file = std::fs::File::open(path)?;
        let catalog: model::Catalog = serde_json::from_reader(file)?;
        Ok(Self::from_static(catalog))
    }

    #[test]
    fn catalog_provider_file_json() {
        let provider = CatalogProvider::from_file_json("tests/default_catalog.json").expect("catalog load failed");
        check_catalog_provider(provider);
    }

    // new !
    #[test]
    fn catalog_provider_file_json_missing() {
        let error = CatalogProvider::from_file_json("tests/missing_catalog.json").err().expect("catalog load MUST fail");
        let ioerror = error.downcast_ref::<std::io::Error>().expect("catalog load error must be an I/O one");
        assert_eq!(std::io::ErrorKind::NotFound, ioerror.kind());
    }

// src/bin/dummy-servicebroker.rs
                osb::new_scope(
                    "",
                    osb::service::CatalogProvider::from_file_json("tests/default_catalog.json")
                                                  .expect("Error on loading default catalog")
                )
```

Due to some limitations of Actix (closure factory can't fail and may be called many times), changes to our dummy Service Broker is more important:

```rust
// src/service.rs
#[derive(Clone)]
pub struct CatalogProvider

// src/bin/dummy-servicebroker.rs
#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let catalog = osb::service::CatalogProvider::from_file_json("tests/default_catalog.json")?;
    HttpServer::new(move || {
        App::new()
            .service(osb::new_scope("", catalog.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
```

Using `Box<dyn Error + 'static>` can be enough but to display context, you still need to encapsulate errors with your own types. Otherwise, consumers (i.e. developers) will have to dig into error stack recursively using [`source`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source).

[`anyhow` crate](https://crates.io/crates/anyhow) provides facilities to help on the topic. First add it to `Cargo.toml`:

```toml
[dependencies]
anyhow = "1.0.32"
```

Then, replace `Result<T, Box<dyn Error + 'static>>` by `anyhow::Result<T>`:

```rust
// src/service.rs
use anyhow::Result;

    pub fn from_file_json(path: &str) -> Result<CatalogProvider> {
        let file = std::fs::File::open(path)?;
        let catalog: model::Catalog = serde_json::from_reader(file)?;
        Ok(Self::from_static(catalog))
    }

// src/bin/dummy-servicebroker.ts
use anyhow::Result;

async fn main() -> Result<()> {
    // ...
}
```

_Note: You may notice that except type, no other changes are required !_

Finally, adds [`Context`](https://docs.rs/anyhow/1.0.32/anyhow/trait.Context.html) to fallible code:

```rust
// src/service.rs
use anyhow::Context;

    pub fn from_file_json(path: &str) -> Result<CatalogProvider> {
        let file = std::fs::File::open(path)
                                 .with_context(|| format!("Access to catalog file '{}' has failed", path))?;
        let catalog: model::Catalog = serde_json::from_reader(file)
                                                 .with_context(|| format!("Can't read catalog file '{}' as JSON", path))?;
        Ok(Self::from_static(catalog))
    }

// src/bin/dummy-servicebroker.ts
use anyhow::Context;

async fn main() -> Result<()> {
    let catalog = osb::service::CatalogProvider::from_file_json("tests/default_catalog.json")
                                                .with_context(|| "Error on loading default catalog")?;
    // ...
}
```


## Let's open catalog provider API

Java language has popularized the [SPI (Service Provider Interface) pattern](https://en.wikipedia.org/wiki/Service_provider_interface). In this concept, some parts of your library is abstracted through interfaces (or `trait`). And everyone can provide implementations to extend your library capabilities (i.e. integrating with new librairies or providers). It also may be combined with service discovery for easier integration.

So transform `CatalogProvider` from a `struct` to a `trait`:

```rust
// src/service.rs
pub trait CatalogProvider {
    fn get_catalog(&self) -> Cow<model::Catalog>;
}

#[derive(Clone)]
pub struct SingleCatalogProvider {
    catalog: model::Catalog,
}

impl SingleCatalogProvider {

    pub fn from_static(catalog: model::Catalog) -> SingleCatalogProvider {
        SingleCatalogProvider {
            catalog,
        }
    }

    pub fn from_file_json(path: &str) -> Result<SingleCatalogProvider> {
        // ...
    }
}

impl CatalogProvider for SingleCatalogProvider {
    fn get_catalog(&self) -> Cow<model::Catalog> {
        Cow::Borrowed(&self.catalog)
    }
}

mod tests {
    use super::SingleCatalogProvider;

    // ...

    fn check_catalog_provider(provider: SingleCatalogProvider) {
        // ...
    }

    fn catalog_provider_static() {
        let provider = SingleCatalogProvider::from_static(build_catalog());
        // ...
    }

    fn catalog_provider_file_json() {
        let provider = SingleCatalogProvider::from_file_json("tests/default_catalog.json").expect("catalog load fail");
        // ...
    }

    fn catalog_provider_file_json_missing() {
        let error = SingleCatalogProvider::from_file_json("tests/missing_catalog.json").err().expect("catalog load MUST fail");
        // ...
    }
}


// src/lib.rs
pub fn new_scope(path: &str, catalog: Box<dyn service::CatalogProvider>) -> actix_web::Scope {}
pub async fn get_catalog(_req: HttpRequest, data: web::Data<Box<dyn service::CatalogProvider>>) -> HttpResponse {}

mod tests {
    // ...

    async fn test_get_catalog() {
        // ...
        let provider = service::SingleCatalogProvider::from_static(model::Catalog::new());
        let res = get_catalog(req, web::Data::new(Box::new(provider))).await;
        // ...
    }
}


// tests/get_catalog.rs
async fn main() {
    let catalog = osb::service::SingleCatalogProvider::from_static(osb::model::Catalog::new());
    let mut app = test::init_service(
        App::new()
            .service(osb::new_scope("", Box::new(catalog)))
    ).await;
    // ...
}


// src/bin/dummy-servicebroker.rs
async fn main() -> Result<()> {
    let catalog = osb::service::SingleCatalogProvider::from_file_json("tests/default_catalog.json")
                                                      .with_context(|| "Error on loading default catalog")?;
    HttpServer::new(move || {
        App::new()
            .service(osb::new_scope("", Box::new(catalog.clone())))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
```

Now that `CatalogProvider` is abstracted, it's possible to transform file-based from a static (loaded on creation) to a dynamic version (loaded on each call):

```rust
// src/service.rs
#[derive(Clone)]
pub struct JsonFileCatalogProvider {
    path: String,
}

impl JsonFileCatalogProvider {
    pub fn new(path: &str) -> Self {
        JsonFileCatalogProvider {
            path: path.to_owned()
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl CatalogProvider for JsonFileCatalogProvider {
    fn get_catalog(&self) -> Cow<model::Catalog> {
        let path = self.path();
        let file = std::fs::File::open(path)
                                 .with_context(|| format!("Access to catalog file '{}' has failed", path))
                                 .expect("");
        let catalog: model::Catalog = serde_json::from_reader(file)
                                                 .with_context(|| format!("Can't read catalog file '{}' as JSON", path))
                                                 .expect("");
        Cow::Owned(catalog)
    }
}

mod tests {
    use super::JsonFileCatalogProvider;

    fn check_catalog_provider(provider: &dyn CatalogProvider) {}

    fn catalog_provider_static() {
        // ...
        check_catalog_provider(&provider);
    }

    fn catalog_provider_file_json() {
        // ...
        check_catalog_provider(&provider);
    }

    #[test]
    fn catalog_provider_dynamic_file_json() {
        let provider = JsonFileCatalogProvider::new("tests/default_catalog.json");
        check_catalog_provider(&provider);
    }

    #[test]
    fn catalog_provider_dynamic_file_json_missing() {
        let provider = JsonFileCatalogProvider::new("tests/default_catalog2.json");
        let result = std::panic::catch_unwind(|| provider.get_catalog());
        assert!(result.is_err());
    }
}
```

However, there's one major issue: error handling. Let's reapply the previously-shown fallible pattern with `anyhow::Result`:

```rust
// src/service.rs
pub trait CatalogProvider {
    fn get_catalog(&self) -> Result<Cow<model::Catalog>>;
}

impl CatalogProvider for SingleCatalogProvider {
    fn get_catalog(&self) -> Result<Cow<model::Catalog>> {
        Ok(Cow::Borrowed(&self.catalog))
    }
}

impl CatalogProvider for JsonFileCatalogProvider {
    fn get_catalog(&self) -> Result<Cow<model::Catalog>> {
        let path = self.path();
        let file = std::fs::File::open(path)
                                 .with_context(|| format!("Access to catalog file '{}' has failed", path))?;
        let catalog: model::Catalog = serde_json::from_reader(file)
                                                 .with_context(|| format!("Can't read catalog file '{}' as JSON", path))?;
        Ok(Cow::Owned(catalog))
    }
}

mod tests {
    fn check_catalog_provider(provider: &dyn CatalogProvider) {
        let catalog  = provider.get_catalog().expect("Error on retrieving catalog");
        // ...
    }

    #[test]
    fn catalog_provider_dynamic_file_json_missing() {
        let provider = JsonFileCatalogProvider::new("tests/missing_catalog.json");
        let error = provider.get_catalog().err().expect("catalog load must fail");
        let ioerror = error.downcast_ref::<std::io::Error>().expect("catalog load error must be an I/O one");
        assert_eq!(std::io::ErrorKind::NotFound, ioerror.kind());
    }
}


// src/lib.rs
pub async fn get_catalog(_req: HttpRequest, data: web::Data<Box<dyn service::CatalogProvider>>) -> HttpResponse {
    match data.get_catalog() {
        Ok(catalog) => HttpResponse::Ok().json(catalog),
        Err(error)  => {
            eprintln!("ERROR: {:?}", error);
            HttpResponse::InternalServerError().finish()
        },
    }

}

mod tests {
    #[actix_rt::test]
    async fn test_get_catalog_missing() {

        let req = test::TestRequest::get()
                                    .uri("/v2/catalog")
                                    .to_http_request();
        let provider = service::JsonFileCatalogProvider::new("tests/missing_catalog.json");
        let res = get_catalog(req, web::Data::new(Box::new(provider))).await;
        assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        match res.body() {
            ResponseBody::Body(body)  => match body {
                Body::Empty => (),
                _           => panic!("Unexpected body type ({:?})", body),
            },
            ResponseBody::Other(body) => panic!("Found response body of type other ({:?})", body),
        };
    }
}


// tests/get_catalog.rs
use actix_web::{test, App, http::StatusCode, body::{Body, ResponseBody}};

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
```

Finally, let's reorganize things:

```rust
// src/service.rs
pub trait CatalogProvider {
    fn to_single(&self) -> Result<SingleCatalogProvider> {
        self.get_catalog()
            .map(|cow| match cow {
                Cow::Owned(catalog)        => catalog,
                Cow::Borrowed(catalog)     => catalog.clone(),
            })
            .map(|catalog| SingleCatalogProvider::new(catalog))
    }
}

impl SingleCatalogProvider {
    // Rename: from_static-> new
    pub fn new(catalog: model::Catalog) -> SingleCatalogProvider { /* ... */ }

    // Delete
    //pub fn from_file_json(path: &str) -> Result<SingleCatalogProvider> { ... }
}

pub mod providers {
    pub mod catalog {
        use super::super::{model, SingleCatalogProvider, JsonFileCatalogProvider};

        pub fn single(catalog: model::Catalog) -> SingleCatalogProvider {
            SingleCatalogProvider::new(catalog)
        }

        pub fn file_json(path: &str) -> JsonFileCatalogProvider {
            JsonFileCatalogProvider::new(path)
        }
    }
}

mod tests {
    // Delete
    // fn catalog_provider_file_json()
    // fn catalog_provider_file_json_missing()
}


// src/bin/dummy-servicebroker.rs
use osb::service::CatalogProvider; // Enable `to_single` function

async fn main() -> Result<()> {
    let catalog = osb::service::providers::catalog::file_json("tests/default_catalog.json")
                                                   .to_single()
                                                   .with_context(|| "Error on loading default catalog")?;
    // ...
}
```


## Let's cache Catalog Provider result

As Catalog Provider API has now been opened, it's easy to extend. In last examples, JSON file Catalog Provider has been used and result has been directly put in cache at initialization through `to_single()` method. However, why not do it lazily (i.e. on first call) ?

As fetching catalog isn't supposed to mutate provider, then [interior mutability](https://doc.rust-lang.org/reference/interior-mutability.html) must be used:

```rust
// src/service.rs
pub struct CachingCatalogProvider<T: CatalogProvider> {
    provider: T,
    cache: std::cell::RefCell<Option<SingleCatalogProvider>>,
}

impl<T: CatalogProvider> CachingCatalogProvider<T> {
    pub fn new(provider: T) -> Self {
        CachingCatalogProvider {
            provider,
            cache: std::cell::RefCell::default(),
        }
    }
}

impl<T: CatalogProvider> CatalogProvider for CachingCatalogProvider<T> {
    fn get_catalog(&self) -> Result<Cow<model::Catalog>> {
        if let Some(provider) = self.cache.borrow().as_ref() {
            let catalog = provider.get_catalog()?;
            return Ok(Cow::Owned(catalog.into_owned()))
        }
        let caching = self.provider.to_single()?;
        *self.cache.borrow_mut() = Some(caching);
        self.get_catalog()
    }
}

pub mod providers {
    pub mod catalog {
        use super::super::{CatalogProvider, CachingCatalogProvider};

        pub fn cache<T: CatalogProvider>(provider: T) -> CachingCatalogProvider<T> {
            CachingCatalogProvider::new(provider)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CachingCatalogProvider;
    use anyhow::Result;

    #[test]
    fn catalog_provider_caching() {
        struct Counting<'a> {
            count: &'a std::cell::Cell<u32>,
        }
        impl<'a> Counting<'a> {
            fn new(count: &'a std::cell::Cell<u32>) -> Self {
                Counting {
                    count
                }
            }
        }
        impl<'a> CatalogProvider for Counting<'a> {
            fn get_catalog(&self) -> Result<std::borrow::Cow<model::Catalog>> {
                self.count.set(self.count.get() + 1);
                Ok(std::borrow::Cow::Owned(model::Catalog::new()))
            }
        }
        let counter = std::cell::Cell::default();

        let cache   = CachingCatalogProvider::new(Counting::new(&counter));
        assert_eq!(0, counter.get());

        assert!(cache.get_catalog().is_ok());
        assert_eq!(1, counter.get());

        assert!(cache.get_catalog().is_ok());
        assert_eq!(1, counter.get());
    }
}
```

Maybe some explanations is needed. [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) let us store  a value which references can be manipulated in controlled scope (i.e. blocks). Borrow rules are dynamically (i.e. at runtime) checked. Thus, we first check if there's some cached catalog holder (i.e. `SingleCatalogProvider`) and returns its catalog. Otherwise, read borrow is dropped, and a write/mutable one is temporary obtain to update with cached catalog holder. Finally, recursively call the function to avoid repeating read code.

For testing, a counting caller is implemented, still using interior mutability pattern. Then, just check there's no call during caching creation (i.e. lazy behavior) and call only once even if asked twice (i.e. cache behavior).

Note that this implementation doesn't support [multithreading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html), which is not a problem as Actix will instantiate a new scope for each worker thread (add a `println!("...")` into `HttpServer::new()` closure to observe it).
