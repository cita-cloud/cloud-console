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
use tokio::runtime::Runtime;

const CACHE_ADDR: &str = "http://192.168.10.120:30067";

async fn handle_response(resp: Result<HttpResponse, Error>) -> Response<String> {
    let Ok(resp) = resp else {
        return StatusCode::UrlError.into();
    };
    let Ok(resp) = resp.json::<Value>().await else {
        return StatusCode::BodyIsNotJson.into();
    };
    let Some(status) = resp["status"].as_u64() else {
        return StatusCode::ApiNotExist.into();
    };
    let 1 = status else {
        return Response::new(StatusCode::ParameterError, resp["message"].to_string());
    };
    Response::new(StatusCode::Success, resp["data"].to_string())
}

fn http_get(api: &str) -> Response<String> {
    let rt = Runtime::new().unwrap();
    let http_response = rt.block_on(reqwest::get(format!("{}/{}", CACHE_ADDR, api)));
    rt.block_on(handle_response(http_response))
}

fn http_post(api: &str, body: &str) -> Response<String> {
    let rt = Runtime::new().unwrap();
    let http_response = rt.block_on(
        reqwest::Client::new()
            .post(format!("{}/{}", CACHE_ADDR, api))
            .body(body.to_owned())
            .send(),
    );
    rt.block_on(handle_response(http_response))
}

pub fn get_block_number() -> Response<String> {
    http_get("api/get-block-number")
}

pub fn get_system_config() -> Response<String> {
    http_get("api/get-system-config")
}

pub fn get_abi(address: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-abi", address))
}

pub fn get_account_nonce(address: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-account-nonce", address))
}

pub fn get_balance(address: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-balance", address))
}

pub fn get_block_hash(block_number: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-block-hash", block_number))
}

pub fn get_block(hash_or_height: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-block", hash_or_height))
}

pub fn get_code(address: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-code", address))
}

pub fn get_receipt(hash: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-receipt", hash))
}

pub fn get_tx(hash: &str) -> Response<String> {
    http_get(&format!("{}/{}", "api/get-tx", hash))
}

pub fn call(data: &str, from: &str, height: &str, to: &str) -> Response<String> {
    let body = format!(
        r#"{{
        "data": "{}",
        "from": "{}",
        "height": {},
        "to": "{}"
      }}"#,
        data, from, height, to
    );
    http_post("api/call", &body)
}

pub fn create(block_count: &str, data: &str, value: &str) -> Response<String> {
    let body = format!(
        r#"{{
        "block_count": {},
        "data": "{}",
        "value": "{}"
      }}"#,
        block_count, data, value
    );
    http_post("api/create", &body)
}

pub fn send_tx(block_count: &str, data: &str, to: &str, value: &str) -> Response<String> {
    let body = format!(
        r#"{{
        "block_count": {},
        "data": "{}",
        "to": "{}",
        "value": "{}"
      }}"#,
        block_count, data, to, value
    );
    http_post("api/sendTx", &body)
}
