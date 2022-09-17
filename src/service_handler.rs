use anyhow::{Context, Result};
use lambda_http::{Body, Request, Response};
use serde_json::json;

pub fn get_rate(event: &Request) -> Result<Response<Body>> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!({"message":"GET /rate"}).to_string().into())
        .context("Failed to Response body.")?;

    Ok(resp)
}
