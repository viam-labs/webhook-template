#![allow(unused)]

mod error;
mod web;
pub use self::error::{Error, Result};

use std::net::SocketAddr;

use axum::Router;
use axum::extract::Query;
use axum::routing::get;
use axum::response::{Html, IntoResponse};
use serde::Deserialize;
use web::routes_login;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!</strong>"))
}

#[tokio::main]
async fn main() {

    let routes_all = Router::new()
        .route(
            "/hello",
            get(handler_hello),
        )
        .merge(routes_login::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap();
}


