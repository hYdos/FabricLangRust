#![feature(in_band_lifetimes)]

use std::borrow::Borrow;
use std::ffi::CStr;
use std::ptr::null;

use bridge::*;

use crate::util::JStringUtils;

mod util;

#[macro_use]
mod bridge;
mod MinecraftClient;

#[robusta_jni::bridge]
mod some_jni {
    use robusta_jni::convert::{Signature, IntoJavaValue, FromJavaValue, TryIntoJavaValue, TryFromJavaValue, Field};
    use robusta_jni::jni::JNIEnv;
    use robusta_jni::jni::objects::{AutoLocal, JValue};
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::errors::Error as JniError;
    use robusta_jni::jni::objects::JObject;
    use crate::bridge::class;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(org.apache.logging.log4j)]
    pub struct LogManager<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    impl<'env: 'borrow, 'borrow> LogManager<'env, 'borrow> {
        pub extern "java" fn getLogger(env: &'borrow JNIEnv<'env>, name: String) -> JniResult<Logger<'env, 'borrow>> { }
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(org.apache.logging.log4j)]
    pub struct Logger<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    impl<'env: 'borrow, 'borrow> Logger<'env, 'borrow> {
        pub extern "java" fn info(&self, env: &'borrow JNIEnv<'env>, msg: String) -> JniResult<()> { }
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(net.minecraft.util.registry)]
    pub struct Registry<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    impl<'env: 'borrow, 'borrow> Registry<'env, 'borrow> {
        fn getBlock(env: &'borrow JNIEnv<'env>) -> JniResult<DefaultedRegistry<'env, 'borrow>> {
            env.get_static_field(
                "net/minecraft/util/registry/Registry",
                "BLOCK",
                <DefaultedRegistry as Signature>::SIG_TYPE
            ).and_then(|val| DefaultedRegistry::try_from(val.l()?, env))
        }

        // TODO: actually be able to pass/return object
        pub extern "java" fn register(&self, env: &'borrow JNIEnv<'env>, ident: Identifier, obj: ()) -> JniResult<()> {}
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(net.minecraft.util.registry)]
    pub struct DefaultedRegistry<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(net.minecraft.util)]
    pub struct Identifier<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    impl<'env: 'borrow, 'borrow> Identifier<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>, name: String) -> JniResult<Self> {}
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(net.minecraft.block)]
    pub struct Material<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    impl<'env: 'borrow, 'borrow> Material<'env, 'borrow> {
        fn getPlant(env: &'borrow JNIEnv<'env>) -> JniResult<Material<'env, 'borrow>> {
            env.get_static_field(
                "net/minecraft/block/Material",
                "PLANT",
                <Material as Signature>::SIG_TYPE
            ).and_then(|val| Material::try_from(val.l()?, env))
        }
    }

    #[derive(TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(net.minecraft.block)]
    pub struct BlockSettings<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    impl<'env: 'borrow, 'borrow> Signature for BlockSettings<'env, 'borrow> {
        const SIG_TYPE: &'static str = "Lnet/minecraft/block/AbstractBlock$Settings;";
    }
    impl<'env: 'borrow, 'borrow> Signature for &BlockSettings<'env, 'borrow> {
        const SIG_TYPE: &'static str = "Lnet/minecraft/block/AbstractBlock$Settings;";
    }
    impl<'env: 'borrow, 'borrow> Signature for &mut BlockSettings<'env, 'borrow> {
        const SIG_TYPE: &'static str = "Lnet/minecraft/block/AbstractBlock$Settings;";
    }

    impl<'env: 'borrow, 'borrow> BlockSettings<'env, 'borrow> {
        pub extern "java" fn of(env: &'borrow JNIEnv<'env>, material: Material) -> JniResult<BlockSettings<'env, 'borrow>> {}
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(net.minecraft.block)]
    pub struct Block<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    impl<'env: 'borrow, 'borrow> Block<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>, settings: BlockSettings) -> JniResult<Self> {}
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(me.hydos.quiltlangrust.entrypoint)]
    pub struct RustModInitializer<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>
    }

    impl<'env: 'borrow, 'borrow> RustModInitializer<'env, 'borrow> {
        pub extern "jni" fn runNativeInitializer(self, env: &JNIEnv, libPath: String, modid: String) -> JniResult<()> {
            println!("This was printed in rust. if this works then i am concern. also hi java");

            println!("{} sounds like an interesting modid", modid);
            println!("{}", libPath);

            let logger = LogManager::getLogger(env, "Logger made in rust".to_string())?;
            let block_registry = Registry::getBlock(env)?;
            let block_registry = Registry::try_from( block_registry.raw.as_obj(), env)?;

            let block_id = Identifier::new(env, "minecraft:corbas".to_string())?;
            let block_material = Material::getPlant(env)?;
            let block_settings = BlockSettings::of(env, block_material)?;
            let block = Block::new(env, block_settings)?;

            let obj = block_registry.register(env, block_id, ())?;

            logger.info(env, "Hello, im printing this from rust!".to_string());

            Ok(())
        }
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