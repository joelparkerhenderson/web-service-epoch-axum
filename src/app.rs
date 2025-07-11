/// Use axum capabilities.
use axum::routing::get;

/// Create our application by creating our router.
pub fn app() -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .route("/", get(epoch))
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}

/// axum handler for "GET /" which shows the current epoch time.
/// This shows how to write a handler that uses time and can error.
pub async fn epoch() -> Result<String, axum::http::StatusCode> {
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(duration) => Ok(format!("{}", duration.as_secs())),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_response() {
        let app: axum::Router = app();
        let server = TestServer::new(app).unwrap();
        let response_text_0 = server.get("/").await.text();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let response_text_1 = server.get("/").await.text();
        assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1)
    }

}
