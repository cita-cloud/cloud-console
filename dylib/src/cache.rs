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
pub extern "system" fn Java_com_cita_cloud_v1_Console_getBlockNumber(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_block_number(&cache_addr).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getSystemConfig(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_system_config(&cache_addr).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getAbi(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    address: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let address: String = env
        .get_string(&address)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_abi(&cache_addr, &address).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getAccountNonce(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    address: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let address: String = env
        .get_string(&address)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_account_nonce(&cache_addr, &address).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getBalance(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    address: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let address: String = env
        .get_string(&address)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_balance(&cache_addr, &address).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getBlockHash(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    block_number: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let block_number: String = env
        .get_string(&block_number)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_block_hash(&cache_addr, &block_number).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getBlock(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    hash_or_height: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let hash_or_height: String = env
        .get_string(&hash_or_height)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_block(&cache_addr, &hash_or_height).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getCode(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    address: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let address: String = env
        .get_string(&address)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_code(&cache_addr, &address).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getReceipt(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    hash: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let hash: String = env
        .get_string(&hash)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_receipt(&cache_addr, &hash).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_getTx(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    hash: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let hash: String = env
        .get_string(&hash)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(get_tx(&cache_addr, &hash).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_call(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    data: JString,
    from: JString,
    height: JString,
    to: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let data: String = env
        .get_string(&data)
        .expect("Couldn't get java string!")
        .into();
    let from: String = env
        .get_string(&from)
        .expect("Couldn't get java string!")
        .into();
    let height: String = env
        .get_string(&height)
        .expect("Couldn't get java string!")
        .into();
    let to: String = env
        .get_string(&to)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(call(&cache_addr, &data, &from, &height, &to).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_create(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    block_count: JString,
    data: JString,
    value: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let block_count: String = env
        .get_string(&block_count)
        .expect("Couldn't get java string!")
        .into();
    let data: String = env
        .get_string(&data)
        .expect("Couldn't get java string!")
        .into();
    let value: String = env
        .get_string(&value)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(create(&cache_addr, &block_count, &data, &value).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_sendTx(
    mut env: JNIEnv,
    _class: JClass,
    cache_addr: JString,
    block_count: JString,
    data: JString,
    to: JString,
    value: JString,
) -> jstring {
    let cache_addr: String = env
        .get_string(&cache_addr)
        .expect("Couldn't get java string!")
        .into();
    let block_count: String = env
        .get_string(&block_count)
        .expect("Couldn't get java string!")
        .into();
    let data: String = env
        .get_string(&data)
        .expect("Couldn't get java string!")
        .into();
    let to: String = env
        .get_string(&to)
        .expect("Couldn't get java string!")
        .into();
    let value: String = env
        .get_string(&value)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(send_tx(&cache_addr, &block_count, &data, &to, &value).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}
