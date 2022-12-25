// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific

use common::{response::Response, status_code::StatusCode};
use reqwest::Error;
use reqwest::Response as HttpResponse;
use serde_json::Value;

const CACHE_ADDR: &str = "http://192.168.10.120:30067";

async fn handle_response(resp: Result<HttpResponse, Error>) -> Response<String> {
    let Ok(resp) = resp else {
        return StatusCode::NetworkError.into();
    };
    let Ok(resp) = resp.json::<Value>().await else {
        return StatusCode::ResponseToJsonError.into();
    };
    let Some(status) = resp["status"].as_u64() else {
        return StatusCode::ApiNotExist.into();
    };
    let 1 = status else {
        return StatusCode::ParameterError.into();
    };
    Response::new(StatusCode::Success, resp["data"].to_string())
}

async fn http_get(api: &str) -> Response<String> {
    let http_response = reqwest::get(format!("{}/{}", CACHE_ADDR, api)).await;
    handle_response(http_response).await
}

async fn http_post(api: &str, body: &str) -> Response<String> {
    let http_response = reqwest::Client::new()
        .post(format!("{}/{}", CACHE_ADDR, api))
        .body(body.to_owned())
        .send()
        .await;
    handle_response(http_response).await
}

pub async fn get_block_number() -> Response<String> {
    http_get("api/get-block-number").await
}

pub async fn get_system_config() -> Response<String> {
    http_get("api/get-system-config").await
}

pub async fn get_abi(address: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-abi", address)).await
}

pub async fn get_account_nonce(address: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-account-nonce", address)).await
}

pub async fn get_balance(address: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-balance", address)).await
}

pub async fn get_block_hash(block_number: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-block-hash", block_number)).await
}

pub async fn get_block(hash_or_height: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-block", hash_or_height)).await
}

pub async fn get_code(address: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-code", address)).await
}

pub async fn get_receipt(hash: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-receipt", hash)).await
}

pub async fn get_tx(hash: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-tx", hash)).await
}

pub async fn call(data: &str, from: &str, height: &str, to: &str) -> Response<String> {
    let body = format!(
        r#"{{
        "data": "{}",
        "from": "{}",
        "height": {},
        "to": "{}"
      }}"#,
        data, from, height, to
    );
    http_post("api/call", &body).await
}

pub async fn create(block_count: &str, data: &str, value: &str) -> Response<String> {
    let body = format!(
        r#"{{
        "block_count": {},
        "data": "{}",
        "value": "{}"
      }}"#,
        block_count, data, value
    );
    http_post("api/create", &body).await
}

pub async fn send_tx(block_count: &str, data: &str, to: &str, value: &str) -> Response<String> {
    let body = format!(
        r#"{{
        "block_count": {},
        "data": "{}",
        "to": "{}",
        "value": "{}"
      }}"#,
        block_count, data, to, value
    );
    http_post("api/sendTx", &body).await
}
