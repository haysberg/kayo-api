use axum::{response::Html, routing::get, Router};

async fn health_handler() -> Html<&'static str> {
    Html("UP")
}

async fn notfound_handler() -> Html<&'static str> {
    Html("404 Page not found")
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .fallback(notfound_handler)
}