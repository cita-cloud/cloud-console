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

use cita_cloud_proto::client::ClientOptions;
use cita_cloud_proto::client::InterceptedSvc;
use cita_cloud_proto::crypto::crypto_service_client::CryptoServiceClient;
use cldi::core::controller::SignerBehaviour;
use cldi::crypto::Address;
use cldi::utils::parse_addr;
use cloud_util::crypto::{hash_data, sign_message};
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

pub const CLIENT_NAME: &str = "console";

#[derive(Debug, Clone)]
pub struct CryptoClient {
    retry_client:
        OnceCell<cita_cloud_proto::retry::RetryClient<CryptoServiceClient<InterceptedSvc>>>,
    admin_addr: Address,
}

impl CryptoClient {
    pub async fn get_crypto_signer(crypto_addr: String, admin_addr: String) -> Self {
        let crypto_client = OnceCell::new_with(Some({
            let client_options =
                ClientOptions::new(CLIENT_NAME.to_string(), format!("http://{crypto_addr}"));
            match client_options.connect_crypto() {
                Ok(retry_client) => retry_client,
                Err(e) => panic!("client init error: {:?}", &e),
            }
        }));
        Self {
            retry_client: crypto_client,
            admin_addr: parse_addr(admin_addr.as_str()).unwrap(),
        }
    }
}

impl SignerBehaviour for CryptoClient {
    fn hash(&self, msg: &[u8]) -> Vec<u8> {
        let client = self.clone();
        let msg = msg.to_owned();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(hash_data(client.retry_client.get().cloned().unwrap(), &msg))
        })
        .join()
        .expect("Thread panicked")
        .unwrap()
    }

    fn address(&self) -> &[u8] {
        &self.admin_addr
    }

    fn sign(&self, msg: &[u8]) -> Vec<u8> {
        let client = self.clone();
        let msg = msg.to_owned();
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(sign_message(
                client.retry_client.get().cloned().unwrap(),
                &msg,
            ))
        })
        .join()
        .expect("Thread panicked")
        .unwrap()
    }
}
