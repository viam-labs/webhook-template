use std::net::SocketAddr;
use std::process::Command;
use serde::{Serialize, Deserialize};

use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct EspParams {
    location: Option<String>,
    api_key_id: Option<String>,
    api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EspResponse {
    response: String,
}

async fn handler_hook(Path(lang): Path<String>, Json(payload): Json<EspParams>) -> Json<EspResponse> {
    let fqdn = payload.location.unwrap_or("url".to_string());
    let api_key_id = payload.api_key_id.unwrap_or("api-key-id".to_string());
    let api_key = payload.api_key.unwrap_or("api-key".to_string());
    match lang.as_str() {
        "py" => {
            Command::new("python3")
                .arg("hook.py")
                .arg(fqdn)
                .arg(api_key_id)
                .arg(api_key)
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
    let routes_all = Router::new()
        .route("/:lang", get(handler_hook));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}
