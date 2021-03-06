//!
//! Dynamic-loading support of `libraw`. This provides both system and VM dynamic-loading support.
//! VM Dynamic-loading is safe, however, system dynamic-loading is pretty unsafe.
//!

use crate::{
    context::{self, getfp, putnfp, Thread},
    executor,
    isa::FuncPtr,
    resolver,
    vmem::{CreateNull, StringRef, Var},
};
use anyhow::anyhow;

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::dl::vload", vload);
    putnfp("raw::coro::enter", coroenter);
    putnfp("raw::vhw::dump", dump);
    putnfp("raw::vhw::expand", expand);
}

/// Core dump context.
pub fn dump(a: &mut [Var]) -> Result<(), anyhow::Error> {
    unsafe {
        *a.get_unchecked_mut(0) = Var::UString((&*context::dump()).into());
    }
    Ok(())
}

/// Expand SIL.
pub fn expand(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let rb = match unsafe { a.get_unchecked(0) } {
        Var::Vector(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    .clone();
    let rb = rb.borrow()?;
    if rb.len() > 50 {
        return Err(anyhow!("raw::fatal::segfault"));
    }
    for i in 0..rb.len() {
        unsafe {
            *a.get_unchecked_mut(i) = rb.get_unchecked(i).to_owned();
        }
    }
    Ok(())
}

/// Dynamically load a VM library from file.
pub fn vload(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let path = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let path = path.borrow()?;
    match resolver::resolve(&*path) {
        Ok(()) => unsafe { *a.get_unchecked_mut(0) = Var::UString(StringRef::null()) },
        Err(x) => unsafe { *a.get_unchecked_mut(0) = Var::UString(x.to_string().into()) },
    }
    Ok(())
}

/// Enter coro mode.
pub fn coroenter(_: &mut [Var]) -> Result<(), anyhow::Error> {
    match getfp("_start_coro") {
        Some(FuncPtr::Virtual(x)) => {
            executor::start_coro(Thread::new(x));
            Ok(())
        }
        _ => Err(anyhow!("raw::fatal::segfault")),
    }
}
