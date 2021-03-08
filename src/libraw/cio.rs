//!
//! Console I/O library of `libraw`.
//!

use crate::{
    context::putnfp,
    vmem::{BytesRef, CreateNull, StringRef, Var},
};
use anyhow::anyhow;
use parking_lot::RwLockReadGuard;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write};

macro_rules! to_buf {
    ($a: expr) => {
        match $a {
            Var::UString(x) => Ok(RwLockReadGuard::map(
                x.0.as_ref()
                    .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
                    .read(),
                |y| y.as_bytes(),
            )),
            Var::Bytes(x) => Ok(RwLockReadGuard::map(
                x.0.as_ref()
                    .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
                    .read(),
                |y| &y[..],
            )),
            _ => Err(anyhow!("raw::fatal::not_a_buf")),
        }
    };
}

macro_rules! impl_print {
    ($a: ident, $b: ident) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let x = $b()
                .write_all(&to_buf!(unsafe { a.get_unchecked(0) })?)
                .is_ok();
            match x {
                true => unsafe { *a.get_unchecked_mut(0) = Var::U8(1) },
                false => unsafe { *a.get_unchecked_mut(0) = Var::U8(0) },
            }
            Ok(())
        }
    };
}

macro_rules! impl_flush {
    ($a: ident, $b: ident) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            match $b().flush() {
                Ok(()) => unsafe { *a.get_unchecked_mut(0) = Var::U8(1) },
                Err(_) => unsafe { *a.get_unchecked_mut(0) = Var::U8(0) },
            }
            Ok(())
        }
    };
}

impl_print!(print, stdout);
impl_flush!(flush, stdout);

/// Read a line into a UString.
pub fn read_str(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let mut buf = String::new();
    match stdin().read_line(&mut buf).is_ok() {
        true => unsafe { *a.get_unchecked_mut(0) = Var::UString(buf.into()) },
        false => unsafe { *a.get_unchecked_mut(0) = Var::UString(StringRef::null()) },
    }
    Ok(())
}

/// Read a line to a Bytes.
pub fn read_bytes(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let mut buf = Vec::new();
    let mut reader = BufReader::new(stdin());
    match reader.read_until(0xA, &mut buf).is_ok() {
        true => unsafe { *a.get_unchecked_mut(0) = Var::Bytes(buf.into()) },
        false => unsafe { *a.get_unchecked_mut(0) = Var::Bytes(BytesRef::null()) },
    }
    Ok(())
}

impl_print!(eprint, stderr);
impl_flush!(eflush, stderr);

/// Initialize the IO library.
#[inline(always)]
pub fn init() {
    putnfp("raw::cio::print", print);
    putnfp("raw::cio::read<str>", read_str);
    putnfp("raw::cio::read<bytes>", read_bytes);
    putnfp("raw::cio::flush", flush);
    putnfp("raw::cio::eflush", eflush);
    putnfp("raw::cio::eprint", eprint);
}
