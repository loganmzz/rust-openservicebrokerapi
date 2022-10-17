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

## Upgrade Actix Web to 3.0.0

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

Finally, to check everything is fine: `cargo test` !
