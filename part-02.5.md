## Freeze Rust version

It was a long time ago since last step (2021-06-10). Latest Rust release available at this time was [`1.52.1`](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1521-2021-05-10).
However, current (2022-10-13) is [`1.64.0`](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1640-2022-09-22).
And trying to build with current Rust release results in following error:

```text
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
   --> /home/loganmzz/.cargo/registry/src/github.com-1ecc6299db9ec823/socket2-0.3.11/src/sockaddr.rs:156:9
    |
156 |         mem::transmute::<SocketAddrV4, sockaddr_in>(v4);
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: source type: `SocketAddrV4` (48 bits)
    = note: target type: `sockaddr_in` (128 bits)

For more information about this error, try `rustc --explain E0512`.
   Compiling proc-macro-hack v0.5.11
error: could not compile `socket2` due to previous error
warning: build failed, waiting for other jobs to finish...
```

Let's fix it quickly by adding a `rust-toolchain` file:

```text
1.52.1
```

Learn more about `rust-toolchain[.toml]` file in [Overrides chapter from rustup documentation](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file).

Now, run `cargo test` to check everything is fine.

## Upgrade Actix Web to 3.3.3

Actix is our most important dependency, it's the framework that provide Web protocol to access our OpenServiceBroker API. Several releases including two majors since the [`2.0`](https://github.com/actix/actix-web/releases/tag/web-v2.0.0) we are using.

Start bumping to `3.0.0` which is both the immediate next release and also next major.

```toml
[dependencies]
actix-web = "=3.0.0"
```

Now, just replace `actix_rt::main` with `actix_web::main`:

```rust
// src/bin/dummy-servicebroker.rs
// use actix_rt;

#[actix_web::main]
async fn main() -> Result<()> {}
```

Then, check everything is fine with `cargo test` ... or not:

```text
   Compiling futures v0.3.4
error[E0432]: unresolved import `futures_core::core_reexport`
   --> /home/loganmzz/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.3.4/src/lib.rs:542:9
    |
542 | pub use futures_core::core_reexport;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `core_reexport` in the root

error[E0433]: failed to resolve: could not find `document_join_macro` in `futures_util`
   --> /home/loganmzz/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.3.4/src/lib.rs:561:15
    |
561 | futures_util::document_join_macro! {
    |               ^^^^^^^^^^^^^^^^^^^ could not find `document_join_macro` in `futures_util`

error[E0433]: failed to resolve: could not find `document_select_macro` in `futures_util`
   --> /home/loganmzz/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.3.4/src/lib.rs:584:15
    |
584 | futures_util::document_select_macro! {
    |               ^^^^^^^^^^^^^^^^^^^^^ could not find `document_select_macro` in `futures_util`

error[E0603]: module `async_await` is private
   --> /home/loganmzz/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.3.4/src/lib.rs:547:23
    |
547 | pub use futures_util::async_await;
    |                       ^^^^^^^^^^^ private module
    |
note: the module `async_await` is defined here
   --> /home/loganmzz/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.24/src/lib.rs:34:1
    |
34  | mod async_await;
    | ^^^^^^^^^^^^^^^^

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0432, E0433, E0603.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `futures`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
```

Cargo is failing because it mixes some crates at different versions (`futures@0.3.4` and `futures-util@0.3.24`). We will try to freeze the `futures` crate family to closest available version, because the [`0.3.4` release](https://crates.io/crates/futures/0.3.4) has been [yanked](https://doc.rust-lang.org/cargo/commands/cargo-yank.html) (i.e. deprecated). So, let's go with [`0.3.11`](https://crates.io/crates/futures/0.3.11).
For this case, you can use `cargo update -p "${crate_name}" --precise "${crate_version}"`.

Unfortunately, Cargo is relying on semver by default. Meaning when you declare `3.x.y` version, it means highest `3.*` possible (depending on other crate constraints).
As several crates in our dependencies don't meet semver requirements, we will have to fix several version issues. Many strategies is possible like retrieving whole dependency declaration and freeze to smallest possible or just die and retry.
For the sake of your mental health, here are the commands:

```bash
cargo update -p actix-rt           --precise 1.1.1  &&
cargo update -p awc                --precise 2.0.0  &&
cargo update -p actix-http         --precise 2.0.0  &&
cargo update -p futures-executor   --precise 0.3.11 &&
cargo update -p cookie             --precise 0.14.1 &&
cargo update -p time               --precise 0.2.11 &&
cargo update -p proc-macro-hack    --precise 0.5.19 &&
cargo update -p time-macros        --precise 0.1.1  &&
cargo update -p futures-util       --precise 0.3.11 &&
cargo update -p futures-channel    --precise 0.3.11 &&
cargo update -p futures-core       --precise 0.3.11 &&
cargo update -p futures-io         --precise 0.3.11 &&
cargo update -p futures-macro      --precise 0.3.11 &&
cargo update -p futures-sink       --precise 0.3.11 &&
cargo update -p futures-task       --precise 0.3.11 &&
cargo update -p futures            --precise 0.3.11 &&
cargo update -p once_cell          --precise 1.12.0 &&
true
```

Then, to check everything is fine: `cargo test` !

Now, move to latest [`3.3.3`](https://crates.io/crates/actix-web/3.3.3) to complete the `3.x` upgrade:

```toml
[dependencies]
actix-web = "=3.3.3"
```

Finally, to check everything is fine: `cargo test` ! And we are ready to bump to the next major version !

## Upgrade Actix Web to 4.3.1

As previously, start by bumping to `4.0.0`:

```toml
[dependencies]
actix-web = "=4.0.0"
```

Starting from this major, directly depending on `actix-rt` is no more required as it also re-export `actix_rt::test` macro:

```toml
[dependencies]
#actix-rt = "1.0"
```

```rust
// src/lib.rs
mod tests {
    // use actix_rt;

    #[actix_web::test]
    async fn test_get_catalog() {}

    #[actix_web::test]
    async fn test_get_catalog_missing() {}
}

// tests/get_catalog.rs
#[actix_web::test]
async fn ok() {}

#[actix_web::test]
async fn missing() {}
```

Also updates Rust version into `rust-toolchain` file:

```text
1.54.0
```

Then, as usual execute `cargo test`:

```text
error: failed to download `hashbrown v0.12.3`

Caused by:
  unable to get packages from source

Caused by:
  failed to parse manifest at `/home/logan/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.3/Cargo.toml`

Caused by:
  feature `edition2021` is required

  this Cargo does not support nightly features, but if you
  switch to nightly channel you can add
  `cargo-features = ["edition2021"]` to enable this feature
```

And the magic formula:

```bash
cargo update -p actix-http          --precise 3.0.0   &&
cargo update -p h2                  --precise 0.3.9   &&
cargo update -p indexmap            --precise 1.5.2   &&
cargo update -p hashbrown           --precise 0.8.1   &&
cargo update -p ahash               --precise 0.7.4   &&
cargo update -p tracing             --precise 0.1.30  &&
cargo update -p tracing-core        --precise 0.1.22  &&
cargo update -p once_cell           --precise 1.5.2   &&
cargo update -p itoa:0.4.5          --precise 0.4.7   &&
cargo update -p time                --precise 0.3.5   &&
cargo update -p actix-web-codegen   --precise 4.0.0   &&
true
```

Finally, issues come from our own outdated code, refresh it !

Starts with `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0.104", features = ["derive"] }
```

Next, [application data](https://docs.rs/actix-web/4.0.0/actix_web/struct.App.html#method.app_data) (i.e. state) must be wrapped into [actix_web::web::Data](https://docs.rs/actix-web/4.0.0/actix_web/web/struct.Data.html):

```rust
// src/lib.rs
pub fn new_scope(path: &str, catalog: Box<dyn service::CatalogProvider>) -> actix_web::Scope {
    actix_web::Scope::new(path)
                     .app_data(web::Data::new(catalog))
                     .route("/v2/catalog", web::get().to(get_catalog))
}
```

Afterward, [response body handling](https://docs.rs/actix-web/4.0.0/actix_web/struct.HttpResponse.html#method.body) must be updated:

```rust
// src/lib.rs
mod tests {
    use actix_web::{body::{MessageBody}, http, test, web};

    #[actix_web::test]
    async fn test_get_catalog() {
        // ...
        assert_eq!(res.status(), http::StatusCode::OK);
        let bytes = res.into_body().try_into_bytes().expect("Unable to get body bytes");
        // ...
    }

    #[actix_web::test]
    async fn test_get_catalog_missing() {
        // ...
        assert_eq!(res.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        let bytes = res.into_body().try_into_bytes().expect("Expected body type, but other was found");
        if bytes.len() != 0 {
            panic!("Unexpected body ({:?})", bytes);
        }
    }
}

// tests/get_catalog.rs
use actix_web::{test, App, http::StatusCode, body::{MessageBody}};

#[actix_web::test]
async fn ok() {
    // ...
    let catalog: osb::model::Catalog = test::call_and_read_body_json(&mut app, req).await;
    // ...
}

#[actix_web::test]
async fn missing() {
    // ...
    let res = test::call_service(&mut app, req).await;
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let bytes = res.into_body().try_into_bytes().expect("Expected body type, but other was found");
    if bytes.len() != 0 {
        panic!("Unexpected body ({:?})", bytes);
    }
}
```

Then, `cargo test` ! Now, the last step: bump `actix-web` to latest version `4.3.1`:


```toml
[dependencies]
actix-web = "=4.3.1"
```

This require to update Rust version into `rust-toolchain` file:

```text
1.57.0
```

Finally, for the last time:

```bash
cargo clean && cargo test
```
