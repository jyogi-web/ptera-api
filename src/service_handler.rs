use std::slice::RSplit;

use anyhow::{Context, Result};
use lambda_http::{Body, Request, Response};
use serde_json::json;

use crate::{
    entity::RateInfo,
    infrastructure::{get_rate, insert_rate, update_rate},
};

pub(crate) const DEFAULT_RATE: u64 = 0;

pub async fn get_rate_handler(event: &Request) -> Result<Response<Body>> {
    let id = event.headers().get("ptera-id").unwrap().to_str().unwrap();

    let rate_info = get_rate(id).await;
    let resp = match rate_info {
        Ok(rate_info) => Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(json!(rate_info).to_string().into())
            .context("Failed to Response body.")?,
        Err(e) => {
            log::debug!("{}", &e);
            Response::builder()
                .status(204)
                .header("content-type", "application/json")
                .body(json!(RateInfo::default()).to_string().into())
                .context("Failed to Response body.")?
        }
    };

    Ok(resp)
}

pub async fn post_rate_handler(event: &Request) -> Result<Response<Body>> {
    let insert_rate_info =
        String::from_utf8(event.body().to_vec()).context("Failed to vec to String")?;
    log::debug!("{}", &insert_rate_info);
    let mut insert_rate_info: RateInfo =
        serde_json::from_str(&insert_rate_info).context("Failed to str to json")?;
    insert_rate_info.rate = DEFAULT_RATE;
    log::debug!("[Body] {:#?}", insert_rate_info);

    insert_rate(&insert_rate_info).await?;
    log::info!("Inserted rate");

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&insert_rate_info)
                .context("Failed to serde json")?
                .into(),
        )
        .context("Failed to Response body.")?;

    Ok(resp)
}

pub async fn put_rate_handler(event: &Request) -> Result<Response<Body>> {
    let update_rate_info =
        String::from_utf8(event.body().to_vec()).context("Failed to vec to String")?;
    let update_rate_info: RateInfo =
        serde_json::from_str(&update_rate_info).context("Failed to str to json")?;
    log::debug!("[Body] {:#?}", update_rate_info);

    update_rate(&update_rate_info).await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&update_rate_info)
                .context("Failed to serde json")?
                .into(),
        )
        .context("Failed to Response body.")?;

    Ok(resp)
}

pub async fn post_rate_calculation_handler(event: &Request) -> Result<Response<Body>> {
    todo!()
}
