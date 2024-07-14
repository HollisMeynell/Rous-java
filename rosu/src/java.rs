use crate::{error_to_bytes, StatusFlag, to_status};
use jni::objects::*;
use jni::JNIEnv;
use rosu_pp::{GradualPerformance};
use error_chain::error_chain;
use jni::sys::{jint, jlong};
use crate::db::*;
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
    calculate_pp(&env, ptr, &score)
        .map(|result| env.byte_array_from_slice(&result).unwrap())
        .unwrap_or_else(|e| {
            let result = error_to_bytes(e.description());
            env.byte_array_from_slice(&result).unwrap()
        })
}

/// 释放指针
///
/// `[(success)1 | (err)0]`
#[no_mangle]
pub extern "system" fn Java_rosu_Native_releaseCalculate<'l>(
    env: JNIEnv<'l>,
    _class: JClass<'l>,
    ptr: jlong,
) -> JByteArray<'l> {
    let result =
        to_status::<GradualPerformance>(ptr)
            .map(|_| vec![])
            .unwrap_or_else(|e| error_to_bytes(e.description()));
    env.byte_array_from_slice(&result).unwrap()
}

/**************************************************************************************************/

#[no_mangle]
pub extern "system" fn Java_rosu_Native_createCollection<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    collection: JObject,
) -> JByteArray<'l> {
    let result = JniCollectionList::new(&mut env, collection)
        .map(|_| vec![])
        .unwrap_or_else(|e| error_to_bytes(e.description()));
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_readCollection<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass,
    data: JByteArray,
    collection: JObject,
) -> JByteArray<'l> {
    let r = JniCollectionList::from_bytes(
        &mut env, data, collection,
    );
    let result = if let Err(e) = r {
        error_to_bytes(e.description())
    } else {
        vec![]
    };
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_writeCollection<'l>(
    env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
) -> JByteArray<'l> {
    let result = write_collection(ptr)
        .map(|mut v| {
            v.insert(0, StatusFlag::None.bits());
            v
        })
        .unwrap_or_else(|e| error_to_bytes(e.description()));

    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_releaseCollection<'l>(
    env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
) -> JByteArray<'l> {
    let result =
        JniCollectionList::release(ptr)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_addCollection<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    name: JString<'l>,
    hashes: JString<'l>,
) -> JByteArray<'l> {
    let result =
        add_collection(&mut env, ptr, &name, &hashes)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_removeCollection<'l>(
    env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    index: jint,
) -> JByteArray<'l> {
    let result =
        remove_collection(ptr, index)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_clearCollection<'l>(
    env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    index: jint,
) -> JByteArray<'l> {
    let result =
        clear_collection(ptr, index)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_addAllCollectionHash<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    index: jint,
    hashes: JString<'l>,
) -> JByteArray<'l> {
    let result =
        add_all_collection_hash(&mut env, ptr, index, &hashes)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_setCollectionName<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    index: jint,
    name: JString<'l>,
) -> JByteArray<'l> {
    let result =
        set_collection_name(&mut env, ptr, index, &name)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_appendCollectionHash<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    index: jint,
    hash: JString<'l>,
) -> JByteArray<'l> {
    let result =
        append_collection_hash(&mut env, ptr, index, &hash)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_insertCollectionHash<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    index: jint,
    hash_index: jint,
    hash: JString<'l>,
) -> JByteArray<'l> {
    let result =
        insert_collection_hash(&mut env, ptr, index, hash_index, &hash)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_setCollectionHash<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    index: jint,
    hash_index: jint,
    hash: JString<'l>,
) -> JByteArray<'l> {
    let result =
        set_collection_hash(&mut env, ptr, index, hash_index, &hash)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_rosu_Native_removeCollectionHash<'l>(
    env: JNIEnv<'l>,
    _class: JClass,
    ptr: jlong,
    index: jint,
    hash_index: jint,
) -> JByteArray<'l> {
    let result =
        remove_collection_hash(ptr, index, hash_index)
            .map(|_| vec![])
            .unwrap_or_else(|e| {
                error_to_bytes(e.description())
            });
    env.byte_array_from_slice(&result).unwrap()
}
