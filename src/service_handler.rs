use anyhow::{Context, Result};
use lambda_http::{Body, Request, Response};
use serde_json::json;

use crate::infrastructure::get_rate;

pub async fn get_rate_handler(event: &Request) -> Result<Response<Body>> {
    let id = event.headers().get("ptera-id").unwrap().to_str().unwrap();

    let rate_info = get_rate(id).await.unwrap_or_default();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!(rate_info).to_string().into())
        // .body(json!({"message":"GET /rate", "id": id}).to_string().into())
        .context("Failed to Response body.")?;

    Ok(resp)
}

pub async fn post_rate_handler(event: &Request) -> Result<Response<Body>> {
    todo!()
}
