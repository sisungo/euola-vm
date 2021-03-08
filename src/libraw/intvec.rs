//!
//! Int-Vector data structure's implemention in `libraw`.
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

/// ID Generator for deques.
static IDGEN: Mutex<IdGen> = const_mutex(IdGen::new());
/// Table of deques.
static DEQUES: Lazy<DashMap<u64, RwLock<Vec<i64>>, FakeHasher>> = Lazy::new(DashMap::default);

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
                    *a.get_unchecked_mut(1) = Var::I64(x);
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
            let val = unsafe { a.get_unchecked(1) }.as_i64().ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
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
    DEQUES.insert(id, RwLock::new(Vec::new()));
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
            *a.get_unchecked_mut(1) = Var::I64(*x);
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
    let val = unsafe { a.get_unchecked(2) }.as_i64().ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
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
    let val = unsafe { a.get_unchecked(2) }.as_i64().ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
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
                .for_each(|x| cur.write().push(x.to_owned()));
            Ok(())
        }
        Var::Bytes(x) => {
            x.borrow()?
                .par_iter()
                .for_each(|x| cur.write().push(*x as i64));
            Ok(())
        }
        _ => Err(anyhow!("raw::fatal::not_a_buf")),
    }
}

/// Content containing test.
pub fn contains(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let val = unsafe { a.get_unchecked(1) }.as_i64().ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let cur = match DEQUES.get(&id) {
        Some(x) => x,
        None => return Err(anyhow!("raw::fatal::segfault")),
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(cur.read().contains(&val) as u8);
    Ok(())
}

/// Timsort.
pub fn sort_tim(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }.as_u64_strict().ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let cur = match DEQUES.get(&id) {
        Some(x) => x,
        None => return Err(anyhow!("raw::fatal::segfault")),
    };
    cur.write().par_sort();
    Ok(())
}

/// Quicksort.
pub fn sort_uns(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }.as_u64_strict().ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let cur = match DEQUES.get(&id) {
        Some(x) => x,
        None => return Err(anyhow!("raw::fatal::segfault")),
    };
    cur.write().par_sort_unstable();
    Ok(())
}

impl_pop!(pop);
impl_push!(push);

/// Initialize the library.
#[inline(always)]
pub fn init() {
    IDGEN.lock().next();
    putnfp("raw::ints::new", new);
    putnfp("raw::ints::drop", drop);
    putnfp("raw::ints::get", get);
    putnfp("raw::ints::set", set);
    putnfp("raw::ints::swap", swap);
    putnfp("raw::ints::truncate", truncate);
    putnfp("raw::ints::len", len);
    putnfp("raw::ints::clear", clear);
    putnfp("raw::ints::pop", pop);
    putnfp("raw::ints::push", push);
    putnfp("raw::ints::insert", insert);
    putnfp("raw::ints::contains", contains);
    putnfp("raw::ints::append", append);
    putnfp("raw::ints::sort<timsort>", sort_tim);
    putnfp("raw::ints::sort<qsort>", sort_uns);
}
