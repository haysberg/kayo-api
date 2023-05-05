extern crate dotenv;

use tracing::{info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, fmt};
use tower_http::trace::TraceLayer;
use axum::{Router};
use std::{net::SocketAddr, fmt::Debug};
use kayo_api::routes;
use tokio::signal;

#[tokio::main]
async fn main() {
    // We initialize the logs formatting
    let format = fmt::format()
        .with_level(true)
        .with_thread_ids(false)
        .with_source_location(false)
        .with_target(false)
        .with_line_number(false)
        .compact();

    //Initializing the log display
    tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().event_format(format))
    .init();

    info!("⚙️  Logger initialized. Loading parameters...");

    let port = kayo_api::config::from_env();

    // build our application with a route
    let app = Router::new()
        .merge(routes::html::router())
        .layer(TraceLayer::new_for_http());
    

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("🚀 KAY/O is online on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

}

// notify os that process will stop
async fn shutdown_signal() {
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

    info!("signal received, starting graceful shutdown");
}
