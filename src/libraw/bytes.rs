//!
//! Bytes operating library of `libraw`.
//!

use crate::{context::putnfp, vmem::Var};
use anyhow::anyhow;

macro_rules! impl_fromint {
    ($a: ident, $b: ident, $c: ident) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let val = match unsafe { a.get_unchecked(0) } {
                Var::$c(x) => x,
                _ => return Err(anyhow!("raw::fatal::not_an_integer")),
            };
            *(unsafe { a.get_unchecked_mut(0) }) = Var::Bytes(val.$b().to_vec().into());
            Ok(())
        }
    };
}

macro_rules! impl_rslvint {
    ($a: ident, $b: ident, $c: ty, $d: ident) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let val = match unsafe { a.get_unchecked(0) } {
                Var::Bytes(x) => x,
                _ => return Err(anyhow!("raw::fatal::not_a_buf")),
            }
            .clone();
            let val = val.borrow()?;
            let mut buf = [0u8; std::mem::size_of::<$c>()];
            val.iter().enumerate().for_each(|(c, i)| {
                if (0..std::mem::size_of::<$c>()).contains(&c) {
                    unsafe { *buf.get_unchecked_mut(c) = *i }
                }
            });
            *(unsafe { a.get_unchecked_mut(0) }) = Var::$b(<$c>::$d(buf));
            Ok(())
        }
    };
}

/// Push an element into a vector.
pub fn push(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => {
            x.push(
                unsafe { a.get_unchecked(1) }
                    .as_u8()
                    .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?,
            )?;
        }
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    Ok(())
}

/// Clear the vector.
pub fn clear(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => {
            x.clear()?;
        }
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    Ok(())
}

/// Convert to pointer.
pub fn to_ptr(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    .borrow_mut()?
    .as_mut_ptr();
    *(unsafe { a.get_unchecked_mut(0) }) = Var::Usize(val as usize);
    Ok(())
}

/// Truncate the vector.
pub fn truncate(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => {
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
        Var::Bytes(x) => {
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
        Var::Bytes(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let mut v = v.borrow_mut()?;
    match v.pop() {
        Some(x) => unsafe {
            drop(v);
            *a.get_unchecked_mut(0) = Var::U8(1);
            *a.get_unchecked_mut(1) = Var::U8(x);
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
        Var::Bytes(x) => {
            x.borrow_mut()?.resize(
                unsafe { a.get_unchecked(1) }
                    .as_usize()
                    .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?,
                unsafe { a.get_unchecked(2) }
                    .as_u8()
                    .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?,
            );
        }
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    Ok(())
}

/// Append.
pub fn append(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let cur = match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => x.clone(),
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let arg = match unsafe { a.get_unchecked(1) } {
        Var::Bytes(x) => x.clone(),
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let arg = arg.borrow()?;
    for i in arg.iter() {
        cur.push(i.to_owned())?;
    }
    Ok(())
}

/// Equal test for bytes.
pub fn eq(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val1 = unsafe { a.get_unchecked(0) };
    let val2 = unsafe { a.get_unchecked(1) };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8((val1 == val2) as u8);
    Ok(())
}

/// Insert.
pub fn insert(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let cur = match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => x.clone(),
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let mut cur = cur.borrow_mut()?;
    let index = unsafe { a.get_unchecked(1) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let val = unsafe { a.get_unchecked(2) }
        .as_u8()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    if index > cur.len() {
        Err(anyhow!("raw::fatal::out_of_range"))
    } else {
        cur.insert(index, val);
        Ok(())
    }
}

/// Deep clone.
pub fn deep_clone(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let cur = match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => x.clone(),
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::Bytes(cur.borrow()?.clone().into());
    Ok(())
}

/// Containing test.
pub fn contains(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let cur = match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    .clone();
    let val = unsafe { a.get_unchecked(1) }
        .as_u8()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(cur.borrow()?.contains(&val) as u8);
    Ok(())
}

impl_fromint!(i8_as_be_bytes, to_be_bytes, I8);
impl_fromint!(i8_as_le_bytes, to_le_bytes, I8);
impl_fromint!(u8_as_be_bytes, to_be_bytes, U8);
impl_fromint!(u8_as_le_bytes, to_le_bytes, U8);
impl_fromint!(i16_as_be_bytes, to_be_bytes, I16);
impl_fromint!(i16_as_le_bytes, to_le_bytes, I16);
impl_fromint!(u16_as_be_bytes, to_be_bytes, U16);
impl_fromint!(u16_as_le_bytes, to_le_bytes, U16);
impl_fromint!(i32_as_be_bytes, to_be_bytes, I32);
impl_fromint!(i32_as_le_bytes, to_le_bytes, I32);
impl_fromint!(u32_as_be_bytes, to_be_bytes, U32);
impl_fromint!(u32_as_le_bytes, to_le_bytes, U32);
impl_fromint!(i64_as_be_bytes, to_be_bytes, I64);
impl_fromint!(i64_as_le_bytes, to_le_bytes, I64);
impl_fromint!(u64_as_be_bytes, to_be_bytes, U64);
impl_fromint!(u64_as_le_bytes, to_le_bytes, U64);
impl_rslvint!(i8_from_be_bytes, I8, i8, from_be_bytes);
impl_rslvint!(i8_from_le_bytes, I8, i8, from_le_bytes);
impl_rslvint!(u8_from_be_bytes, U8, u8, from_be_bytes);
impl_rslvint!(u8_from_le_bytes, U8, u8, from_le_bytes);
impl_rslvint!(i16_from_be_bytes, I16, i16, from_be_bytes);
impl_rslvint!(i16_from_le_bytes, I16, i16, from_le_bytes);
impl_rslvint!(u16_from_be_bytes, U16, u16, from_be_bytes);
impl_rslvint!(u16_from_le_bytes, U16, u16, from_le_bytes);
impl_rslvint!(i32_from_be_bytes, I32, i32, from_be_bytes);
impl_rslvint!(i32_from_le_bytes, I32, i32, from_le_bytes);
impl_rslvint!(u32_from_be_bytes, U32, u32, from_be_bytes);
impl_rslvint!(u32_from_le_bytes, U32, u32, from_le_bytes);
impl_rslvint!(i64_from_be_bytes, I64, i64, from_be_bytes);
impl_rslvint!(i64_from_le_bytes, I64, i64, from_le_bytes);
impl_rslvint!(u64_from_be_bytes, U64, u64, from_be_bytes);
impl_rslvint!(u64_from_le_bytes, U64, u64, from_le_bytes);

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::bytes::push", push);
    putnfp("raw::bytes::pop", pop);
    putnfp("raw::bytes::clear", clear);
    putnfp("raw::bytes::truncate", truncate);
    putnfp("raw::bytes::remove", remove);
    putnfp("raw::bytes::insert", insert);
    putnfp("raw::bytes::resize", resize);
    putnfp("raw::bytes::append", append);
    putnfp("raw::bytes::contains", contains);
    putnfp("raw::bytes::from<i8,be>", i8_as_be_bytes);
    putnfp("raw::bytes::from<i8,le>", i8_as_le_bytes);
    putnfp("raw::bytes::from<u8,be>", u8_as_be_bytes);
    putnfp("raw::bytes::from<u8,le>", u8_as_le_bytes);
    putnfp("raw::bytes::from<i16,be>", i16_as_be_bytes);
    putnfp("raw::bytes::from<i16,le>", i16_as_le_bytes);
    putnfp("raw::bytes::from<u16,be>", u16_as_be_bytes);
    putnfp("raw::bytes::from<u16,le>", u16_as_le_bytes);
    putnfp("raw::bytes::from<i32,be>", i32_as_be_bytes);
    putnfp("raw::bytes::from<i32,le>", i32_as_le_bytes);
    putnfp("raw::bytes::from<u32,be>", u32_as_be_bytes);
    putnfp("raw::bytes::from<u32,le>", u32_as_le_bytes);
    putnfp("raw::bytes::from<i64,be>", i64_as_be_bytes);
    putnfp("raw::bytes::from<i64,le>", i64_as_le_bytes);
    putnfp("raw::bytes::from<u64,be>", u64_as_be_bytes);
    putnfp("raw::bytes::from<u64,le>", u64_as_le_bytes);
    putnfp("raw::bytes::resolve<i8,be>", i8_from_be_bytes);
    putnfp("raw::bytes::resolve<i8,le>", i8_from_le_bytes);
    putnfp("raw::bytes::resolve<u8,be>", u8_from_be_bytes);
    putnfp("raw::bytes::resolve<u8,le>", u8_from_le_bytes);
    putnfp("raw::bytes::resolve<i16,be>", i16_from_be_bytes);
    putnfp("raw::bytes::resolve<i16,le>", i16_from_le_bytes);
    putnfp("raw::bytes::resolve<u16,be>", u16_from_be_bytes);
    putnfp("raw::bytes::resolve<u16,le>", u16_from_le_bytes);
    putnfp("raw::bytes::resolve<i32,be>", i32_from_be_bytes);
    putnfp("raw::bytes::resolve<i32,le>", i32_from_le_bytes);
    putnfp("raw::bytes::resolve<u32,be>", u32_from_be_bytes);
    putnfp("raw::bytes::resolve<u32,le>", u32_from_le_bytes);
    putnfp("raw::bytes::resolve<i64,be>", i64_from_be_bytes);
    putnfp("raw::bytes::resolve<i64,le>", i64_from_le_bytes);
    putnfp("raw::bytes::resolve<u64,be>", u64_from_be_bytes);
    putnfp("raw::bytes::resolve<u64,le>", u64_from_le_bytes);
    putnfp("raw::bytes::to_ptr", to_ptr);
    putnfp("raw::bytes::eq", eq);
    putnfp("raw::bytes::deep_clone", deep_clone);
}
