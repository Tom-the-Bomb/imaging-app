use axum::{
    body::Body,
    handler::Handler,
    http::StatusCode,
    routing::{get_service, post},
    Router,
};
use ril::prelude::*;
use std::net::SocketAddr;
use crate::wrap_fn as wrap;

mod functions;
mod wrapper;
mod models;

const MAX_IMAGE_SIZE: usize = 15_000_000;

/// a simple function that creates a server,
/// serving the router and then running the server.
async fn run(app: Router<Body>, port: Option<u16>) {
    let port = port.unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to await for SIGINT")
        });

    println!("Server initialized");
    server.await.expect("Failed to start server");
}

#[tokio::main]
async fn main() {
    let app: Router<Body> = Router::new()
        .route("/lego", post(wrap!(functions::lego, models::SizeOption)))
        .fallback(
            get_service(functions::not_found.into_service())
                .handle_error(|err| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Something went wrong: {}", err),
                    )
                }),
        );

    run(app, None).await;
}
