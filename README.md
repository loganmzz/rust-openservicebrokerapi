Rust - Open Service Broker API
===



## Presentation

This projects aims to demonstrate [Open Service Broker](https://www.openservicebrokerapi.org/) implementation using Rust language.



## What is Open Service Broker API ?

On Cloud platform such as Cloud Foundry, Heroku, Kubernetes, ... it is very important to deploy applications. And it's also very important to integrate services such as database, a message-oriented middleware, etc. There are often call managed services and accessible through a service catalog.

The Open Service Broker API aims to provide an easy way to extend this service catalog. Whatever you're a service vendor wanting to have a Cloud offering or an engineering team wanting to provide enterprise services to development teams, you can expose your own catalog and make it "consumable" by development teams.

The Open Service Broker API has been first defined and used by Pivotal in its Cloud Foundry solution. Then the specification has been opened, so service offering provider can integrate with many Cloud solution. For example, [the Kubernetes Service Catalog ISG](https://svc-cat.io/) is responsible of integration with Kubernetes.

If you want more information, please visit: https://www.openservicebrokerapi.org/



## How to process ?

It currently exists some framework to help you develop an Open Service Broker without to deal with low level (HTTP-based API) consideration:

* [Spring Cloud Open Service Broker](https://spring.io/projects/spring-cloud-open-service-broker) (JVM)
* [brokerapi](https://github.com/pivotal-cf/brokerapi) (Go)
* [Open Service Broker API for .NET](https://github.com/AXOOM/OpenServiceBroker) (.Net)

So, idea is to provide a library crate handling HTTP API and delegating calls to some abstractions (i.e. `trait`).

Following Test-Driven Development, we will:

1. right some tests,
1. implements them,
1. optionally improves written code,
1. and going back to (1) until specification coverage is complete



## How to get started ?

First, if you don't have Rust toolchains installed, just visit: https://www.rust-lang.org/tools/install. Looking for an editor ? Check [Are we (I)DE yet?](https://areweideyet.com/).

Then:

```
cargo new --lib openservicebroker
```

In order to implement HTTP handlers, [actix-web](https://actix.rs/) will be used. Thus, edit `Cargo.toml` to add this dependency:

```toml
[dependencies]
actix-web = "2.0"
actix-rt = "1.0"
```



## What about the catalog ?

The first step is to expose a service catalog. It lets inform the platform about services you provide. So let's start with [`GET /v2/catalog` route](https://github.com/openservicebrokerapi/servicebroker/blob/v2.15/spec.md#route).

### RED step

Let's create a first test, edit `lib.rs`:

```rust
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
            assert_eq!(body.services.len(), 0);
        } else {
            assert!(false, "Expected body type, but other was found");
        }
    }
}
```

As usual in TDD, it won't compile as our code is empty. You can run `cargo build --tests` if you don't trust me !

### GREEN step

So let's add `get_catalog` function and `Catalog` struct:

```rust
use actix_web::{HttpRequest, HttpResponse};

struct Catalog {
    services: Vec<()>,
}

async fn get_catalog(_req: HttpRequest) -> HttpResponse<Catalog> {
    HttpResponse::Ok().message_body(Catalog { services: vec![] })
}
```

Now, it's time to check for THE green status, run `cargo test`. You should have something like:

```text
   Compiling openservicebroker v0.1.0 (/home/logan/projects/openservicebroker)
warning: struct is never constructed: `Catalog`
 --> src/lib.rs:4:8
  |
4 | struct Catalog {
  |        ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: function is never used: `get_catalog`
 --> src/lib.rs:9:10
  |
9 | async fn get_catalog(_req: HttpRequest) -> HttpResponse<Catalog> {
  |          ^^^^^^^^^^^

    Finished test [unoptimized + debuginfo] target(s) in 3.56s
     Running target/debug/deps/openservicebroker-c3104580c5641bb9

running 1 test
test tests::test_get_catalog ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests openservicebroker

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Test has passed ! Finally, go for the final TDD step: refactor.

### REFACTOR step

When refactoring, you are supposed to do one thing at a time and check each time tests are still passing. However, it will take too much time time to explain each action/modification. To resume the following code will fix warnings and use more idiomatic Rust:

```rust
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
```



## How to write integration tests ?

In normal cases, you are supposed to continue TDD iterations until you complete specification coverage. However, it seems interesting to check validity at higher level. So let's add an integration test with `tests/get_catalog.rs`:

```rust
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
```

As in TDD cases, code isn't compiling. In fact, we need to introduce JSON serialization. Let's add Serde dependency into `Cargo.toml`:

```toml
[dependencies]
serde = "1.0.104"
serde_json = "1.0.48"
```

And update application code:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Catalog { /* ... */ }

pub async fn get_catalog(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(Catalog::new())
}
```

And update unit test code:

```rust
use actix_web::{http, test, dev::{ResponseBody, Body}};

async fn test_get_catalog() {
    // ...
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
```

Thus, run `cargo test` to check all tests are passing !



## What about manual/external testing ?

Even with integration test, an HTTP server has never been run. Such setup prevents executing manual test or using external tools to run test suite (e.g. [Postman Test scripts](https://learning.postman.com/docs/postman/scripts/test-scripts/)).

Fix it right now by adding an optional binary, edit `src/bin/dummy-servicebroker.rs`:

```rust
use openservicebroker as osb;

use actix_web::{web, App, HttpServer};
use actix_rt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/v2/catalog", web::get().to(osb::get_catalog))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

As it's our sole binary, just run `cargo run` and open [http://localhost:8080/v2/catalog](http://localhost:8080/v2/catalog) !
