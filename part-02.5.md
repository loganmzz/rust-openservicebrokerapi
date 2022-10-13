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
