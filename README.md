# Web service epoch axum

**[documentation](https://docs.rs/web-service-epoch-axum/)**
•
**[source](https://github.com/joelparkerhenderson/web-service-epoch-axum/)**
•
**[llms.txt](https://raw.githubusercontent.com/joelparkerhenderson/web-service-epoch-axum/refs/heads/main/llms.txt)**
•
**[crate](https://crates.io/crates/web-service-epoch-axum)**
•
**[email](mailto:joel@joelparkerhenderson.com)**

Web service that displays the epoch time by using Axum, Tokio, Rust.

This is a very simple web service that we use for testing our systems.

## Steps

Run the service using the default address 0.0.0.0:8080:

```sh
cargo run
```

Browse <https://localhost:8080/epoch>

You should see a web page that displays the epoch time in seconds.

Wait a little bit, then use your browser to reload the web page.

You should see the epoch time increase a little bit.

## Options

Run the service using a command line option for a custom address:

```sh
cargo run -- "1.2.3.4:5678"
```

Run the service using an environment variable for a custom address:

```sh
export ADDRESS="1.2.3.4:5678"
cargo run
```

## References

Based on free open source software [Demo Rust Axum](https://github.com/joelparkerhenderson/demo-rust-axum).
