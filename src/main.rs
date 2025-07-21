//! # Web service epoch axum
//!
//! **[documentation](https://docs.rs/web-service-epoch-axum/)**
//! •
//! **[source](https://github.com/joelparkerhenderson/web-service-epoch-axum/)**
//! •
//! **[llms.txt](https://raw.githubusercontent.com/joelparkerhenderson/web-service-epoch-axum/refs/heads/main/llms.txt)**
//! •
//! **[crate](https://crates.io/crates/web-service-epoch-axum)**
//! •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! Web service that displays the Unix epoch time by using Axum, Tokio, Rust.
//!
//! This is a very simple web service that we use for testing our systems.
//!
//! ## Steps
//!
//! Run the service using the default address 0.0.0.0:8080:
//!
//! ```sh
//! cargo run
//! ```
//!
//! Browse <https://localhost:8080/epoch>
//!
//! You should see a web page that displays the epoch time in seconds.
//!
//! Wait a little bit, then use your browser to reload the web page.
//!
//! You should see the epoch time increase a little bit.
//!
//! ## Options
//!
//! Run the service using a command line option for a custom address:
//!
//! ```sh
//! cargo run -- "1.2.3.4:5678"
//! ```
//!
//! Run the service using an environment variable for a custom address:
//!
//! ```sh
//! export ADDRESS="1.2.3.4:5678"
//! cargo run
//! ```
//!
//! ## References
//!
//! Based on Demo Rust Axum free open source software:
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//!

mod app;

/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// The main function does these steps:
/// - Start tracing and emit a tracing event.
/// - Get a command line argument as our bind address.
/// - Create our application which is an axum router.
/// - Run our application as a hyper server.
#[tokio::main]
async fn main() {
    // Start tracing and emit a tracing event.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::event!(tracing::Level::INFO, "main");

    // Get command line arguments.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Use the first arg for tokio::net::TcpListener::bind(…),
    // or then env var ADDRESS, or default "0.0.0.0:8080".
    let bind_address_string = match args.get(0) {
        Some(x) => x.clone(),
        None => match std::env::var("ADDRESS") {
            Ok(address) => address,
            Err(_e) => "0.0.0.0:8080".into()
        }
    };

    // Create our application which is an axum router.
    let app = crate::app::app();

    // Run our application as a hyper server.
    let listener = tokio::net::TcpListener::bind(bind_address_string).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Shutdown signal to run axum with graceful shutdown when
/// a user presses Ctrl+C or Unix sends a terminate signal.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
