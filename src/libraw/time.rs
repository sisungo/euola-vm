//!
//! Utilities for getting/formatting time of `libraw`.
//!

use crate::{context::putnfp, vmem::Var};
use anyhow::anyhow;
use chrono::prelude::*;

/// Get UNIX timestamp.
pub fn gettime(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe {
        *a.get_unchecked_mut(0) = Var::I64(Utc::now().timestamp());
    }
    Ok(())
}

/// Get UNIX timestamp.
pub fn gettime_ms(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe {
        *a.get_unchecked_mut(0) = Var::I64(Utc::now().timestamp_millis());
    }
    Ok(())
}

/// Get UNIX timestamp.
pub fn gettime_ns(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe {
        *a.get_unchecked_mut(0) = Var::I64(Utc::now().timestamp_nanos());
    }
    Ok(())
}

/// Get UNIX timestamp.
pub fn gettime_lms(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe {
        *a.get_unchecked_mut(0) = Var::I64(Local::now().timestamp_millis());
    }
    Ok(())
}

/// Get UNIX timestamp.
pub fn gettime_l(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe {
        *a.get_unchecked_mut(0) = Var::I64(Local::now().timestamp());
    }
    Ok(())
}

/// Get UNIX timestamp.
pub fn gettime_lns(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe {
        *a.get_unchecked_mut(0) = Var::I64(Local::now().timestamp_nanos());
    }
    Ok(())
}

/// Format UNIX timestamp.
pub fn tformat(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let ts = unsafe { a.get_unchecked(0) }
        .as_i64()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let fmt = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let fmt = fmt.borrow()?;
    let result = NaiveDateTime::from_timestamp(ts, 0)
        .format(&fmt[..])
        .to_string();
    *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(result.into());
    Ok(())
}

/// Parse string-formatted time.
pub fn parse_s(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let src = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let fmt = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let src = src.borrow()?;
    let fmt = fmt.borrow()?;
    let result = NaiveDateTime::parse_from_str(&src[..], &fmt[..]);
    match result {
        Ok(x) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
            *(unsafe { a.get_unchecked_mut(1) }) = Var::I64(x.timestamp());
        }
        Err(_) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
        }
    }
    Ok(())
}

/// Parse string-formatted time.
pub fn parse_ms(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let src = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let fmt = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let src = src.borrow()?;
    let fmt = fmt.borrow()?;
    let result = NaiveDateTime::parse_from_str(&src[..], &fmt[..]);
    match result {
        Ok(x) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
            *(unsafe { a.get_unchecked_mut(1) }) = Var::I64(x.timestamp_millis());
        }
        Err(_) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
        }
    }
    Ok(())
}

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::time::get<utc,sec>", gettime);
    putnfp("raw::time::get<utc,msec>", gettime_ms);
    putnfp("raw::time::get<utc,nsec>", gettime_ns);
    putnfp("raw::time::get<local,msec>", gettime_lms);
    putnfp("raw::time::get<local,sec>", gettime_l);
    putnfp("raw::time::get<local,nsec>", gettime_lns);
    putnfp("raw::time::format", tformat);
    putnfp("raw::str::parse<sec>", parse_s);
    putnfp("raw::str::parse<msec>", parse_ms);
}
