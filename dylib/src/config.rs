use console::config::update_chain_config;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_Console_updateChainConfig(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(update_chain_config(input).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}
