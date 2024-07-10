use std::panic;
use crate::{error_to_bytes, StatusFlag, to_status};
use jni::objects::*;
use jni::JNIEnv;
use rosu_pp::{GradualPerformance};
use error_chain::error_chain;
use jni::sys::{jboolean, jlong};
use crate::db::{add_map, append_collection_list, create_collection, create_collection_list};
use crate::pp::{calculate, calculate_pp, get_calculate};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Jni(jni::errors::Error);
        JniOther(jni::errors::JniError);
        Db(osu_db::Error);
    }
    errors {
        LocalError(s: String)
    }
}
/// 计算 pp
/// - all: `[(mode)u8 | (pp)f64 | (star)f64 | (max combo)i32]`
/// - osu: `[(pp_{acc, aim, speed, fl})f64 * 4]`
/// - taiko: `[(pp_{acc, difficulty})f64 * 2]`
/// - taiko: `[(pp_difficulty)f64]`
#[no_mangle]
pub extern "system" fn Java_rosu_Native_calculate<'l>(
    env: JNIEnv<'l>,
    _class: JClass<'l>,
    local_map: JByteArray,
    score: JByteArray,
) -> JByteArray<'l> {
    match calculate(&env, &local_map, &score) {
        Ok(result) => env.byte_array_from_slice(&result).unwrap(),
        Err(e) => {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }
}

/// 渐进 pp 的计算器
///
/// 获取 [`GradualPerformance`] 的指针
///
/// ` [(mode)u8 | (mods)i32 | (ptr)f64] `
#[no_mangle]
pub extern "system" fn Java_rosu_Native_getCalculateIterator<'l>(
    env: JNIEnv<'l>,
    _class: JClass<'l>,
    local_map: JByteArray,
    attr: JByteArray,
) -> JByteArray<'l> {
    match get_calculate(&env, &local_map, &attr) {
        Ok(result) => env.byte_array_from_slice(&result).unwrap(),
        Err(e) => {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }
}

/// 渐进计算 pp
///
/// ptr: [`GradualPerformance`] 的指针
///
/// 返回值与 [`Java_rosu_Native_calculate`] 相同
#[no_mangle]
pub extern "system" fn Java_rosu_Native_calculateIterator<'l>(
    env: JNIEnv<'l>,
    _class: JClass<'l>,
    ptr: jlong,
    score: JByteArray,
) -> JByteArray<'l> {
    match calculate_pp(&env, ptr, &score) {
        Ok(result) => env.byte_array_from_slice(&result).unwrap(),
        Err(e) => {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }
}

/// 释放指针
///
/// `[(success)1 | (err)0]`
#[no_mangle]
pub extern "system" fn Java_rosu_Native_collectionCalculate<'l>(
    _env: JNIEnv<'l>,
    _class: JClass<'l>,
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

#[no_mangle]
pub extern "system" fn Java_rosu_Native_createCollection<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    name: JString,
) -> JByteArray<'l> {
    match create_collection(&mut env, &name) {
        Ok(mut result) => {
            result.insert(0, StatusFlag::None.bits());
            env.byte_array_from_slice(&result).unwrap()
        }
        Err(e) => {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_setCollectionMap<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    ptr: jlong,
    map: JString,
) -> jboolean {
    let r = panic::catch_unwind(move || {
        add_map(&mut env, ptr, &map).unwrap()
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

#[no_mangle]
    pub extern "system" fn Java_rosu_Native_newCollectionList<'l>(
    env: JNIEnv<'l>,
    _class: JClass<'l>,
    ptr: jlong,
) -> JByteArray<'l> {
    match create_collection_list(ptr) {
        Ok(mut result) => {
            result.insert(0, StatusFlag::None.bits());
            env.byte_array_from_slice(&result).unwrap()
        }
        Err(e) => {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }
}

#[no_mangle]
    pub extern "system" fn Java_rosu_Native_appendCollectionList<'l>(
    env: JNIEnv<'l>,
    _class: JClass<'l>,
    collection_list: JByteArray,
    ptr: jlong,
) -> JByteArray<'l> {
    match append_collection_list(&env, &collection_list, ptr) {
        Ok(mut result) => {
            result.insert(0, StatusFlag::None.bits());
            env.byte_array_from_slice(&result).unwrap()
        }
        Err(e) => {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        }
    }
}