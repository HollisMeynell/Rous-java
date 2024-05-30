use crate::{JniScore, StatusFlag};
use bytes::BufMut;
use jni::objects::*;
use jni::JNIEnv;
use rosu_pp::any::{PerformanceAttributes};
use rosu_pp::{Beatmap, Difficulty, Performance};
use error_chain::error_chain;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        Jni(jni::errors::Error);
        JniOther(jni::errors::JniError);
    }
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_calculate<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    local_map: JByteArray,
    score: JByteArray,
) -> JByteArray<'local> {
    match calculate(&env, &local_map, &score) {
        Ok(result) => env.byte_array_from_slice(&result).unwrap(),
        Err(e) => {
            let mut  result = Vec::new();
            result.put_u8(StatusFlag::Error.bits());
            add_str(&mut result, e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }

}

fn calculate (
    env: &JNIEnv,
    local_map: &JByteArray,
    score: &JByteArray,
) -> Result<Vec<u8>> {
    let map_bytes = env.convert_byte_array(local_map)?;
    let mut map = Beatmap::from_bytes(&map_bytes)?;

    let score_bytes = env.convert_byte_array(score)?;
    let score = JniScore::from(score_bytes.as_slice());

    if let Some(m) = score.mode{
        map.mode = m;
    }

    let difficulty = Difficulty::new()
        .mods(score.mods);
    let attributes = if score.speed > 0.0 {
        difficulty.clock_rate(score.speed).calculate(&map)
    } else {
        difficulty.calculate(&map)
    };


    let max_combo = attributes.max_combo();
    let performance = score.performance(max_combo, Performance::new(attributes));
    let mut result = Vec::<u8>::new();
    match performance.calculate() {
        PerformanceAttributes::Osu(data) => {
            result.put_u8(StatusFlag::Osu.bits());
            result.put_f64(data.pp());
            result.put_f64(data.stars());
            result.put_i32(data.max_combo() as i32);

            result.put_f64(data.pp_acc);
            result.put_f64(data.pp_aim);
            result.put_f64(data.pp_speed);
            result.put_f64(data.pp_flashlight);
        }
        PerformanceAttributes::Taiko(data) => {
            result.put_u8(StatusFlag::Taiko.bits());
            result.put_f64(data.pp());
            result.put_f64(data.stars());
            result.put_i32(data.max_combo() as i32);

            result.put_f64(data.pp_acc);
            result.put_f64(data.pp_difficulty);
        }
        PerformanceAttributes::Catch(data) => {
            result.put_u8(StatusFlag::Catch.bits());
            result.put_f64(data.pp());
            result.put_f64(data.stars());
            result.put_i32(data.max_combo() as i32);
        }
        PerformanceAttributes::Mania(data) => {
            result.put_u8(StatusFlag::Mania.bits());
            result.put_f64(data.pp());
            result.put_f64(data.stars());
            result.put_i32(data.max_combo() as i32);

            result.put_f64(data.pp_difficulty);
        }
    }
    Ok(result)
}



/*
#[no_mangle]
pub extern "system" fn Java_rosu_Native_test<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    path: JString<'local>,
    source: JByteBuffer<'local>,
) -> JByteArray<'local> {
    let s = env.get_string(&path).expect("err");

    let s = format!("fuck you, {}", s.to_str().unwrap());
    let d = (&s).as_bytes();

    let js = get_buffer(&env, &source).unwrap();

    let dl = d.len();
    let mut buffer: Vec<u8> = Vec::with_capacity(size_of_val(&dl) + dl);
    buffer.put_i32(dl as i32);
    buffer.put(d);

    js[..buffer.len()].copy_from_slice(&buffer);

    env.byte_array_from_slice(d).unwrap()
}

fn get_buffer<'t>(env: &'t JNIEnv, buff: &'t JByteBuffer) -> jni::errors::Result<&'t mut [u8]> {
    let jsp = env.get_direct_buffer_address(&buff)?;
    let jsl = env.get_direct_buffer_capacity(&buff)?;
    unsafe { Ok(std::slice::from_raw_parts_mut(jsp, jsl)) }
}

fn get_str<'str>(env: &mut JNIEnv, string: &'str JString) -> jni::errors::Result<&'str str> {
    let s = env.get_string(string)?.to_str().expect("to str err");
    Ok(s)
}

fn get_string(env: &mut JNIEnv, string: &JString) -> jni::errors::Result<String> {
    let s: String = env.get_string(string)?.into();
    Ok(s)
}

#[inline]
fn to_bytes<'local>(env: &'local JNIEnv, data: &[u8]) -> jni::errors::Result<JByteArray<'local>> {
    env.byte_array_from_slice(data)
}

#[inline]
fn to_ptr<T>(s: T) -> jlong {
    Box::into_raw(Box::new(s)) as jlong
}
#[inline]
fn to_status_use<T>(p: jlong) -> *mut T {
    p as *mut T
}
#[inline]
fn to_status<T>(p: jlong) -> Box<T> {
    let point = p as *mut T;
    unsafe { Box::from_raw(point) }
}
*/
#[inline]
fn add_str(vec:&mut Vec<u8>, str:&str) {
    let bytes = str.as_bytes();
    vec.put_i32(bytes.len() as i32);
    vec.put(bytes);
}