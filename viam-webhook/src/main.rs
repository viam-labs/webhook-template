use std::net::SocketAddr;
use std::process::Command;
use serde::{Serialize, Deserialize};

use tracing::{info, debug, instrument};
use tracing_subscriber::{
    prelude::*,
    layer::SubscriberExt,
};

use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use tower_http::trace::TraceLayer;


#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct HelloResponse {
    response: String,
}

#[derive(Debug, Deserialize)]
struct EspParams {
    location: Option<String>,
    secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EspResponse {
    response: String,
}


#[instrument]
async fn handler_hello(Json(payload): Json<HelloParams>) -> Json<HelloResponse> {
    debug!("->> handler_hello - {payload:?}");
    let name = payload.name.unwrap_or("World".to_string());
    let response = format!("Hello, {}!", name);
    let response = HelloResponse { response };
    Json(response)
}

#[instrument]
async fn handler_hook(Path(lang): Path<String>, Json(payload): Json<EspParams>) -> Json<EspResponse> {
    let fqdn = payload.location.unwrap_or("url".to_string());
    let secret = payload.secret.unwrap_or("secret".to_string());
    match lang.as_str() {
        "py" => {
            Command::new("python3")
                .arg("hook.py")
                .arg(fqdn)
                .arg(secret)
                .spawn().unwrap();
        }
        "go" => {
            Command::new("./gohook")
                .arg(fqdn)
                .arg(secret)
                .spawn().unwrap();
        }
        _ => {
            return Json(EspResponse {
            response: "that's not a valid endpoint...".to_string(),
    });
            
        }
    } 

    Json(EspResponse {
        response: "secret received".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // Install a new OpenTelemetry trace pipeline

    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_auto_split_batch(true)
        .with_service_name("hook-endpoint")
        .install_batch(opentelemetry::runtime::Tokio).expect("there");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("TRACE"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init().expect("failed to init tracer");

    let routes_all = Router::new()
        .route("/:lang", get(handler_hook))
        .route("/hello", get(handler_hello))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    info!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}
