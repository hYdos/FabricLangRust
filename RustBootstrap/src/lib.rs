use std::borrow::Borrow;
use std::ffi::CStr;

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use crate::util::JStringUtils;

mod util;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_me_hydos_quiltlangrust_entrypoint_RustModInitializer_runNativeInitializer(env: JNIEnv, instance: JObject, libPath: JString, modid: JString) {
    println!("This was printed in rust. if this works then i am concern. also hi java");

    let mut combined_str = modid.into_str(&env);
    combined_str.push_str(" Sounds like an interesting modid");
    println!("{}", combined_str);
    println!("{}", libPath.into_str(&env));
}

/// This is the way to interact with all things java related (most mods and Minecraft)
impl Minecraft {
    /// Calls LogManager.getLogger(logger_name).info(message);
    fn log(logger_name: &str, message: &str) {}

    /// Gets an instance of the minecraft client. this is should NOT be called on the server
    fn get_client() -> MinecraftClient {
        return MinecraftClient {};
    }

    /// Gets an instance of the minecraft dedicated server. this is should NOT be called on the client
    fn get_server() -> MinecraftDedicatedServer {
        return MinecraftDedicatedServer {};
    }
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}