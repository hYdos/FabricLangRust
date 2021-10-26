use std::borrow::Borrow;
use std::ffi::CStr;

use robusta_jni::jni;

use jni::JNIEnv;
use jni::objects::{JObject, JString};

pub trait JStringUtils {
    fn into_str(self, env: &JNIEnv) -> String;
}

impl JStringUtils for JString<'_> {
    fn into_str(self, env: &JNIEnv) -> String {
        unsafe { CStr::from_ptr(env.get_string_utf_chars(self).unwrap()).to_string_lossy().into_owned() }
    }
}