//!
//! Vector operating library of `libraw`.
//!

use crate::{context::putnfp, vmem::Var};
use anyhow::anyhow;

/// Push an element into a vector.
pub fn push(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => {
            x.push(unsafe { a.get_unchecked(1) }.clone())?;
        }
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    Ok(())
}

/// Clear the vector.
pub fn clear(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => {
            x.clear()?;
        }
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    Ok(())
}

/// Truncate the vector.
pub fn truncate(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => {
            x.borrow_mut()?.truncate(
                unsafe { a.get_unchecked(1) }
                    .as_usize()
                    .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?,
            );
        }
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    Ok(())
}

/// Remove an element from the vector.
pub fn remove(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => {
            let index = unsafe { a.get_unchecked(1) }
                .as_usize()
                .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
            if index >= x.len()? {
                return Err(anyhow!("raw::fatal::out_of_range"));
            }
            x.borrow_mut()?.remove(index);
        }
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    Ok(())
}

/// Pop an element from this vec.
pub fn pop(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let v = match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let mut v = v.borrow_mut()?;
    match v.pop() {
        Some(x) => unsafe {
            drop(v);
            *a.get_unchecked_mut(0) = Var::U8(1);
            *a.get_unchecked_mut(1) = x;
        },
        None => unsafe {
            drop(v);
            *a.get_unchecked_mut(0) = Var::U8(0);
        },
    }
    Ok(())
}

/// Resize a vector, truncate or fill.
pub fn resize(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => {
            x.borrow_mut()?.resize(
                unsafe { a.get_unchecked(1) }
                    .as_usize()
                    .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?,
                unsafe { a.get_unchecked(2) }.to_owned(),
            );
        }
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    Ok(())
}

/// Append.
pub fn append(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let cur = match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => x.clone(),
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let arg = match unsafe { a.get_unchecked(1) } {
        Var::Vector(x) => x.clone(),
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let arg = arg.borrow()?;
    for i in arg.iter() {
        cur.push(i.to_owned())?;
    }
    Ok(())
}

/// Insert.
pub fn insert(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let cur = match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => x.clone(),
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let mut cur = cur.borrow_mut()?;
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

/// From bytes.
pub fn frombytes(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::Vector(val.into());
    Ok(())
}

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::vec::push", push);
    putnfp("raw::vec::pop", pop);
    putnfp("raw::vec::clear", clear);
    putnfp("raw::vec::truncate", truncate);
    putnfp("raw::vec::remove", remove);
    putnfp("raw::vec::insert", insert);
    putnfp("raw::vec::resize", resize);
    putnfp("raw::vec::append", append);
    putnfp("raw::vec::from<bytes>", frombytes);
}
