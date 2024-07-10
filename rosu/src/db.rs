use bytes::BufMut;
use jni::JNIEnv;
use jni::objects::{JByteArray, JString};
use osu_db::collection::Collection;
use osu_db::CollectionList;

use crate::java::Result;
use crate::{to_ptr, to_status, to_status_use};

const VERSION: u32 = 20220424;

pub fn create_collection(env: &mut JNIEnv, name: &JString) -> Result<Vec<u8>> {
    let name: String = env.get_string(name)?.into();

    let collection = Collection {
        name: Some(name),
        beatmap_hashes: vec![],
    };
    let ptr = to_ptr(collection);
    let mut out = Vec::<u8>::new();
    out.put_i64(ptr);
    Ok(out)
}

pub fn add_map(env: &mut JNIEnv, ptr: i64, md5: &JString) -> Result<()> {
    let md5: String = env.get_string(md5)?.into();
    let collection = to_status_use::<Collection>(ptr);
    collection.beatmap_hashes.push(Some(md5));
    Ok(())
}

pub fn create_collection_list(ptr: i64) -> Result<Vec<u8>> {
    let collection = to_status::<Collection>(ptr);
    let collection_list = CollectionList {
        version: VERSION,
        collections: vec![*collection],
    };
    let mut result = vec![];
    collection_list.to_writer(&mut result)?;
    Ok(result)
}


pub fn append_collection_list(env: &JNIEnv, bytes: &JByteArray, ptr: i64) -> Result<Vec<u8>> {
    let collection_list_data = env.convert_byte_array(bytes)?;
    let mut collection_list = CollectionList::from_bytes(&collection_list_data)?;

    let collection = to_status::<Collection>(ptr);
    collection_list.collections.push(*collection);
    let mut out: Vec<u8> = Vec::new();
    let _ = collection_list.to_writer(&mut out);
    Ok(out)
}
