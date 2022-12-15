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

pub mod cli;
pub mod config;
pub mod crypto;

use cldi::core::{
    client::GrpcClientBehaviour,
    controller::ControllerClient,
    wallet::{Account, MultiCryptoAccount},
};
use cldi::crypto::SmCrypto;
use std::fs;
use std::time::Duration;

pub async fn controller_client(addr: String) -> ControllerClient {
    GrpcClientBehaviour::connect_lazy(addr.as_str(), Duration::from_secs(3)).unwrap()
}

pub fn get_local_signer() -> MultiCryptoAccount {
    let account: Account<SmCrypto> = Account::from_secret_key(
        hex::decode(fs::read("./private_key").expect("ensure file private_key is exist"))
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap(),
    );
    MultiCryptoAccount::from(account)
}
