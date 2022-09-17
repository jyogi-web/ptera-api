use std::{collections::HashMap, convert::TryFrom};

use anyhow::{bail, ensure, Context, Result};
use aws_sdk_dynamodb::model::AttributeValue;

use crate::{entity::RateInfo, CLIENT, CONFIG};

pub(crate) async fn get_rate(id: &str) -> Result<RateInfo> {
    // # Example
    // let item = client.get_item().table_name(&table_name).key(
    //     "user_id",
    //     AttributeValue::S("99315bb2-c1eb-4875-9080-67f41281ea7c".to_string()),
    // );
    // let item = item.send().await?;
    // dbg!(item.item);

    let item_output = CLIENT
        .get()
        .unwrap()
        .get_item()
        .table_name(&CONFIG.table_name)
        .key("user_id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .context("Failed to get_rate send()")?;
    let item_hash_map = item_output.item().context("Failed to get_rate item()")?;

    let rate_info = RateInfo::try_from(item_hash_map)?;

    Ok(rate_info)
}

pub(crate) async fn insert_rate(insert_rate_info: &RateInfo) -> Result<()> {
    let insert_item = CLIENT
        .get()
        .unwrap()
        .put_item()
        .table_name(&CONFIG.table_name)
        .item(
            "user_id",
            AttributeValue::S(insert_rate_info.user_id.to_string()),
        )
        .item(
            "user_name",
            AttributeValue::S(insert_rate_info.user_name.to_string()),
        )
        .item("rate", AttributeValue::N(insert_rate_info.rate.to_string()));

    insert_item
        .send()
        .await
        .context("Failed to insert_rate send()")?;

    Ok(())
}

pub(crate) async fn update_rate(update_rate_info: &RateInfo) -> Result<()> {
    let current_rate = get_rate(&update_rate_info.user_id).await?.rate;
    ensure!(
        update_rate_info.rate.abs_diff(current_rate) <= CONFIG.max_delta_rate,
        "[Invalid value] update_rate_info.rate. current is {}, update is {}, max_delta_rate is {}",
        current_rate,
        update_rate_info.rate,
        CONFIG.max_delta_rate
    );

    let update_item = CLIENT
        .get()
        .unwrap()
        .update_item()
        .table_name(&CONFIG.table_name)
        .key(
            "user_id",
            AttributeValue::S(update_rate_info.user_id.to_string()),
        )
        .update_expression("SET user_name=:user_name")
        .expression_attribute_values(
            ":user_name",
            AttributeValue::S(update_rate_info.user_name.to_string()),
        );

    update_item
        .send()
        .await
        .context("Failed to update_rate send()")?;

    let update_item = CLIENT
        .get()
        .unwrap()
        .update_item()
        .table_name(&CONFIG.table_name)
        .key(
            "user_id",
            AttributeValue::S(update_rate_info.user_id.to_string()),
        )
        .update_expression("SET rate=:rate")
        .expression_attribute_values(
            ":rate",
            AttributeValue::N(update_rate_info.rate.to_string()),
        );

    update_item
        .send()
        .await
        .context("Failed to update_rate send()")?;

    Ok(())
}
