//!
//! Raw float math library. This provides only 64-bit floats, and no 32-bit floats are provided.
//! This uses `u64` as 64-bit float, and should not be used directly. Official `system` library's
//! `system::f64` module provides high-level abstraction of this library.
//!

use crate::{
    context::{putnfp, putstatic},
    vmem::Var,
};
use anyhow::anyhow;

macro_rules! impl_fpcalc {
    ($a: ident, $b: tt) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let val1 = f64::from_bits(
                unsafe { a.get_unchecked(0) }
                    .as_u64_strict()
                    .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?
            );
            let val2 = f64::from_bits(
                unsafe { a.get_unchecked(1) }
                    .as_u64_strict()
                    .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?
            );
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64((val1 $b val2).to_bits());
            Ok(())
        }
    }
}

/// Convert an integer to a float.
pub fn from_int(a: &mut [Var]) -> Result<(), anyhow::Error> {
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(
        unsafe { a.get_unchecked(0) }
            .as_f64()
            .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?
            .to_bits(),
    );
    Ok(())
}

/// Parse a string to float.
pub fn parse(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let tp = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let tp = tp.borrow()?;
    match tp.parse::<f64>() {
        Ok(x) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
            *(unsafe { a.get_unchecked_mut(1) }) = Var::U64(x.to_bits());
        }
        Err(_) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
        }
    }
    Ok(())
}

/// Convert to string.
pub fn to_str(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(val.to_string().into());
    Ok(())
}

/// Equal test.
pub fn eq(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val1 = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    let val2 = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8((val1 == val2) as u8);
    Ok(())
}

/// Convert from F64 to I64.
pub fn to_i64(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::I64(val as i64);
    Ok(())
}

/// Less-than test.
pub fn lt(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val1 = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    let val2 = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8((val1 < val2) as u8);
    Ok(())
}

/// More-than test.
pub fn mt(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val1 = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    let val2 = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8((val1 > val2) as u8);
    Ok(())
}

/// SQRT math algorithm.
pub fn sqrt(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(val.sqrt().to_bits());
    Ok(())
}

/// CBRT math algorithm.
pub fn cbrt(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe {
        f64::from_bits(
            a.get_unchecked(0)
                .as_u64_strict()
                .ok_or_else(|| anyhow!("raw::fatal::math_type_error"))?,
        )
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(val.cbrt().to_bits());
    Ok(())
}

impl_fpcalc!(add, +);
impl_fpcalc!(sub, -);
impl_fpcalc!(mul, *);
impl_fpcalc!(div, /);

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::f64::from<int>", from_int);
    putnfp("raw::f64::from<str>", parse);
    putnfp("raw::f64::down<i64>", to_i64);
    putnfp("raw::str::parse<f64>", parse);
    putnfp("raw::fpu::add", add);
    putnfp("raw::fpu::sub", sub);
    putnfp("raw::fpu::mul", mul);
    putnfp("raw::fpu::div", div);
    putnfp("raw::fpu::eq", eq);
    putnfp("raw::fpu::lt", lt);
    putnfp("raw::fpu::mt", mt);
    putnfp("raw::fpu::sqrt", sqrt);
    putnfp("raw::fpu::cbrt", cbrt);
    putnfp("raw::f64::to_string", to_str);
    putstatic("raw::f64::nan", Var::U64(f64::NAN.to_bits()));
    putstatic("raw::f64::inf", Var::U64(f64::INFINITY.to_bits()));
    putstatic(
        "raw::f64::neg_inf",
        Var::U64(f64::NEG_INFINITY.to_bits()),
    );
}
