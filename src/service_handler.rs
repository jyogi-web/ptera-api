use std::slice::RSplit;

use anyhow::{Context, Result};
use lambda_http::{Body, Request, Response};
use serde_json::json;

use crate::{
    bll::rate_calculation,
    entity::{RateInfo, RateInfoList},
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
    let r: RateInfoList = serde_json::from_str(
        r#"{
    "rate_info_list": [
        {
            "user_id": "3d03c5e3-d771-4cf5-b6fa-76f9255101fc",
            "user_name": "test1",
            "rate": 550,
            "is_winner": true
        },
        {
            "user_id": "a8d13189-351d-42ab-8c5e-1c1fcd7a8a81",
            "user_name": "test2",
            "rate": 400,
            "is_winner": false
        }
    ]
}"#,
    )
    .unwrap();
    log::debug!("{:?}", r);
    log::debug!("{:?}", serde_json::to_string(&r).unwrap());
    todo!();
    let rate_info_list =
        String::from_utf8(event.body().to_vec()).context("Failed to vec to String")?;
    log::debug!("{:?}", rate_info_list.trim());
    let mut rate_info_list: RateInfoList =
        serde_json::from_str(&rate_info_list).context("Failed to str to json")?;

    if rate_info_list[0].is_winner.unwrap_or_default() {
        let (win, lose) = rate_calculation(rate_info_list[0].rate, rate_info_list[1].rate);
        rate_info_list[0].rate = win;
        rate_info_list[1].rate = lose;
    } else if rate_info_list[1].is_winner.unwrap_or_default() {
        let (win, lose) = rate_calculation(rate_info_list[1].rate, rate_info_list[0].rate);
        rate_info_list[0].rate = lose;
        rate_info_list[1].rate = win;
    } else {
        // 不正なリクエスト
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(
                json!({"message": "json format is invalid."})
                    .to_string()
                    .into(),
            )
            .context("Failed to Response body.")?);
    }

    update_rate(&rate_info_list[0]).await?;
    update_rate(&rate_info_list[1]).await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!({"message": "success"}).to_string().into())
        .context("Failed to Response body.")?;

    Ok(resp)
}
