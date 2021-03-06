//!
//! This module provides `HashMap` data structure in `libraw`.
//!

use crate::{
    context::putnfp,
    libraw::iohmgr::{FakeHasher, IdGen},
    vmem::Var,
};
use anyhow::anyhow;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::{const_mutex, Mutex};

/// Id generator for hashmaps.
static IDGEN: Mutex<IdGen> = const_mutex(IdGen::new());
/// Table of hashmaps.
static TABLE: Lazy<DashMap<u64, DashMap<Var, Var, ahash::RandomState>, FakeHasher>> =
    Lazy::new(DashMap::default);

/// Make a new one.
pub fn new(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = IDGEN.lock().next();
    TABLE.insert(id, DashMap::default());
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
    let key = unsafe { a.get_unchecked(1) }.to_owned();
    let map = match TABLE.get(&id) {
        Some(x) => x,
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
            return Ok(());
        }
    };
    match map.get(&key) {
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
    map.par_iter()
        .for_each(|x| rslt.lock().push(x.key().to_owned()));
    *(unsafe { a.get_unchecked_mut(0) }) = Var::Vector(rslt.into_inner().into());
    Ok(())
}

/// Set a value to a map.
pub fn set(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let key = unsafe { a.get_unchecked(1) }.to_owned();
    let val = unsafe { a.get_unchecked(2) }.to_owned();
    let map = match TABLE.get(&id) {
        Some(x) => x,
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
            return Ok(());
        }
    };
    map.insert(key, val);
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
    Ok(())
}

/// Remove a value from a map.
pub fn remove(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let key = unsafe { a.get_unchecked(1) };
    let map = match TABLE.get(&id) {
        Some(x) => x,
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
            return Ok(());
        }
    };
    match map.remove(key) {
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
    map.clear();
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
    Ok(())
}

/// Initialize the library.
#[inline(always)]
pub fn init() {
    IDGEN.lock().next();
    putnfp("raw::hashmap::new", new);
    putnfp("raw::hashmap::remove", remove);
    putnfp("raw::hashmap::get", get);
    putnfp("raw::hashmap::set", set);
    putnfp("raw::hashmap::drop", drop);
    putnfp("raw::hashmap::clear", clear);
    putnfp("raw::hashmap::keys", keys);
}
