use std::panic;
use crate::{attr_to_bytes, calculate_to_bytes, error_to_bytes, JniMapAttr, JniScore};
use jni::objects::*;
use jni::JNIEnv;
use rosu_pp::{Beatmap, Difficulty, GradualPerformance, Performance};
use error_chain::error_chain;
use jni::sys::{jboolean, jlong};

error_chain!{
    foreign_links {
        Io(std::io::Error);
        Jni(jni::errors::Error);
        JniOther(jni::errors::JniError);
    }
    errors {
        LocalError(s: String)
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
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }

}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_getCalculateIterator<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    local_map: JByteArray,
    attr: JByteArray,
) -> JByteArray<'local> {
    match get_calculate(&env, &local_map, &attr) {
        Ok(result) => env.byte_array_from_slice(&result).unwrap(),
        Err(e) => {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }

}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_calculateIterator<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    score: JByteArray,
) -> JByteArray<'local> {
    match calculate_pp(&env, ptr, &score) {
        Ok(result) => env.byte_array_from_slice(&result).unwrap(),
        Err(e) => {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_collectionCalculate<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jboolean {
    let r = panic::catch_unwind(|| {
        to_status::<GradualPerformance>(ptr)
    });
    match r {
        Ok(_) => {
            1u8
        }
        Err(_) => {
            0u8
        }
    }
}

/// 计算 pp, 如果没有成绩就是 map 的fc成绩
fn calculate (
    env: &JNIEnv,
    local_map: &JByteArray,
    score: &JByteArray,
) -> Result<Vec<u8>> {
    let (map, score) = get_map_and_score(env, local_map, score)?;

    let difficulty = Difficulty::new()
        .mods(score.attr.mods);
    let attributes = if score.attr.speed > 0.0 {
        difficulty.clock_rate(score.attr.speed).calculate(&map)
    } else {
        difficulty.calculate(&map)
    };

    let max_combo = attributes.max_combo();
    let performance = score.performance(max_combo, Performance::new(attributes));
    let mut result = Vec::<u8>::new();
    attr_to_bytes(&performance.calculate(), &mut result);
    Ok(result)
}

/// 渐进式计算成绩 获得计算器
fn get_calculate(
    env: &JNIEnv,
    local_map: &JByteArray,
    attr: &JByteArray,
) -> Result<Vec<u8>> {
    let (map, attr) = get_map_and_attr(env, local_map, attr)?;
    let mode = map.mode;
    let mods = attr.mods;
    let difficulty = Difficulty::new()
        .mods(mods);
    let gradual = if attr.speed > 0.0 {
        difficulty.clock_rate(attr.speed).gradual_performance(&map)
    } else {
        difficulty.gradual_performance(&map)
    };

    let ptr = to_ptr(gradual);
    let mut result = Vec::<u8>::new();
    calculate_to_bytes(ptr, mode, mods, &mut result);
    Ok(result)
}

fn calculate_pp(
    env: &JNIEnv,
    ptr: i64,
    score: &JByteArray
) -> Result<Vec<u8>> {
    let gradual = to_status_use::<GradualPerformance>(ptr);
    let score = get_score(env, score)?;
    if score.score.is_none() {
        return Err(Error::from("no score"));
    }
    let state = score.score.as_ref().unwrap();
    let attr = gradual.next(state.clone());
    if attr.is_none() {
        return Err(Error::from("gradual error"));
    }
    let attr = attr.unwrap();

    let mut result = Vec::<u8>::new();
    attr_to_bytes(&attr, &mut result);

    Ok(result)
}

fn get_map_and_score(
    env: &JNIEnv,
    local_map: &JByteArray,
    score: &JByteArray,
) -> Result<(Beatmap, JniScore)> {
    let mut map = get_map(env, local_map)?;
    let score = get_score(env, score)?;

    if let Some(m) = score.attr.mode {
        map.mode = m;
    }

    Ok((map, score))
}

fn get_map_and_attr(
    env: &JNIEnv,
    local_map: &JByteArray,
    attr: &JByteArray,
) -> Result<(Beatmap, JniMapAttr)> {
    let mut map = get_map(env, local_map)?;
    let attr = get_map_attr(env, attr)?;

    if let Some(m) = attr.mode {
        map.mode = m;
    }

    Ok((map, attr))
}

fn get_map(env: &JNIEnv, local_map: &JByteArray,) -> Result<Beatmap> {
    let map_bytes = env.convert_byte_array(local_map)?;
    let map = Beatmap::from_bytes(&map_bytes)?;
    Ok(map)
}

fn get_map_attr(env: &JNIEnv, attr: &JByteArray,) -> Result<JniMapAttr> {
    let attr_bytes = env.convert_byte_array(attr)?;
    let attr = JniMapAttr::from(attr_bytes.as_slice());
    Ok(attr)
}

fn get_score(env: &JNIEnv, score: &JByteArray,) -> Result<JniScore> {
    let score_bytes = env.convert_byte_array(score)?;
    let score = JniScore::from(score_bytes.as_slice());
    Ok(score)
}

/*
fn get_score_status(env: &JNIEnv, state: &JByteArray) ->Result<rosu_pp::any::ScoreState>{
    let state_bytes = env.convert_byte_array(state)?;
    let stats = bytes_to_score_state(bytes::Bytes::from(state_bytes));
    Ok(stats)
}

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
*/
#[inline]
fn to_ptr<T>(s: T) -> i64 {
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