//!
//! Random number generating utilities of `libraw`.
//!

use crate::{context::putnfp, vmem::Var};
use anyhow::anyhow;
use getrandom::getrandom as os_getrand;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rand_chacha::*;
use rand_core::{RngCore, SeedableRng};
use std::mem::{size_of, transmute};

/// Chacha 8 Random Generator
static CHACHA8: Lazy<Mutex<ChaCha8Rng>> = Lazy::new(|| {
    Mutex::new(ChaCha8Rng::from_seed({
        let mut buf = [0u8; 32];
        os_getrand(&mut buf).ok();
        buf
    }))
});
/// Chacha 12 Random Generator
static CHACHA12: Lazy<Mutex<ChaCha12Rng>> = Lazy::new(|| {
    Mutex::new(ChaCha12Rng::from_seed({
        let mut buf = [0u8; 32];
        os_getrand(&mut buf).ok();
        buf
    }))
});
/// Chacha 20 Random Generator
static CHACHA20: Lazy<Mutex<ChaCha20Rng>> = Lazy::new(|| {
    Mutex::new(ChaCha20Rng::from_seed({
        let mut buf = [0u8; 32];
        os_getrand(&mut buf).ok();
        buf
    }))
});

macro_rules! impl_genrand {
    ($a: ty, $b: ident, $c: ident) => {
        pub fn $b(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let mut buf = [0u8; size_of::<$a>()];
            match os_getrand(&mut buf) {
                Ok(_) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::$c(unsafe { transmute(buf) });
                }
                Err(_) => return Err(anyhow!("raw::fatal::environment_error")),
            }
            Ok(())
        }
    };
}

macro_rules! impl_chacha {
    ($a: ty, $b: ident, $c: ident, $d: ident) => {
        pub fn $c(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let mut buf = [0u8; size_of::<$a>()];
            $b.lock().fill_bytes(&mut buf);
            *(unsafe { a.get_unchecked_mut(0) }) = Var::$d(unsafe { transmute(buf) });
            Ok(())
        }
    };
}

impl_genrand!(i8, osrng_i8, I8);
impl_genrand!(u8, osrng_u8, U8);
impl_genrand!(i16, osrng_i16, I16);
impl_genrand!(u16, osrng_u16, U16);
impl_genrand!(i32, osrng_i32, I32);
impl_genrand!(u32, osrng_u32, U32);
impl_genrand!(i64, osrng_i64, I64);
impl_genrand!(u64, osrng_u64, U64);

impl_chacha!(i8, CHACHA8, chacha8rng_i8, I8);
impl_chacha!(u8, CHACHA8, chacha8rng_u8, U8);
impl_chacha!(i16, CHACHA8, chacha8rng_i16, I16);
impl_chacha!(u16, CHACHA8, chacha8rng_u16, U16);
impl_chacha!(i32, CHACHA8, chacha8rng_i32, I32);
impl_chacha!(u32, CHACHA8, chacha8rng_u32, U32);
impl_chacha!(i64, CHACHA8, chacha8rng_i64, I64);
impl_chacha!(u64, CHACHA8, chacha8rng_u64, U64);

impl_chacha!(i8, CHACHA12, chacha12rng_i8, I8);
impl_chacha!(u8, CHACHA12, chacha12rng_u8, U8);
impl_chacha!(i16, CHACHA12, chacha12rng_i16, I16);
impl_chacha!(u16, CHACHA12, chacha12rng_u16, U16);
impl_chacha!(i32, CHACHA12, chacha12rng_i32, I32);
impl_chacha!(u32, CHACHA12, chacha12rng_u32, U32);
impl_chacha!(i64, CHACHA12, chacha12rng_i64, I64);
impl_chacha!(u64, CHACHA12, chacha12rng_u64, U64);

impl_chacha!(i8, CHACHA20, chacha20rng_i8, I8);
impl_chacha!(u8, CHACHA20, chacha20rng_u8, U8);
impl_chacha!(i16, CHACHA20, chacha20rng_i16, I16);
impl_chacha!(u16, CHACHA20, chacha20rng_u16, U16);
impl_chacha!(i32, CHACHA20, chacha20rng_i32, I32);
impl_chacha!(u32, CHACHA20, chacha20rng_u32, U32);
impl_chacha!(i64, CHACHA20, chacha20rng_i64, I64);
impl_chacha!(u64, CHACHA20, chacha20rng_u64, U64);

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::rand::os::get<i8>", osrng_i8);
    putnfp("raw::rand::os::get<u8>", osrng_u8);
    putnfp("raw::rand::os::get<i16>", osrng_i16);
    putnfp("raw::rand::os::get<u16>", osrng_u16);
    putnfp("raw::rand::os::get<i32>", osrng_i32);
    putnfp("raw::rand::os::get<u32>", osrng_u32);
    putnfp("raw::rand::os::get<i64>", osrng_i64);
    putnfp("raw::rand::os::get<u64>", osrng_u64);
    putnfp("raw::rand::chacha8::get<i8>", chacha8rng_i8);
    putnfp("raw::rand::chacha8::get<u8>", chacha8rng_u8);
    putnfp("raw::rand::chacha8::get<i16>", chacha8rng_i16);
    putnfp("raw::rand::chacha8::get<u16>", chacha8rng_u16);
    putnfp("raw::rand::chacha8::get<i32>", chacha8rng_i32);
    putnfp("raw::rand::chacha8::get<u32>", chacha8rng_u32);
    putnfp("raw::rand::chacha8::get<i64>", chacha8rng_i64);
    putnfp("raw::rand::chacha8::get<u64>", chacha8rng_u64);
    putnfp("raw::rand::chacha12::get<i8>", chacha12rng_i8);
    putnfp("raw::rand::chacha12::get<u8>", chacha12rng_u8);
    putnfp("raw::rand::chacha12::get<i16>", chacha12rng_i16);
    putnfp("raw::rand::chacha12::get<u16>", chacha12rng_u16);
    putnfp("raw::rand::chacha12::get<i32>", chacha12rng_i32);
    putnfp("raw::rand::chacha12::get<u32>", chacha12rng_u32);
    putnfp("raw::rand::chacha12::get<i64>", chacha12rng_i64);
    putnfp("raw::rand::chacha12::get<u64>", chacha12rng_u64);
    putnfp("raw::rand::chacha20::get<i8>", chacha20rng_i8);
    putnfp("raw::rand::chacha20::get<u8>", chacha20rng_u8);
    putnfp("raw::rand::chacha20::get<i16>", chacha20rng_i16);
    putnfp("raw::rand::chacha20::get<u16>", chacha20rng_u16);
    putnfp("raw::rand::chacha20::get<i32>", chacha20rng_i32);
    putnfp("raw::rand::chacha20::get<u32>", chacha20rng_u32);
    putnfp("raw::rand::chacha20::get<i64>", chacha20rng_i64);
    putnfp("raw::rand::chacha20::get<u64>", chacha20rng_u64);
}
