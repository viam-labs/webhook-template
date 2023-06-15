#![allow(unused)]

mod error;
mod web;
pub use self::error::{Error, Result};

use std::net::SocketAddr;
use std::time::Duration;
use duct::cmd;
use std::io::prelude::*;
use std::io::BufReader;

use axum::{Router, Json};
use axum::extract::Query;
use axum::routing::get;
use axum::response::{Html, IntoResponse};
use axum::http::StatusCode;
use serde::Deserialize;
use tracing::{error, event, Level, span};
use opentelemetry::sdk::export::trace::stdout;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use web::routes_login;

use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
struct EspParams {
    location: Option<String>,
    secret: Option<String>,
    target: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HelloResponse {
    response: String,
}

async fn handler_hello(Json(payload): Json<HelloParams>) -> impl IntoResponse {
    event!(Level::INFO, "->> {:<12} - handler_esp - {payload:?}", "HANDLER");
    let name = payload.name.unwrap_or("World".to_string());
    Html(format!("Hello, {}!", name))
}

async fn handler_esp(Json(payload): Json<EspParams>) -> impl IntoResponse {
    event!(Level::INFO, "->> {:<12} - handler_hello - {payload:?}", "HANDLER");
    println!("received payload: {:?}", payload);

    let big_cmd = cmd!(
        "python3", 
        "hook.py", 
        payload.location.unwrap_or("url".to_string()), 
        payload.secret.unwrap_or("password".to_string()), 
        payload.target.unwrap_or("target".to_string()),
        "1>&2");
    let reader = big_cmd.stderr_to_stdout().reader().unwrap();
    let mut lines = BufReader::new(reader).lines();
    for line in lines {
        println!("{}", line.unwrap());
    }

}

#[tokio::main]
async fn main() {


    // Install a new OpenTelemetry trace pipeline
    let tracer = stdout::new_pipeline().install_simple();

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        error!("This event will be logged in the root span.");
    });

    let routes_all = Router::new()
        .route(
            "/esp",
            get(handler_esp),
        )
        .route(
            "/hello",
            get(handler_hello),
        )
        .merge(routes_login::routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap();
}


