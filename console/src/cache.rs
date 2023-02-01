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
        return Response::error(StatusCode::ParameterError, resp["message"].to_string().trim_matches('"'));
    };
    Response::new(
        StatusCode::Success,
        resp["data"].to_string().trim_matches('"').to_string(),
    )
}

fn http_get(url: &str) -> Response<String> {
    let rt = Runtime::new().unwrap();
    let http_response = rt.block_on(reqwest::get(url));
    rt.block_on(handle_response(http_response))
}

fn http_post(url: &str, body: &str) -> Response<String> {
    let rt = Runtime::new().unwrap();
    let http_response = rt.block_on(
        reqwest::Client::new()
            .post(url)
            .body(body.to_owned())
            .send(),
    );
    rt.block_on(handle_response(http_response))
}

pub fn get_block_number(cache_addr: &str) -> Response<String> {
    http_get(&format!("{}/{}", cache_addr, "api/get-block-number"))
}

pub fn get_system_config(cache_addr: &str) -> Response<String> {
    http_get(&format!("{}/{}", cache_addr, "api/get-system-config"))
}

pub fn get_abi(cache_addr: &str, address: &str) -> Response<String> {
    http_get(&format!("{}/{}/{}", cache_addr, "api/get-abi", address))
}

pub fn get_account_nonce(cache_addr: &str, address: &str) -> Response<String> {
    http_get(&format!(
        "{}/{}/{}",
        cache_addr, "api/get-account-nonce", address
    ))
}

pub fn get_balance(cache_addr: &str, address: &str) -> Response<String> {
    http_get(&format!("{}/{}/{}", cache_addr, "api/get-balance", address))
}

pub fn get_block_hash(cache_addr: &str, block_number: &str) -> Response<String> {
    http_get(&format!(
        "{}/{}/{}",
        cache_addr, "api/get-block-hash", block_number
    ))
}

pub fn get_block(cache_addr: &str, hash_or_height: &str) -> Response<String> {
    http_get(&format!(
        "{}/{}/{}",
        cache_addr, "api/get-block", hash_or_height
    ))
}

pub fn get_code(cache_addr: &str, address: &str) -> Response<String> {
    http_get(&format!("{}/{}/{}", cache_addr, "api/get-code", address))
}

pub fn get_receipt(cache_addr: &str, hash: &str) -> Response<String> {
    http_get(&format!("{}/{}/{}", cache_addr, "api/get-receipt", hash))
}

pub fn get_tx(cache_addr: &str, hash: &str) -> Response<String> {
    http_get(&format!("{}/{}/{}", cache_addr, "api/get-tx", hash))
}

pub fn call(cache_addr: &str, data: &str, from: &str, height: &str, to: &str) -> Response<String> {
    let body = format!(
        r#"{{
        "data": "{data}",
        "from": "{from}",
        "height": {height},
        "to": "{to}"
      }}"#
    );
    http_post(&format!("{}/{}", cache_addr, "api/call"), &body)
}

pub fn create(cache_addr: &str, block_count: &str, data: &str, value: &str) -> Response<String> {
    let body = format!(
        r#"{{
        "block_count": {block_count},
        "data": "{data}",
        "value": "{value}"
      }}"#
    );
    http_post(&format!("{}/{}", cache_addr, "api/create"), &body)
}

pub fn send_tx(
    cache_addr: &str,
    block_count: &str,
    data: &str,
    to: &str,
    value: &str,
) -> Response<String> {
    let body = format!(
        r#"{{
        "block_count": {block_count},
        "data": "{data}",
        "to": "{to}",
        "value": "{value}"
      }}"#
    );
    http_post(&format!("{}/{}", cache_addr, "api/sendTx"), &body)
}
