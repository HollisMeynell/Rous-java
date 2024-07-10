use bytes::BufMut;

pub mod java;
pub mod macros;
mod pp;
mod db;

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
    for b in bytes { vec.put_u8(*b); }
}

#[inline]
pub(crate) fn to_ptr<T>(s: T) -> i64 {
    Box::into_raw(Box::new(s)) as i64
}
#[inline]
pub fn to_status_use<'l, T>(p: i64) -> &'l mut T {
    unsafe { &mut *(p as *mut T) }
}
#[inline]
fn to_status<T>(p: i64) -> Box<T> {
    let point = p as *mut T;
    unsafe { Box::from_raw(point) }
}

#[test]
fn test_byte_to_jni_score() {
    let f = std::fs::read("F:\\bot\\attr").unwrap();
    let s = crate::pp::JniScore::from(f.as_slice());
    println!("s{}", s.attr.speed)
}

#[test]
fn box_use() {
    let mut t = Vec::new();
    t.push(1u8);
    t.push(6u8);
    t.push(3u8);
    t.push(12u8);

    let p = Box::new(t);
    let p = Box::into_raw(p);

    let t = to_status_use::<Vec<u8>>(p as i64);
    // let mut t = unsafe { &mut *p };
    t.push(0u8);
    t.push(1u8);
    println!("{:?}", t);

    let x = unsafe { Box::from_raw(p) };
    println!("{:?}", x.as_slice());
}