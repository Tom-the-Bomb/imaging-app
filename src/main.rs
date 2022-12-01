use axum::{
    body::Body,
    handler::Handler,
    http::StatusCode,
    routing::{get_service, post},
    response::{Html, IntoResponse, Response},
    Router,
};
use ril::prelude::*;
use std::{io, net::SocketAddr};
use tower::util::ServiceExt;
use tower_http::services::ServeDir;
use crate::wrap_fn as wrap;

mod braille_data;
mod helpers;
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

/// 404 Not Found fallback handler
pub async fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../frontend/not_found.html"))
    ).into_response()
}

/// handler for root "/" (homepage)
pub async fn root() -> Html<String> {
    Html(include_str!("../frontend/index.html").to_string())
}

#[tokio::main]
async fn main() {
    let app: Router<Body> = Router::new()
        .route("/lego", post(wrap!(functions::lego, models::SizeOption)))
        .route("/minecraft", post(wrap!(functions::minecraft, models::SizeOption)))
        .route("/paint", post(wrap!(functions::paint, models::NoArgs)))
        .route("/frost", post(wrap!(functions::frost, models::NoArgs)))
        .route("/braille", post(wrap!(functions::braille, models::BrailleOption)))
        .fallback(
            get_service(
                ServeDir::new("./frontend/")
                .not_found_service(
                    not_found
                        .into_service()
                        .map_err(|_| io::Error::from(io::ErrorKind::Other))
                )
            )
            .handle_error(|err| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Something went wrong: {}", err),
                )
            }),
        );

    run(app, None).await;
}
