use std::borrow::Borrow;
use std::ffi::CStr;

use jni::JNIEnv;
use jni::objects::{JObject, JString};

use macros::*;

use crate::util::JStringUtils;

mod util;

#[macro_use]
mod macros;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_me_hydos_quiltlangrust_entrypoint_RustModInitializer_runNativeInitializer(env: JNIEnv, instance: JObject, libPath: JString, modid: JString) {
    println!("This was printed in rust. if this works then i am concern. also hi java");

    let mut combined_str = modid.into_str(&env);
    combined_str.push_str(" Sounds like an interesting modid");
    println!("{}", combined_str);
    println!("{}", libPath.into_str(&env));

    class!("test2");
}

/// Called when the java ModInitializer is
trait ModInitializer {
    fn on_initialize();
}

/// Called when the java ServerModInitializer is
trait ServerModInitializer {
    fn on_initialize_server();
}

/// Called when the java ClientModInitializer is
trait ClientModInitializer {
    fn on_initialize_client();
}