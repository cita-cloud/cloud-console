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
        hex::decode(fs::read("console/src/private_key").expect("ensure file private_key is exist"))
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap(),
    );
    MultiCryptoAccount::from(account)
}
