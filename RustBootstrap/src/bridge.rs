use std::borrow::Borrow;

use robusta_jni::jni;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::strings::JNIString;

pub struct Class<'a> {
    j_class: JClass<'a>,
}

pub struct Object<'a> {
    j_object: JObject<'a>,
}

pub struct Value<'a> {
    value: JValue<'a>,
}

impl Class<'a> {
    pub fn method(&self, env: &JNIEnv<'a>, name: &str, signature: &str, args: &[JValue]) -> Value<'a> {
        let raw_result = env.call_static_method(self.j_class, name, signature, args);
        env.exception_describe();
        Value {
            value: raw_result.expect("Couldn't unwrap return value."),
        }
    }

    pub fn new(&self, env: &JNIEnv<'a>, signature: &str, args: &[JValue]) -> Value<'a> {
        let raw_result = env.new_object(self.j_class, signature, args);
        env.exception_describe();
        Value {
            value: JValue::Object(raw_result.expect("Couldn't unwrap return value.")),
        }
    }

    pub fn field(&self, env: &JNIEnv<'a>, name: &str, signature: &str) -> Value<'a> {
        let raw_result = env.get_static_field(self.j_class, name, signature);
        env.exception_describe();
        Value {
            value: raw_result.expect("Couldn't unwrap return value."),
        }
    }
}

impl Object<'a> {
    pub fn method(&self, env: &JNIEnv<'a>, name: &str, signature: &str, args: &[JValue]) -> Value<'a> {
        let raw_result = env.call_method(self.j_object, name, signature, args);
        env.exception_describe();
        Value {
            value: raw_result.expect("Couldn't unwrap return value."),
        }
    }

    pub fn get_raw(&self) -> JObject {
        self.j_object
    }
}

impl Value<'a> {
    pub fn as_object(&self) -> Object {
        let raw_object = self.value.l().expect("Failed to unwrap to JObject.");
        Object {
            j_object: raw_object
        }
    }
}

pub fn jstring<'a>(env: &'a JNIEnv, string: &'a str) -> JString<'a> {
    env.new_string(string).unwrap()
}

pub fn class(env: &JNIEnv<'a>, class: &str) -> Class<'a> {
    let clean_string = class.replace(".", "/");
    Class {
        j_class: env.find_class(clean_string.as_str()).unwrap(),
    }
}