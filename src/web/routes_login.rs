use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/robot", post(robot_login))
}

async fn robot_login(payload: Json<RobotLogin>) -> Result<Json<Value>> {
    println!("->> {:12} - robot_login", "HANDLER");
    if payload.address != "my-robot.viam.com" || 
    payload.secret != "leeeerrroooyyyy" {
        return Err(Error::LoginFail);
    }

    // set cookies?
    let body = Json(json!({
        "result": {
        "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct RobotLogin {
    address: String, 
    secret: String,
}
