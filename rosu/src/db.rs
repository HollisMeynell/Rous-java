use jni::JNIEnv;
use jni::objects::{GlobalRef, JByteArray, JObject, JString};
use jni::sys::jlong;
use osu_db::collection::Collection;
use osu_db::CollectionList;

use crate::{to_ptr, to_status, to_status_use};
use crate::java::Result;

const VERSION: u32 = 20220424;

pub struct JniCollectionList {
    data: CollectionList,
    callback: GlobalRef,
}

impl JniCollectionList {
    pub fn new(env: &mut JNIEnv, obj: JObject) -> Result<()> {
        let callback = env.new_global_ref(&obj)?;
        let data = CollectionList {
            version: VERSION,
            collections: vec![],
        };
        let result = Self { data, callback };
        init_jni_collection_list(env, result)?;
        Ok(())
    }
    pub fn from_bytes(env: &mut JNIEnv, bytes: JByteArray, obj: JObject) -> Result<()> {
        let collection_list_data = env.convert_byte_array(bytes)?;
        let data = CollectionList::from_bytes(&collection_list_data)?;
        let callback = env.new_global_ref(&obj)?;
        let result = JniCollectionList {
            data,
            callback,
        };
        init_jni_collection_list(env, result)?;
        Ok(())
    }

    pub fn release(ptr: i64) -> Result<Box<Self>> {
        to_status::<Self>(ptr)
    }
}

fn init_jni_collection_list(
    env: &mut JNIEnv,
    collection: JniCollectionList,
) -> Result<()> {
    let callback = collection.callback.clone();
    let version = collection.data.version as i32;
    let mut str = String::new();
    let str_build = &mut str;
    collection.data.collections.iter().for_each(|collection| {
        if let Some(name) = &collection.name {
            str_build.push_str(name);
        }
        str_build.push('#');
        collection.beatmap_hashes.iter().for_each(|hash| {
            if let Some(hash) = hash {
                str_build.push_str(hash);
            }
            str_build.push(',');
        });
        str_build.push('|');
    });
    let jni_str = env.new_string(str)?;

    let ptr = to_ptr(collection);
    env.call_method(&callback, "setCollections$rosu_java", "(Ljava/lang/String;)V", &[(&jni_str).into()])?;
    env.call_method(&callback, "setVersion$rosu_java", "(I)V", &[version.into()])?;
    env.call_method(&callback, "setPtr$rosu_java", "(J)V", &[ptr.into()])?;
    Ok(())
}

pub fn write_collection(ptr: i64) -> Result<Vec<u8>> {
    let collection_list = to_status_use::<JniCollectionList>(ptr)?;
    let mut result = vec![];
    collection_list.data.to_writer(&mut result)?;
    Ok(result)
}

pub fn add_collection(
    env: &mut JNIEnv,
    ptr: jlong,
    name: &JString,
    hashes: &JString,
) -> Result<()> {
    let name: String = env.get_string(name)?.into();
    let hashes: String = env.get_string(hashes)?.into();
    let mut beatmap_hashes = vec![];
    hashes
        .split(",")
        .filter(|x| x.len() > 0)
        .for_each(|x| (&mut beatmap_hashes).push(Some(x.to_string())));
    let collection = Collection { name: Some(name), beatmap_hashes };
    to_status_use::<JniCollectionList>(ptr)?
        .data
        .collections
        .push(collection);
    Ok(())
}

pub fn remove_collection(ptr: i64, index: i32) -> Result<()> {
    to_status_use::<JniCollectionList>(ptr)?
        .data
        .collections
        .remove(index as usize);
    Ok(())
}

pub fn clear_collection(ptr: i64, index: i32) -> Result<()> {
    to_status_use::<JniCollectionList>(ptr)?
        .data.collections[index as usize]
        .beatmap_hashes.clear();
    Ok(())
}

pub fn add_all_collection_hash(
    env: &mut JNIEnv,
    ptr: i64,
    index: i32,
    hashes: &JString,
) -> Result<()> {
    let jni = to_status_use::<JniCollectionList>(ptr)?;
    let hashes_vec = &mut jni.data.collections[index as usize].beatmap_hashes;
    let hashes: String = env.get_string(hashes)?.into();
    hashes
        .split(",")
        .filter(|x| x.len() > 0)
        .for_each(|x| hashes_vec.push(Some(x.to_string())));
    Ok(())
}

pub fn set_collection_name(
    env: &mut JNIEnv,
    ptr: i64,
    index: i32,
    name: &JString,
) -> Result<()> {
    let name: String = env.get_string(name)?.into();
    to_status_use::<JniCollectionList>(ptr)?
        .data
        .collections[index as usize]
        .name = Some(name);
    Ok(())
}

pub fn append_collection_hash(
    env: &mut JNIEnv,
    ptr: i64,
    index: i32,
    hash: &JString,
) -> Result<()> {
    let hash: String = env.get_string(hash)?.into();
    to_status_use::<JniCollectionList>(ptr)?
        .data
        .collections[index as usize]
        .beatmap_hashes
        .push(Some(hash));
    Ok(())
}

pub fn insert_collection_hash(
    env: &mut JNIEnv,
    ptr: i64,
    index: i32,
    hash_index: i32,
    hash: &JString,
) -> Result<()> {
    let hash: String = env.get_string(hash)?.into();
    to_status_use::<JniCollectionList>(ptr)?
        .data
        .collections[index as usize]
        .beatmap_hashes
        .insert(hash_index as usize, Some(hash));
    Ok(())
}

pub fn set_collection_hash(
    env: &mut JNIEnv,
    ptr: i64,
    index: i32,
    hash_index: i32,
    hash: &JString,
) -> Result<()> {
    let hash: String = env.get_string(hash)?.into();
    to_status_use::<JniCollectionList>(ptr)?
        .data
        .collections[index as usize]
        .beatmap_hashes[hash_index as usize] = Some(hash);
    Ok(())
}

pub fn remove_collection_hash(
    ptr: i64,
    index: i32,
    hash_index: i32,
) -> Result<()> {
    to_status_use::<JniCollectionList>(ptr)?
        .data
        .collections[index as usize]
        .beatmap_hashes
        .remove(hash_index as usize);
    Ok(())
}