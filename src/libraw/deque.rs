//!
//! Deque data structure's implemention in `libraw`.
//!

use crate::{
    context::putnfp,
    libraw::iohmgr::{FakeHasher, IdGen},
    vmem::Var,
};
use anyhow::anyhow;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::{const_mutex, Mutex, RwLock};
use rayon::prelude::*;
use std::collections::VecDeque;

/// ID Generator for deques.
static IDGEN: Mutex<IdGen> = const_mutex(IdGen::new());
/// Table of deques.
static DEQUES: Lazy<DashMap<u64, RwLock<VecDeque<Var>>, FakeHasher>> = Lazy::new(DashMap::default);

macro_rules! impl_pop {
    ($a: ident) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let id = unsafe { a.get_unchecked(0) }
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
            let deque = match DEQUES.get(&id) {
                Some(x) => x,
                None => unsafe {
                    *a.get_unchecked_mut(0) = Var::U8(0);
                    return Ok(());
                },
            };
            match deque.write().$a() {
                Some(x) => unsafe {
                    *a.get_unchecked_mut(0) = Var::U8(1);
                    *a.get_unchecked_mut(1) = x;
                },
                None => unsafe {
                    *a.get_unchecked_mut(0) = Var::U8(0);
                },
            }
            Ok(())
        }
    };
}

macro_rules! impl_push {
    ($a: ident) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let id = unsafe { a.get_unchecked(0) }
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
            let val = unsafe { a.get_unchecked(1) }.to_owned();
            let deque = match DEQUES.get(&id) {
                Some(x) => x,
                None => unsafe {
                    *a.get_unchecked_mut(0) = Var::U8(0);
                    return Ok(());
                },
            };
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
            deque.write().$a(val);
            Ok(())
        }
    };
}

/// Create a new one.
pub fn new(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = IDGEN.lock().next();
    DEQUES.insert(id, RwLock::new(VecDeque::new()));
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(id);
    Ok(())
}

/// Drop a deque.
pub fn drop(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    if DEQUES.remove(&id).is_some() {
        IDGEN.lock().free(id);
    }
    Ok(())
}

/// Get a value.
pub fn get(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let index = unsafe { a.get_unchecked(1) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let deque = match DEQUES.get(&id) {
        Some(x) => x,
        None => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
            return Ok(());
        },
    };
    match deque.read().get(index) {
        Some(x) => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(1);
            *a.get_unchecked_mut(1) = x.to_owned();
        },
        None => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
        },
    }
    Ok(())
}

/// Set a value.
pub fn set(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let index = unsafe { a.get_unchecked(1) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let val = unsafe { a.get_unchecked(2) }.to_owned();
    let deque = match DEQUES.get(&id) {
        Some(x) => x,
        None => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
            return Ok(());
        },
    };
    match deque.write().get_mut(index) {
        Some(x) => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(1);
            *x = val;
        },
        None => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
        },
    }
    Ok(())
}

/// Swap values.
pub fn swap(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let index1 = unsafe { a.get_unchecked(1) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let index2 = unsafe { a.get_unchecked(2) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let deque = match DEQUES.get(&id) {
        Some(x) => x,
        None => return Err(anyhow!("raw::fatal::segfault")),
    };
    let mut deque = deque.write();
    let len = deque.len();
    if index1 >= len || index2 >= len {
        Err(anyhow!("raw::fatal::out_of_range"))
    } else {
        deque.swap(index1, index2);
        Ok(())
    }
}

/// Truncate.
pub fn truncate(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let newlen = unsafe { a.get_unchecked(1) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let deque = match DEQUES.get(&id) {
        Some(x) => x,
        None => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
            return Ok(());
        },
    };
    deque.write().truncate(newlen);
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
    Ok(())
}

/// Get length.
pub fn len(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let deque = match DEQUES.get(&id) {
        Some(x) => x,
        None => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
            return Ok(());
        },
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
    *(unsafe { a.get_unchecked_mut(1) }) = Var::U64(deque.read().len() as u64);
    Ok(())
}

/// Clear.
pub fn clear(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let deque = match DEQUES.get(&id) {
        Some(x) => x,
        None => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
            return Ok(());
        },
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
    deque.write().clear();
    Ok(())
}

/// Insert.
pub fn insert(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let cur = match DEQUES.get(&id) {
        Some(x) => x,
        _ => return Err(anyhow!("raw::fatal::segfault")),
    };
    let mut cur = cur.write();
    let index = unsafe { a.get_unchecked(1) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let val = unsafe { a.get_unchecked(2) }.to_owned();
    if index > cur.len() {
        Err(anyhow!("raw::fatal::out_of_range"))
    } else {
        cur.insert(index, val);
        Ok(())
    }
}

/// Remove an element from the deque.
pub fn remove(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let cur = match DEQUES.get(&id) {
        Some(x) => x,
        None => return Err(anyhow!("raw::fatal::segfault")),
    };
    let mut cur = cur.write();

    let index = unsafe { a.get_unchecked(1) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    if index >= cur.len() {
        return Err(anyhow!("raw::fatal::out_of_range"));
    }
    cur.remove(index);
    Ok(())
}

/// Convert to vector.
pub fn to_vec(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let cur = match DEQUES.get(&id) {
        Some(x) => x,
        None => return Err(anyhow!("raw::fatal::segfault")),
    };
    *(unsafe { a.get_unchecked_mut(0) }) =
        Var::Vector(Into::<Vec<_>>::into(cur.read().to_owned()).into());
    Ok(())
}

/// Create a new one from a vector.
pub fn from_vec(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = IDGEN.lock().next();
    DEQUES.insert(
        id,
        RwLock::new(VecDeque::from(Into::<Vec<Var>>::into(
            &match unsafe { a.get_unchecked(0) } {
                Var::Vector(x) => x,
                _ => return Err(anyhow!("raw::fatal::not_a_buf")),
            }
            .borrow()?[..],
        ))),
    );
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(id);
    Ok(())
}

/// Append elements.
pub fn append(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let cur = match DEQUES.get(&id) {
        Some(x) => x,
        None => return Err(anyhow!("raw::fatal::segfault")),
    };
    match unsafe { a.get_unchecked(1) } {
        Var::U64(x) => {
            let ndeq = match DEQUES.get(&x) {
                Some(x) => x,
                None => return Err(anyhow!("raw::fatal::segfault")),
            };
            ndeq.read()
                .par_iter()
                .for_each(|x| cur.write().push_back(x.to_owned()));
            Ok(())
        }
        Var::Vector(x) => {
            x.borrow()?
                .par_iter()
                .for_each(|x| cur.write().push_back(x.to_owned()));
            Ok(())
        }
        _ => Err(anyhow!("raw::fatal::not_a_buf")),
    }
}

impl_pop!(pop_front);
impl_pop!(pop_back);
impl_push!(push_front);
impl_push!(push_back);

/// Initialize the library.
#[inline(always)]
pub fn init() {
    IDGEN.lock().next();
    putnfp("raw::deque::new", new);
    putnfp("raw::deque::drop", drop);
    putnfp("raw::deque::to_vec", to_vec);
    putnfp("raw::deque::from_vec", from_vec);
    putnfp("raw::deque::get", get);
    putnfp("raw::deque::set", set);
    putnfp("raw::deque::swap", swap);
    putnfp("raw::deque::truncate", truncate);
    putnfp("raw::deque::len", len);
    putnfp("raw::deque::clear", clear);
    putnfp("raw::deque::pop_front", pop_front);
    putnfp("raw::deque::pop_back", pop_back);
    putnfp("raw::deque::push_front", push_front);
    putnfp("raw::deque::push_back", push_back);
    putnfp("raw::deque::insert", insert);
    putnfp("raw::deque::append", append);
}
