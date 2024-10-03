use crate::db::*;
use crate::pp::{calculate, calculate_pp, get_calculate};
use crate::{error_to_bytes, to_status};
use error_chain::error_chain;
use jni::objects::*;
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use rosu_pp::GradualPerformance;

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

macro_rules! jni_fn {
    ( $name:ident ( $env:ident; $($param:ident: $param_type:ty),* ) { $($codeLine:stmt)* } ) => {
        paste::paste!{
            #[no_mangle]
            pub extern "system" fn [<Java_rosu_Native_$name>]<'l> (
                $env: JNIEnv<'l>,
                _class: JClass<'l>,
                $( $param: $param_type),*
            ) -> JByteArray<'l>  { (||{ $($codeLine)* })() }
        }
    };
    ( $name:ident (mut $env:ident; $($param:ident: $param_type:ty),* ) { $($codeLine:stmt)* } ) => {
        paste::paste!{
            #[no_mangle]
            pub extern "system" fn [<Java_rosu_Native_$name>]<'l> (
                mut $env: JNIEnv<'l>,
                _class: JClass<'l>,
                $( $param: $param_type),*
            ) -> JByteArray<'l>  { (||{ $($codeLine)* })() }
        }
    };
}

macro_rules! jni_result {
    ($env:ident,(u) $result:ident) => {{
        let result = $result
            .map(|_| vec![])
            .unwrap_or_else(|e| error_to_bytes(e.description()));

        $env.byte_array_from_slice(&result).unwrap()
    }};
    ($env:ident, $result:ident) => {{
        $result
            .map(|result| $env.byte_array_from_slice(&result).unwrap())
            .unwrap_or_else(|e| {
                let result = error_to_bytes(e.description());
                $env.byte_array_from_slice(&result).unwrap()
            })
    }};
}

jni_fn! {
    calculate(env; local_map:JByteArray, score:JByteArray) {
        let result = calculate(&env, &local_map, &score)
        jni_result!(env, result)
    }
}

jni_fn! {
    getCalculateIterator(env; local_map:JByteArray, attr:JByteArray) {
        let result =  get_calculate(&env, &local_map, &attr)
        jni_result!(env, result)
    }
}
jni_fn! {
    calculateIterator(env; ptr:jlong, score:JByteArray) {
        let result = calculate_pp(&env, ptr, &score)
        jni_result!(env, result)
    }
}

jni_fn! {
    releaseCalculate(env; ptr:jlong) {
        let result = to_status::<GradualPerformance>(ptr)
        jni_result!(env, (u) result)
    }
}

/**************************************************************************************************/
jni_fn! {
    createCollection(mut env; collection: JObject) {
        let result= JniCollectionList::new(&mut env, collection)
        jni_result!(env, (u) result)
    }
}

jni_fn! {
    readCollection(mut env; data: JByteArray,collection: JObject) {
        let result = JniCollectionList::from_bytes(&mut env, data, collection)
        jni_result!(env, (u) result)
    }
}

jni_fn! {
    writeCollection(env; ptr: jlong) {
        let result = write_collection(ptr)
        jni_result!(env, result)
    }
}

jni_fn! {
    releaseCollection(env; ptr: jlong) {
        let result = JniCollectionList::release(ptr)
        jni_result!(env, (u) result)
    }
}

jni_fn! {
    addCollection(mut env;ptr: jlong, name: JString<'l>, hashes: JString<'l>) {
        let result = add_collection(&mut env, ptr, &name, &hashes)
        jni_result!(env, (u) result)
    }
}

jni_fn! {
    removeCollection(env; ptr:jlong,index:jint) {
        let result = remove_collection(ptr, index)
        jni_result!(env, (u) result)
    }
}

jni_fn! {
    clearCollection(env; ptr: jlong, index: jint) {
        let result = clear_collection(ptr, index)
        jni_result!(env, (u) result)
    }
}
jni_fn! {
    addAllCollectionHash(mut env; ptr: jlong, index: jint, hashes: JString<'l>) {
        let result = add_all_collection_hash(&mut env, ptr, index, &hashes)
        jni_result!(env, (u) result)
    }
}
jni_fn! {
    setCollectionName(mut env; ptr: jlong, index: jint, name: JString<'l>) {
        let result = set_collection_name(&mut env, ptr, index, &name)
        jni_result!(env, (u) result)
    }
}
jni_fn! {
    appendCollectionHash(mut env; ptr: jlong, index: jint, hash: JString<'l>) {
        let result = append_collection_hash(&mut env, ptr, index, &hash)
        jni_result!(env, (u) result)
    }
}
jni_fn! {
    insertCollectionHash(mut env; ptr: jlong, index: jint, hash_index: jint, hash: JString<'l>) {
        let result = insert_collection_hash(&mut env, ptr, index, hash_index, &hash)
        jni_result!(env, (u) result)
    }
}
jni_fn! {
    setCollectionHash(mut env; ptr: jlong, index: jint, hash_index: jint, hash: JString<'l>) {
        let result = set_collection_hash(&mut env, ptr, index, hash_index, &hash)
        jni_result!(env, (u) result)
    }
}
jni_fn! {
    removeCollectionHash(env; ptr: jlong, index: jint, hash_index: jint) {
        let result = remove_collection_hash(ptr, index, hash_index)
        jni_result!(env, (u) result)
    }
}
