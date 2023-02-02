use axum::{
    body::Body,
    handler::Handler,
    http::StatusCode,
    routing::{get_service, get, post},
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
                .expect("Failed to await for SIGINT");
        });

    println!("Server initialized");
    server.await.expect("Failed to start server");
}

/// 404 Not Found fallback handler
#[allow(clippy::unused_async)]
pub async fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../frontend/not_found.html"))
    ).into_response()
}

/// handler for root "/" (homepage)
#[allow(clippy::unused_async)]
pub async fn root() -> Html<String> {
    Html(include_str!("../frontend/index.html").to_string())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv()
        .ok();

    let app: Router<Body> = Router::new()
        .route("/", get(root))
        .route("/lego", post(wrap!(functions::lego, models::SizeOption)))
        .route("/minecraft", post(wrap!(functions::minecraft, models::SizeOption)))
        .route("/paint", post(wrap!(functions::paint, models::PaintOption)))
        .route("/frost", post(wrap!(functions::frost, models::NoArgs)))
        .route("/braille", post(wrap!(functions::braille, models::BrailleOption)))
        .route("/ascii", post(wrap!(functions::ascii, models::AsciiOption)))
        .route("/matrix", post(wrap!(functions::matrix, models::MatrixOption)))
        .route("/lines", post(wrap!(functions::lines, models::ShapesOption)))
        .route("/balls", post(wrap!(functions::balls, models::ShapesOption)))
        .route("/squares", post(wrap!(functions::squares, models::ShapesOption)))
        .route("/black_white", post(wrap!(functions::black_white, models::SmoothOption)))
        .route("/edge", post(wrap!(functions::edge, models::NoArgs)))
        .route("/emboss", post(wrap!(functions::emboss, models::NoArgs)))
        .route("/hue_rotate", post(wrap!(functions::hue_rotate, models::NoArgs)))
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
                    format!("Something went wrong: {err}"),
                )
            }),
        );

    let port = std::env::var("PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    run(app, Some(port)).await;
}
