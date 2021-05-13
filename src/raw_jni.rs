use jni::JNIEnv;
use jni::objects::{JObject, JString};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_me_hydos_quiltlangrust_entrypoint_RustModInitializer_runNativeInitializer(
    env: JNIEnv,
    instance: JObject,
    libPath: JString,
    modid: JString) {
    println!("This was printed in rust. if this works then i am concern. also hi java");
}