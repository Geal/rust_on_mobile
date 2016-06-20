extern crate libc;
extern crate inrustwetrust;

use libc::{c_void, c_int};

#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,
    // much more actually in here for practical JNI code, but not
    // relevant for this very simple example...
}

pub type JNIEnv = *const JNINativeInterface;

#[allow(unused_variables,non_snake_case)]
#[no_mangle]
pub extern fn Java_com_geal_InRustWeTrust_add(jre: *mut JNIEnv, class: *const c_void, v1: c_int, v2: c_int) -> c_int {
    // yes, Java only has signed integers, ugly cast here
    unsafe  { inrustwetrust::add(v1 as u32, v2 as u32) as i32 }
}

