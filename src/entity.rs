use anyhow::{bail, Context};
use std::{
    collections::HashMap,
    convert::TryFrom,
    ops::{Index, IndexMut},
};

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct RateInfo {
    pub(crate) user_id: String,
    pub(crate) user_name: String,
    pub(crate) rate: u64,
    pub(crate) is_winner: Option<bool>,
}

impl TryFrom<&HashMap<String, AttributeValue>> for RateInfo {
    type Error = anyhow::Error;
    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let converted = match (
            value.get("user_id"),
            value.get("user_name"),
            value.get("rate"),
        ) {
            (Some(user_id), Some(user_name), Some(rate)) => {
                let user_id = if let AttributeValue::S(u) = user_id {
                    u.to_string()
                } else {
                    bail!("Not match user_id AttributeValue.");
                };
                let user_name = if let AttributeValue::S(n) = user_name {
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
                    user_name,
                    rate,
                    is_winner: None,
                })
            }
            _ => bail!("Not Found key item"),
        };

        converted
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct RateInfoList {
    pub(crate) rate_info_list: Vec<RateInfo>,
}

impl Index<usize> for RateInfoList {
    type Output = RateInfo;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rate_info_list[index]
    }
}

impl IndexMut<usize> for RateInfoList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rate_info_list[index]
    }
}
