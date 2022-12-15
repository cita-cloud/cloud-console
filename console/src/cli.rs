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
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::controller_client;
use crate::crypto::CryptoClient;
use crate::get_local_signer;
use cldi::core::admin::AdminBehaviour;
use cldi::crypto::Address;
use cldi::display::Display;
use cldi::utils::parse_addr;
use cldi::utils::parse_validator_addr;
use common::{response::Response, status_code::StatusCode};
use tokio::runtime::Runtime;

pub fn update_admin(
    controller_addr: String,
    crypto_addr: String,
    account_addr: String,
    new_admin: String,
) -> Response<String> {
    let new_admin: Address = parse_addr(new_admin.as_str()).unwrap();

    let rt = Runtime::new().unwrap();
    let controller_client = rt.block_on(controller_client(controller_addr));

    match if crypto_addr.is_empty() {
        rt.block_on(AdminBehaviour::update_admin(
            &controller_client,
            &get_local_signer(),
            new_admin,
        ))
    } else {
        let crypto_signer = rt.block_on(CryptoClient::connect(crypto_addr, account_addr));
        rt.block_on(AdminBehaviour::update_admin(
            &controller_client,
            &crypto_signer,
            new_admin,
        ))
    } {
        Ok(hash) => Response::new(StatusCode::Success, hash.display()),
        Err(e) => Response::new(StatusCode::Error, format!("{}", e)),
    }
}

pub fn set_block_interval(
    controller_addr: String,
    crypto_addr: String,
    account_addr: String,
    block_interval: String,
) -> Response<String> {
    let block_interval = block_interval.parse::<u32>().unwrap();

    let rt = Runtime::new().unwrap();
    let controller_client = rt.block_on(controller_client(controller_addr));

    match if crypto_addr.is_empty() {
        rt.block_on(AdminBehaviour::set_block_interval(
            &controller_client,
            &get_local_signer(),
            block_interval,
        ))
    } else {
        let crypto_signer = rt.block_on(CryptoClient::connect(crypto_addr, account_addr));
        rt.block_on(AdminBehaviour::set_block_interval(
            &controller_client,
            &crypto_signer,
            block_interval,
        ))
    } {
        Ok(hash) => Response::new(StatusCode::Success, hash.display()),
        Err(e) => Response::new(StatusCode::Error, format!("{}", e)),
    }
}

pub fn update_validators(
    controller_addr: String,
    crypto_addr: String,
    account_addr: String,
    validators: Vec<String>,
) -> Response<String> {
    let validators = validators
        .into_iter()
        .map(|v| parse_validator_addr(v.as_str()).unwrap())
        .collect::<Vec<Vec<u8>>>();
    let rt = Runtime::new().unwrap();
    let controller_client = rt.block_on(controller_client(controller_addr));

    match if crypto_addr.is_empty() {
        rt.block_on(AdminBehaviour::update_validators(
            &controller_client,
            &get_local_signer(),
            &validators,
        ))
    } else {
        let crypto_signer = rt.block_on(CryptoClient::connect(crypto_addr, account_addr));
        rt.block_on(AdminBehaviour::update_validators(
            &controller_client,
            &crypto_signer,
            &validators,
        ))
    } {
        Ok(hash) => Response::new(StatusCode::Success, hash.display()),
        Err(e) => Response::new(StatusCode::Error, format!("{}", e)),
    }
}

pub fn emergency_brake(
    controller_addr: String,
    crypto_addr: String,
    account_addr: String,
    switch: String,
) -> Response<String> {
    let switch = match switch.as_str() {
        "on" => true,
        "off" => false,
        _ => panic!("unexpected value"),
    };

    let rt = Runtime::new().unwrap();
    let controller_client = rt.block_on(controller_client(controller_addr));
    match if crypto_addr.is_empty() {
        rt.block_on(AdminBehaviour::emergency_brake(
            &controller_client,
            &get_local_signer(),
            switch,
        ))
    } else {
        let crypto_signer = rt.block_on(CryptoClient::connect(crypto_addr, account_addr));
        rt.block_on(AdminBehaviour::emergency_brake(
            &controller_client,
            &crypto_signer,
            switch,
        ))
    } {
        Ok(hash) => Response::new(StatusCode::Success, hash.display()),
        Err(e) => Response::new(StatusCode::Error, format!("{}", e)),
    }
}

pub fn set_quota_limit(
    controller_addr: String,
    crypto_addr: String,
    account_addr: String,
    quota_limit: String,
) -> Response<String> {
    let quota_limit = quota_limit.parse::<u64>().unwrap();

    let rt = Runtime::new().unwrap();
    let controller_client = rt.block_on(controller_client(controller_addr));

    match if crypto_addr.is_empty() {
        rt.block_on(AdminBehaviour::set_quota_limit(
            &controller_client,
            &get_local_signer(),
            quota_limit,
        ))
    } else {
        let crypto_signer = rt.block_on(CryptoClient::connect(crypto_addr, account_addr));
        rt.block_on(AdminBehaviour::set_quota_limit(
            &controller_client,
            &crypto_signer,
            quota_limit,
        ))
    } {
        Ok(hash) => Response::new(StatusCode::Success, hash.display()),
        Err(e) => Response::new(StatusCode::Error, format!("{}", e)),
    }
}
