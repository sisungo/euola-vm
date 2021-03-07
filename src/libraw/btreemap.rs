//!
//! This module provides `BTreeMap` data structure in `libraw`.
//!

use crate::{
    context::putnfp,
    libraw::iohmgr::{FakeHasher, IdGen},
    vmem::Var,
};
use anyhow::anyhow;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use std::{collections::BTreeMap, convert::TryFrom};

/// Id generator for hashmaps.
static IDGEN: Lazy<Mutex<IdGen>> = Lazy::new(|| Mutex::new(IdGen::default()));
/// Table of hashmaps.
#[allow(clippy::type_complexity)]
static TABLE: Lazy<DashMap<u64, RwLock<BTreeMap<Key, Var>>, FakeHasher>> =
    Lazy::new(DashMap::default);

/// A subset of a `Var` that can be hashed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    Signed(i64),
    Unsigned(u64),
    UString(Box<str>),
    Bytes(Box<[u8]>),
}
impl TryFrom<Var> for Key {
    type Error = anyhow::Error;

    fn try_from(a: Var) -> Result<Key, anyhow::Error> {
        Ok(match a {
            Var::I8(x) => Key::Signed(x as i64),
            Var::U8(x) => Key::Unsigned(x as u64),
            Var::I16(x) => Key::Signed(x as i64),
            Var::U16(x) => Key::Unsigned(x as u64),
            Var::I32(x) => Key::Signed(x as i64),
            Var::U32(x) => Key::Unsigned(x as u64),
            Var::I64(x) => Key::Signed(x),
            Var::U64(x) => Key::Unsigned(x),
            Var::Usize(x) => Key::Unsigned(x as u64),
            Var::UString(x) => Key::UString(Box::from(&x.borrow()?[..])),
            Var::Bytes(x) => Key::Bytes(x.borrow()?.to_vec().into_boxed_slice()),
            Var::Vector(_) => return Err(anyhow!("raw::fatal::invalid")),
            Var::Object(_) => return Err(anyhow!("raw::fatal::invalid")),
        })
    }
}
impl From<Key> for Var {
    fn from(a: Key) -> Var {
        use crate::vmem::{BytesRef, StringRef};

        match a {
            Key::Signed(x) => Var::I64(x),
            Key::Unsigned(x) => Var::U64(x),
            Key::UString(x) => Var::UString(StringRef::from(&*x)),
            Key::Bytes(x) => Var::Bytes(BytesRef::from(Vec::from(x))),
        }
    }
}

/// Make a new one.
pub fn new(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = IDGEN.lock().next();
    TABLE.insert(id, RwLock::new(BTreeMap::new()));
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(id);
    Ok(())
}
/// Drop a map.
pub fn drop(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    if TABLE.remove(&id).is_some() {
        IDGEN.lock().free(id);
    }
    Ok(())
}

/// Get a value from a map.
pub fn get(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let key = match Key::try_from(unsafe { a.get_unchecked(1) }.to_owned()) {
        Ok(x) => x,
        Err(_) => return Err(anyhow!("raw::fatal::invalid")),
    };
    let map = match TABLE.get(&id) {
        Some(x) => x,
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
            return Ok(());
        }
    };
    match map.read().get(&key) {
        Some(x) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
            *(unsafe { a.get_unchecked_mut(1) }) = x.to_owned();
        }
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
        }
    }
    Ok(())
}

/// Get a vector that contains all keys of a hashmap.
pub fn keys(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::vmem::VectorRef;
    use rayon::prelude::*;

    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let map = match TABLE.get(&id) {
        Some(x) => x,
        None => {
            use crate::vmem::CreateNull;

            *(unsafe { a.get_unchecked_mut(0) }) = Var::Vector(VectorRef::null());
            return Ok(());
        }
    };
    let rslt = Mutex::new(Vec::new());
    map.read()
        .par_iter()
        .for_each(|(x, _)| rslt.lock().push(x.to_owned().into()));
    *(unsafe { a.get_unchecked_mut(0) }) = Var::Vector(rslt.into_inner().into());
    Ok(())
}

/// Set a value to a map.
pub fn set(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let key = match Key::try_from(unsafe { a.get_unchecked(1) }.to_owned()) {
        Ok(x) => x,
        Err(_) => return Err(anyhow!("raw::fatal::invalid")),
    };
    let val = unsafe { a.get_unchecked(2) }.to_owned();
    let map = match TABLE.get(&id) {
        Some(x) => x,
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
            return Ok(());
        }
    };
    map.write().insert(key, val);
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
    Ok(())
}

/// Remove a value from a map.
pub fn remove(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let key = match Key::try_from(unsafe { a.get_unchecked(1) }.to_owned()) {
        Ok(x) => x,
        Err(_) => return Err(anyhow!("raw::fatal::invalid")),
    };
    let map = match TABLE.get(&id) {
        Some(x) => x,
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
            return Ok(());
        }
    };
    match map.write().remove(&key) {
        Some(_) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
        }
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
        }
    }
    Ok(())
}

/// Clear a map.
pub fn clear(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let map = match TABLE.get(&id) {
        Some(x) => x,
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
            return Ok(());
        }
    };
    map.write().clear();
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
    Ok(())
}

/// Initialize the library.
#[inline(always)]
pub fn init() {
    IDGEN.lock().next();
    putnfp("raw::btreemap::new", new);
    putnfp("raw::btreemap::remove", remove);
    putnfp("raw::btreemap::get", get);
    putnfp("raw::btreemap::set", set);
    putnfp("raw::btreemap::drop", drop);
    putnfp("raw::btreemap::clear", clear);
    putnfp("raw::btreemap::keys", keys);
}
