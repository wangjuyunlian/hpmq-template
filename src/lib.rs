#![allow(unused_imports)]
use hpmq::anyhow::{anyhow, bail, Result};
use hpmq::HandResult;
use hpmq::{sdk_init, serde_json, wapc_guest, PublishDatas};
use std::collections::HashMap;

sdk_init!();

/// (下行)云平台发布 转换
#[hpmq::downstream]
fn custom_cloud_publish(
    topic: String,
    _payload: &[u8],
    his_payload: Option<Vec<u8>>,
    _context: HashMap<String, String>,
) -> Result<HandResult> {
    if let Some(his) = his_payload {
        if his.as_slice() == _payload {
            println!("云平台发布消息重复，不进行投递");
            return Ok(HandResult::Discard);
        }
    }
    println!("云平台发布新消息，进行投递");
    Ok(HandResult::Transmit((
        topic.as_bytes().to_vec(),
        _payload.to_vec(),
    )))
}

/// （上行）设备发布 转换
#[hpmq::upstream]
fn custom_local_publish(
    topic: String,
    _payload: &[u8],
    his_payload: Option<Vec<u8>>,
    _context: HashMap<String, String>,
) -> Result<HandResult> {
    if let Some(his) = his_payload {
        if his.as_slice() == _payload {
            println!("设备发布消息重复，不进行投递");
            return Ok(HandResult::Discard);
        }
    }
    println!("设备发布新消息，进行投递");
    Ok(HandResult::Transmit((
        topic.as_bytes().to_vec(),
        _payload.to_vec(),
    )))
}
