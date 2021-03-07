//!
//! UTF-8 String operations. This provides `raw::string` type.
//!

use crate::{
    context::putnfp,
    vmem::{BytesRef, CreateNull, StringRef, Var},
};
use anyhow::anyhow;
use parking_lot::RwLock;
use rayon::prelude::*;
use std::convert::TryFrom;

macro_rules! impl_parse {
    ($a: ty, $b: ident, $c: ident) => {
        pub fn $b(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let parsed = unsafe { a.get_unchecked(0) }
                .as_sr()
                .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?
                .borrow()?
                .parse::<$a>();
            match parsed {
                Ok(x) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
                    *(unsafe { a.get_unchecked_mut(1) }) = Var::$c(x);
                }
                Err(_) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
                }
            }
            Ok(())
        }
    };
}

macro_rules! impl_trim {
    ($a: ident) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let untrimmed = unsafe { a.get_unchecked(0) }
                .as_sr()
                .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
            let untrimmed = untrimmed.borrow()?;
            let trimmed = untrimmed.$a();
            *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(trimmed.into());
            Ok(())
        }
    };
}

/// Push a char into the paassed string.
pub fn push(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let c = match unsafe { a.get_unchecked(1) } {
        Var::I8(x) => *x as u8 as char,
        Var::U8(x) => *x as char,
        Var::I16(x) => u16_to_char(*x as u16)?,
        Var::U16(x) => u16_to_char(*x)?,
        Var::I32(x) => u32_to_char(*x as u32)?,
        Var::U32(x) => u32_to_char(*x)?,
        _ => return Err(anyhow!("raw::fatal::not_a_char")),
    };
    let r = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    r.borrow_mut()?.push(c);
    Ok(())
}

/// Push a string into the passed string.
pub fn push_str(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?
        .borrow_mut()?
        .push_str(
            &*unsafe { a.get_unchecked(1) }
                .as_sr()
                .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?
                .borrow()?,
        );
    Ok(())
}

/// Create a new string from anything.
pub fn autofrom(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use std::fmt::Write;

    let mut buf: String;
    match unsafe { a.get_unchecked(0) } {
        Var::I8(x) => {
            buf = String::with_capacity(4);
            write!(buf, "{}", x).unwrap();
        }
        Var::U8(x) => {
            buf = String::with_capacity(4);
            write!(buf, "{}", x).unwrap();
        }
        Var::I16(x) => {
            buf = String::with_capacity(6);
            write!(buf, "{}", x).unwrap();
        }
        Var::U16(x) => {
            buf = String::with_capacity(6);
            write!(buf, "{}", x).unwrap();
        }
        Var::I32(x) => {
            buf = String::with_capacity(8);
            write!(buf, "{}", x).unwrap();
        }
        Var::U32(x) => {
            buf = String::with_capacity(8);
            write!(buf, "{}", x).unwrap();
        }
        Var::I64(x) => {
            buf = String::with_capacity(10);
            write!(buf, "{}", x).unwrap();
        }
        Var::U64(x) => {
            buf = String::with_capacity(10);
            write!(buf, "{}", x).unwrap();
        }
        Var::Usize(x) => {
            buf = String::with_capacity(10);
            write!(buf, "{}", x).unwrap();
        }
        Var::UString(x) => buf = x.to_string(),
        Var::Bytes(x) => match StringRef::try_from(x) {
            Ok(y) => {
                *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(y);
                return Ok(());
            }
            Err(_) => {
                buf = String::with_capacity(32);
                write!(buf, "{:?}", x).unwrap();
            }
        },
        Var::Vector(x) => {
            buf = String::with_capacity(32);
            write!(buf, "{:?}", x).unwrap();
        }
        Var::Object(x) => {
            buf = String::with_capacity(64);
            write!(buf, "{:?}", x).unwrap();
        }
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(StringRef::from(buf));
    Ok(())
}

/// From bytes.
pub fn from_bytes(a: &mut [Var]) -> Result<(), anyhow::Error> {
    match unsafe { a.get_unchecked(0) } {
        Var::Bytes(x) => {
            if x.is_null() {
                *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(StringRef::null());
            } else {
                match StringRef::try_from(x) {
                    Ok(y) => {
                        *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(y);
                    }
                    Err(_) => {
                        *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(StringRef::null());
                    }
                }
            }
            Ok(())
        }
        _ => Err(anyhow!("raw::fatal::not_a_buf")),
    }
}

/// From chars.
pub fn from_chars(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let sbuf = RwLock::new(String::new());
    for i in val.borrow()?.iter() {
        let ch = match *i {
            Var::I8(x) => x as u8 as char,
            Var::U8(x) => x as char,
            Var::I32(x) => match char::try_from(x as u32) {
                Ok(y) => y,
                Err(_) => return Err(anyhow!("raw::fatal::invalid")),
            },
            Var::U32(x) => match char::try_from(x) {
                Ok(y) => y,
                Err(_) => return Err(anyhow!("raw::fatal::invalid")),
            },
            _ => return Err(anyhow!("raw::fatal::not_an_integer")),
        };
        sbuf.write().push(ch);
    }
    *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(sbuf.into());
    Ok(())
}

/// Equal test for strings.
pub fn eq(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val1 = unsafe { a.get_unchecked(0) };
    let val2 = unsafe { a.get_unchecked(1) };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8((val1 == val2) as u8);
    Ok(())
}

/// Convert to bytes.
pub fn tb(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let val = val.borrow();
    match val {
        Ok(x) => unsafe { *a.get_unchecked_mut(0) = Var::Bytes((&x[..]).into()) },
        Err(_) => unsafe { *a.get_unchecked_mut(0) = Var::Bytes(BytesRef::null()) },
    }
    Ok(())
}

/// Convert to chars.
pub fn to_chars(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let ch = val.borrow()?;
    let result = RwLock::new(Vec::new());
    ch.par_chars()
        .for_each(|x| result.write().push(Var::U32(x as u32)));
    *(unsafe { a.get_unchecked_mut(0) }) = Var::Vector(result.into());
    Ok(())
}

/// Deep clone a string.
pub fn deep_clone(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    if val.is_null() {
        unsafe { *a.get_unchecked_mut(0) = Var::UString(StringRef::null()) }
    } else {
        unsafe { *a.get_unchecked_mut(0) = Var::UString(StringRef::from(val.to_string())) }
    }
    Ok(())
}

/// Split the string.
pub fn split(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::vmem::VectorRef;

    let sym = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let sym = sym.borrow()?;
    let val = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let val = val.borrow()?;
    let vr = VectorRef::empty();
    for i in val.split(&*sym) {
        vr.push(Var::UString(i.into()))?;
    }
    *(unsafe { a.get_unchecked_mut(0) }) = Var::Vector(vr);
    Ok(())
}

/// Judge if A contains B as a substring.
pub fn contains(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let val = val.borrow()?;
    let subval = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let subval = subval.borrow()?;
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(val.contains(&subval[..]) as u8);
    Ok(())
}

/// Create a string that handles an IO error integer.
pub fn from_ioerr(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let code = unsafe { a.get_unchecked(0) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))? as u64;
    let rslt = crate::libraw::iohmgr::error::to_string(code);
    *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(rslt.into());
    Ok(())
}

/// Substring replace.
pub fn replace(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let s = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let s = s.borrow()?;
    let src = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let src = src.borrow()?;
    let dest = unsafe { a.get_unchecked(2) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let dest = dest.borrow()?;
    *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(s.replace(&src[..], &dest[..]).into());
    Ok(())
}

/// Clear a string.
pub fn clear(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?
        .borrow_mut()?
        .clear();
    Ok(())
}

/// Judge if a string's content is ASCII-encoded.
pub fn is_ascii(a: &mut [Var]) -> Result<(), anyhow::Error> {
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(
        unsafe { a.get_unchecked(0) }
            .as_sr()
            .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?
            .borrow()?
            .is_ascii() as u8,
    );
    Ok(())
}

impl_parse!(i8, parse_i8, I8);
impl_parse!(u8, parse_u8, U8);
impl_parse!(i16, parse_i16, I16);
impl_parse!(u16, parse_u16, U16);
impl_parse!(i32, parse_i32, I32);
impl_parse!(u32, parse_u32, U32);
impl_parse!(i64, parse_i64, I64);
impl_parse!(u64, parse_u64, U64);
impl_trim!(trim);
impl_trim!(trim_start);
impl_trim!(trim_end);
impl_trim!(to_lowercase);
impl_trim!(to_uppercase);

/// Initialize the `string` library.
#[inline(always)]
pub fn init() {
    putnfp("raw::str::push_char", push);
    putnfp("raw::str::push_str", push_str);
    putnfp("raw::str::trim", trim);
    putnfp("raw::str::trim_start", trim_start);
    putnfp("raw::str::trim_end", trim_end);
    putnfp("raw::str::eq", eq);
    putnfp("raw::str::to_bytes", tb);
    putnfp("raw::str::split", split);
    putnfp("raw::str::parse<i8>", parse_i8);
    putnfp("raw::str::parse<u8>", parse_u8);
    putnfp("raw::str::parse<i16>", parse_i16);
    putnfp("raw::str::parse<u16>", parse_u16);
    putnfp("raw::str::parse<i32>", parse_i32);
    putnfp("raw::str::parse<u32>", parse_u32);
    putnfp("raw::str::parse<i64>", parse_i64);
    putnfp("raw::str::parse<u64>", parse_u64);
    putnfp("raw::str::from<auto>", autofrom);
    putnfp("raw::str::from<bytes>", from_bytes);
    putnfp("raw::str::from<io_error>", from_ioerr);
    putnfp("raw::str::from<chars>", from_chars);
    putnfp("raw::str::deep_clone", deep_clone);
    putnfp("raw::str::contains", contains);
    putnfp("raw::str::replace", replace);
    putnfp("raw::str::lowercase", to_lowercase);
    putnfp("raw::str::uppercase", to_uppercase);
    putnfp("raw::str::clear", clear);
    putnfp("raw::str::is_ascii", is_ascii);
    putnfp("raw::str::to_chars", to_chars);
}

/// Convert from U32 to char, returns an result with interruptable error.
fn u32_to_char(c: u32) -> Result<char, anyhow::Error> {
    match char::try_from(c) {
        Ok(x) => Ok(x),
        Err(_) => Err(anyhow!("raw::fatal::not_valid_unicode")),
    }
}
/// Convert from U16 to char, returns an result with interruptable error.
fn u16_to_char(c: u16) -> Result<char, anyhow::Error> {
    match String::from_utf16(&[c]) {
        Ok(x) => Ok(x.chars().next().unwrap()),
        Err(_) => Err(anyhow!("raw::fatal::not_valid_unicode")),
    }
}
