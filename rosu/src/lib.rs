use std::ops::Not;
use bytes::BufMut;

use java::Result;

mod db;
pub mod java;
pub mod macros;
mod pp;
bitflags::bitflags! {
    struct StatusFlag :u8 {
        const Error = 0b10000000u8;
        const None = 0u8;
        const Osu = 0b00000001u8;
        const Taiko = 0b00000010u8;
        const Catch = 0b00000100u8;
        const Mania = 0b00001000u8;
    }
}

fn error_to_bytes(str: &str) -> Vec<u8> {
    let mut result = Vec::new();
    result.put_u8(StatusFlag::Error.bits());
    vec_add_str(str, &mut result);
    result
}

fn vec_add_str(str: &str, vec: &mut dyn BufMut) {
    let bytes = str.as_bytes();
    vec.put_i32(bytes.len() as i32);
    for b in bytes {
        vec.put_u8(*b);
    }
}

#[inline]
pub(crate) fn to_ptr<T>(s: T) -> i64 {
    Box::into_raw(Box::new(s)) as i64
}

#[inline]
fn check_ptr<T>(point: *mut T) -> Result<()>{
    if point.is_null() || point.is_aligned().not() {
        return Err("point is null or not".into());
    }
    Ok(())
}

#[inline]
pub fn to_status_use<T>(p: i64) -> Result<&'static mut T> {
    let point = p as *mut T;
    check_ptr(point)?;
    unsafe {
        point.as_mut()
            .ok_or_else(|| format!("read pointer error: ({})", p).into())
    }
}

#[inline]
fn to_status<T>(p: i64) -> Result<Box<T>> {
    let point = p as *mut T;
    check_ptr(point)?;
    unsafe {
        if let None = point.as_ref() {
            Err(format!("read pointer error: ({})", p).into())
        } else {
            Ok(Box::from_raw(point))
        }
    }
}
