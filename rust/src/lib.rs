#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::borrow::Borrow;
use std::convert::Infallible;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fmt::Error;
use std::fs::File;
use std::io::{BufWriter, Cursor};
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;
use std::str::from_utf8;
use std::string::FromUtf8Error;

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jstring;
use once_cell::sync::OnceCell;
use sled::Db;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

static SLED_DB: OnceCell<Db> = OnceCell::new();

fn setup_sled_db(path: &str) -> Result<()> {
    if SLED_DB.get().is_some() {
        return Ok(());
    }
    let db = sled::open(path).unwrap();
    SLED_DB.set(db);
    Ok(())
}

fn get_sled_db() -> Option<&'static Db> {
    SLED_DB.get()
}

#[no_mangle]
pub unsafe extern fn Java_com_example_sledissue_MainActivity_setupSledStore(env: JNIEnv, _: JObject, j_path: JString) -> jstring {
    let recipient = CString::from(
        CStr::from_ptr(
            env.get_string(j_path).unwrap().as_ptr()
        )
    );
    setup_sled_db(recipient.to_str().unwrap());
    let output = env.new_string("true".to_owned()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_example_sledissue_MainActivity_updateSledStore(env: JNIEnv, _: JObject) -> jstring {
    let db = get_sled_db().unwrap();
    db.insert("KEY1", "VAL1");

    let output = env.new_string("true".to_owned()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_example_sledissue_MainActivity_updateSledStoreWithFlush(env: JNIEnv, _: JObject) -> jstring {
    let db = get_sled_db().unwrap();
    db.insert("KEY1", "VAL1");
    db.flush();

    let output = env.new_string("true".to_owned()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_example_sledissue_MainActivity_querySledStore(env: JNIEnv, _: JObject) -> jstring {
    let db = get_sled_db().unwrap();
    let res = db.get(&"KEY1").unwrap();
    let bytes = match &res {
        Some(v) => v.deref(),
        None => b"none"
    };

    let to_return = from_utf8(bytes).unwrap();

    let output = env.new_string(to_return.to_owned()).unwrap();
    output.into_inner()
}
