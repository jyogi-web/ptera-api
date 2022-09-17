use anyhow::{bail, Context};
use std::{collections::HashMap, convert::TryFrom};

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct RateInfo {
    user_id: String,
    name: String,
    rate: u64,
}

impl TryFrom<&HashMap<String, AttributeValue>> for RateInfo {
    type Error = anyhow::Error;
    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let converted = match (value.get("user_id"), value.get("name"), value.get("rate")) {
            (Some(user_id), Some(name), Some(rate)) => {
                let user_id = if let AttributeValue::S(u) = user_id {
                    u.to_string()
                } else {
                    bail!("Not match user_id AttributeValue.");
                };
                let name = if let AttributeValue::S(n) = name {
                    n.to_string()
                } else {
                    bail!("Not match name AttributeValue.");
                };
                let rate = if let AttributeValue::N(r) = rate {
                    r.parse().context("Failed to parse rate")?
                } else {
                    bail!("Not match rate AttributeValue.");
                };

                Ok(Self {
                    user_id,
                    name,
                    rate,
                })
            }
            _ => bail!("Not Found key item"),
        };

        converted
    }
}
