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

use console::cache::*;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getBlockNumber(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    env.new_string(get_block_number().to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getSystemConfig(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    env.new_string(get_system_config().to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getAbi(
    env: JNIEnv,
    _class: JClass,
    address: JString,
) -> jstring {
    let address: String = env
        .get_string(address)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_abi(&address).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getAccountNonce(
    env: JNIEnv,
    _class: JClass,
    address: JString,
) -> jstring {
    let address: String = env
        .get_string(address)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_account_nonce(&address).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getBalance(
    env: JNIEnv,
    _class: JClass,
    address: JString,
) -> jstring {
    let address: String = env
        .get_string(address)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_balance(&address).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getBlockHash(
    env: JNIEnv,
    _class: JClass,
    block_number: JString,
) -> jstring {
    let block_number: String = env
        .get_string(block_number)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_block_hash(&block_number).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getBlock(
    env: JNIEnv,
    _class: JClass,
    hash_or_height: JString,
) -> jstring {
    let hash_or_height: String = env
        .get_string(hash_or_height)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_block(&hash_or_height).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getCode(
    env: JNIEnv,
    _class: JClass,
    address: JString,
) -> jstring {
    let address: String = env
        .get_string(address)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_code(&address).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getReceipt(
    env: JNIEnv,
    _class: JClass,
    hash: JString,
) -> jstring {
    let hash: String = env
        .get_string(hash)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_receipt(&hash).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_getTx(
    env: JNIEnv,
    _class: JClass,
    hash: JString,
) -> jstring {
    let hash: String = env
        .get_string(hash)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_tx(&hash).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_call(
    env: JNIEnv,
    _class: JClass,
    data: JString,
    from: JString,
    height: JString,
    to: JString,
) -> jstring {
    let data: String = env
        .get_string(data)
        .expect("Couldn't get java string!")
        .into();
    let from: String = env
        .get_string(from)
        .expect("Couldn't get java string!")
        .into();
    let height: String = env
        .get_string(height)
        .expect("Couldn't get java string!")
        .into();
    let to: String = env
        .get_string(to)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(call(&data, &from, &height, &to).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_create(
    env: JNIEnv,
    _class: JClass,
    block_count: JString,
    data: JString,
    value: JString,
) -> jstring {
    let block_count: String = env
        .get_string(block_count)
        .expect("Couldn't get java string!")
        .into();
    let data: String = env
        .get_string(data)
        .expect("Couldn't get java string!")
        .into();
    let value: String = env
        .get_string(value)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(create(&block_count, &data, &value).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_ConsoleV1_sendTx(
    env: JNIEnv,
    _class: JClass,
    block_count: JString,
    data: JString,
    to: JString,
    value: JString,
) -> jstring {
    let block_count: String = env
        .get_string(block_count)
        .expect("Couldn't get java string!")
        .into();
    let data: String = env
        .get_string(data)
        .expect("Couldn't get java string!")
        .into();
    let to: String = env
        .get_string(to)
        .expect("Couldn't get java string!")
        .into();
    let value: String = env
        .get_string(value)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(send_tx(&block_count, &data, &to, &value).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}
