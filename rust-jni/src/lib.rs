extern crate libc;
extern crate jni_sys;
extern crate inrustwetrust;

use libc::{c_void, c_int};
use jni_sys::JNIEnv;

#[allow(unused_variables,non_snake_case)]
#[no_mangle]
pub extern fn Java_com_geal_InRustWeTrust_add(jre: *mut JNIEnv, class: *const c_void, v1: c_int, v2: c_int) -> c_int {
    // yes, Java only has signed integers, ugly cast here
    unsafe  { inrustwetrust::add(v1 as u32, v2 as u32) as i32 }
}

