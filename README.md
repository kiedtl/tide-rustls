# Tide rustls listener

A TLS listener for [tide](https://github.com/http-rs/tide), based on `futures-rustls`.

* [CI ![CI][ci-badge]][ci]
* [API Docs][docs] [![docs.rs docs][docs-badge]][docs]
* [Releases][releases] [![crates.io version][version-badge]][lib-rs]

[ci]: https://github.com/http-rs/tide-rustls/actions?query=workflow%3ACI
[ci-badge]: https://github.com/http-rs/tide-rustls/workflows/CI/badge.svg
[releases]: https://github.com/http-rs/tide-rustls/releases
[docs]: https://docs.rs/tide-rustls
[lib-rs]: https://lib.rs/tide-rustls
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[version-badge]: https://img.shields.io/crates/v/tide-rustls.svg?style=flat-square

## Installation
```sh
$ cargo add tide-rustls
```

## Using with tide
```rust
fn main() -> tide::Result<()> {
    smol::block_on(async {
        let mut app = tide::new();
        app.at("/").get(|_| async { Ok("Hello TLS") });
        app.listen(
            TlsListener::build()
                .addrs("localhost:4433")
                .cert(std::env::var("TIDE_CERT_PATH").unwrap())
                .key(std::env::var("TIDE_KEY_PATH").unwrap()),
            )
            .await?;
        Ok(())
    })
}
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
