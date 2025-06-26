//! # Web service epoch axum
//! 
//! Web service that displays the Unix epoch time by using Axum, Tokio, Rust.
//! 
//! This is a very simple web service that we use for testing our systems.
//! 
//! ## Steps
//! 
//! Run the service on host 0.0.0.0 port 3000 or wherever you wish:
//! 
//! ```sh
//! cargo run -- "0.0.0.0:3000"
//! ```
//! 
//! Browse <https://localhost:3000>
//! 
//! You should see a web page that displays the epoch time in seconds.
//! 
//! ## References
//! 
//! Based on Demo Rust Axum free open source software:
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//! 

/// Use axum capabilities.
use axum::routing::get;

/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]  
async fn main() {
    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::event!(tracing::Level::INFO, "main");

    // Get command line arguments.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Use the first arg for tokio::net::TcpListener::bind(…)  
    let bind_address = match args.get(0) {
        Some(x) => x.clone(),
        None => "0.0.0.0:3000".into(),
    };

    // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(epoch));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

////
// Shutdown signal to run axum with graceful shutdown
//
// This handles a user pressing Ctrl+C.
// This handles a Unix terminate signal.
////

use tokio::signal;

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
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

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}

/// axum handler for "GET /epoch" which shows the current epoch time.
/// This shows how to write a handler that uses time and can error.
pub async fn epoch() -> Result<String, axum::http::StatusCode> {
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(duration) => Ok(format!("{}", duration.as_secs())),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
