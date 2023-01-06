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

use console::cli::*;
use jni::objects::{JClass, JString};
use jni::sys::{jobject, jstring};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_updateAdmin(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    admin_addr: JString,
    new_admin: JString,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let admin_addr: String = env
        .get_string(admin_addr)
        .expect("Couldn't get java string!")
        .into();
    let new_admin: String = env
        .get_string(new_admin)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(update_admin(controller_addr, crypto_addr, admin_addr, new_admin).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_setBlockInterval(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    admin_addr: JString,
    block_interval: JString,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let admin_addr: String = env
        .get_string(admin_addr)
        .expect("Couldn't get java string!")
        .into();
    let block_interval: String = env
        .get_string(block_interval)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(
        set_block_interval(controller_addr, crypto_addr, admin_addr, block_interval).to_json(),
    )
    .expect("Couldn't create java string!")
    .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_updateValidators(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    admin_addr: JString,
    validators_jobject: jobject,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let admin_addr: String = env
        .get_string(admin_addr)
        .expect("Couldn't get java string!")
        .into();
    let validators_len = env.get_array_length(validators_jobject).unwrap();
    let mut validators: Vec<String> = Vec::with_capacity(validators_len as usize);
    for i in 0..validators_len {
        let j_object = env.get_object_array_element(validators_jobject, i).unwrap();
        let validator = env
            .get_string(JString::from(j_object))
            .expect("Couldn't get java string!")
            .into();
        validators.push(validator);
    }

    env.new_string(
        update_validators(controller_addr, crypto_addr, admin_addr, validators).to_json(),
    )
    .expect("Couldn't create java string!")
    .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_emergencyBrake(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    admin_addr: JString,
    switch: JString,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let admin_addr: String = env
        .get_string(admin_addr)
        .expect("Couldn't get java string!")
        .into();
    let switch: String = env
        .get_string(switch)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(emergency_brake(controller_addr, crypto_addr, admin_addr, switch).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_setQuotaLimit(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    admin_addr: JString,
    quota_limit: JString,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let admin_addr: String = env
        .get_string(admin_addr)
        .expect("Couldn't get java string!")
        .into();
    let quota_limit: String = env
        .get_string(quota_limit)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(set_quota_limit(controller_addr, crypto_addr, admin_addr, quota_limit).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}
