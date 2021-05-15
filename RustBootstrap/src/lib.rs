#![feature(in_band_lifetimes)]

use std::borrow::Borrow;
use std::ffi::CStr;
use std::ptr::null;

use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};

use bridge::*;

use crate::util::JStringUtils;

mod util;

#[macro_use]
mod bridge;
mod MinecraftClient;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_me_hydos_quiltlangrust_entrypoint_RustModInitializer_runNativeInitializer(env: JNIEnv, instance: JObject, libPath: JString, modid: JString) {
    println!("This was printed in rust. if this works then i am concern. also hi java");

    let mut combined_str = modid.into_str(&env);
    combined_str.push_str(" Sounds like an interesting modid");
    println!("{}", combined_str);
    println!("{}", libPath.into_str(&env));

    let logger = class(env, "org/apache/logging/log4j/LogManager").method(
        env,
        "getLogger",
        "(Ljava/lang/String;)Lorg/apache/logging/log4j/Logger;",
        &[JValue::Object(JObject::from(jstring(&env, "Logger made in rust")))],
    );

    let block_registry = class(env, "net.minecraft.util.registry.Registry").field(
        env,
        "BLOCK",
        "Lnet/minecraft/util/registry/DefaultedRegistry;",
    );

    let block_id = class(env, "net.minecraft.util.Identifier").new(
        env,
        "(Ljava/lang/String;)V",
        &[JValue::Object(JObject::from(jstring(&env, "minecraft:corbas")))],
    );

    let block_material = class(env, "net.minecraft.block.Material").field(
        env,
        "PLANT",
        "Lnet/minecraft/block/Material;",
    );

    let block_settings = class(env, "net/minecraft/block/AbstractBlock$Settings").method(
        env,
        "of",
        "(Lnet/minecraft/block/Material;)Lnet/minecraft/block/AbstractBlock$Settings;",
        &[JValue::Object(block_material.as_object().get_raw())],
    );

    let block = class(env, "net/minecraft/block/Block").new(
        env,
        "(Lnet/minecraft/block/AbstractBlock$Settings;)V",
        &[JValue::Object(block_settings.as_object().get_raw())],
    );

    class(env, "net.minecraft.util.registry.Registry").method(
        env,
        "register",
        "(Lnet/minecraft/util/registry/Registry;Lnet/minecraft/util/Identifier;Ljava/lang/Object;)Ljava/lang/Object;",
        &[
            JValue::Object(block_registry.as_object().get_raw()),
            JValue::Object(block_id.as_object().get_raw()),
            JValue::Object(block.as_object().get_raw()),
        ],
    );

    logger.as_object().method(env, "info", "(Ljava/lang/String;)V", &[JValue::Object(JObject::from(jstring(&env, "Hello, im printing this from rust!")))]);
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