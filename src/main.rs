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
use tracing::{event, Level};
use web::routes_login;

use std::process::Command;

#[derive(Debug, Deserialize)]
struct EspParams {
    location: Option<String>,
    secret: Option<String>,
}

async fn handler_esp(Query(params): Query<EspParams>) -> impl IntoResponse {
    event!(Level::INFO, "->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let output = Command::new("python3")
        .arg("./hook.py")
        .arg(params.location.clone().unwrap_or("location".to_string()))
        .arg(params.secret.clone().unwrap_or("secret".to_string()))
        .output().unwrap();
    println!("command executed with {:?}", params);
}

#[tokio::main]
async fn main() {

    let routes_all = Router::new()
        .route(
            "/esp",
            get(handler_esp),
        )
        .merge(routes_login::routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap();
}


