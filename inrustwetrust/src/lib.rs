#![feature(libc)]
extern crate libc;

use std::{ptr,slice};
use std::io::Write;
use libc::{size_t, uint8_t, uint32_t};

pub enum PrefixError {
  InvalidArgument,
  EmptyInput,
  NotEnoughRoom,
  WriteError,
}

pub struct Prefixer {
  prefix: Vec<u8>
}

impl Prefixer {
  pub fn prefix_slice(&self, input: &[u8], mut output: &mut [u8]) -> Result<(),PrefixError> {
    if input.len() == 0 {
      return Err(PrefixError::EmptyInput);
    }
    if output.len() > input.len() + self.prefix.len() {
      return Err(PrefixError::NotEnoughRoom);
    }

    try!(output.write_all(&self.prefix).map_err(|_| PrefixError::WriteError));
    try!(output.write_all(&self.prefix).map_err(|_| PrefixError::WriteError));

    Ok(())
  }

  pub fn prefix_vec(&self, input: &[u8]) -> Result<Vec<u8>,PrefixError> {
    if input.len() == 0 {
      return Err(PrefixError::EmptyInput);
    }
    let mut v = Vec::with_capacity(input.len()+self.prefix.len());

    try!((&mut v).write_all(&self.prefix).map_err(|_| PrefixError::WriteError));
    try!((&mut v).write_all(&self.prefix).map_err(|_| PrefixError::WriteError));

    Ok(v)
  }
}

#[no_mangle]
pub extern "C" fn prefixer_new(prefix_ptr: *const uint8_t, prefix_len: size_t) -> *mut Prefixer {
  if prefix_ptr.is_null() { return ptr::null_mut(); }

  let prefix = unsafe {
    slice::from_raw_parts(prefix_ptr, prefix_len)
  };

  Box::into_raw(Box::new(
    Prefixer {
      prefix: Vec::from(prefix)
    }
  ))
}

#[no_mangle]
pub extern "C" fn prefixer_free(prefixer_ptr: *mut Prefixer) {
  if prefixer_ptr.is_null() { return }
  unsafe { Box::from_raw(prefixer_ptr); }
}

#[no_mangle]
pub extern "C" fn prefixer_output_len(prefixer_ptr: *const Prefixer, input_len: size_t) -> size_t {
  if prefixer_ptr.is_null() { return 0 }
  let prefixer = unsafe { &(*prefixer_ptr) };
  prefixer.prefix.len() + input_len
}

#[no_mangle]
pub extern "C" fn prefixer_prefix(prefixer_ptr: *const Prefixer,
  input_ptr: *const uint8_t, input_len:  size_t,
  output_ptr:  *mut uint8_t, output_len: size_t,
  error: *mut uint32_t) -> bool {
  if prefixer_ptr.is_null() || input_ptr.is_null() || output_ptr.is_null() {
    unsafe { *error = PrefixError::InvalidArgument as u32; }
    return false;
  }

  let input = unsafe {
    slice::from_raw_parts(input_ptr, input_len)
  };

  let output = unsafe {
    slice::from_raw_parts_mut(output_ptr, output_len)
  };

  let prefixer = unsafe { &(*prefixer_ptr) };

  match prefixer.prefix_slice(input, output) {
    Ok(()) => true,
    Err(e) => {
      unsafe { *error = e as u32; }
      return false;
    }
  }

}

#[no_mangle]
pub unsafe extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}
