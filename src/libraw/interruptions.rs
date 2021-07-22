//!
//! Functions to work with euolaVM interruptions.
//!

use crate::{
    context::{intabort, putnfp},
    vmem::Var,
};
use anyhow::anyhow;

pub fn ign(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::context::intignore;

    intignore(
        &*unsafe { a.get_unchecked(0) }
            .as_sr()
            .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?
            .borrow()?,
    );
    Ok(())
}
pub fn abrt(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let id = id.borrow()?;
    let msg = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let msg = match msg.borrow() {
        Ok(x) => Some(x.to_owned()),
        Err(_) => None,
    };
    intabort(&id[..], msg);
    Ok(())
}
pub fn intcatch(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::{
        context::{getfp, intcatch as catchcore},
        isa::FuncPtr,
    };

    let id = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let id = id.borrow()?;
    let fp = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let fp = match getfp(&fp.borrow()?[..]).ok_or_else(|| anyhow!("raw::fatal::no_such_func"))? {
        FuncPtr::Virtual(x) => x,
        _ => return Err(anyhow!("raw::fatal::segfault")),
    };
    catchcore(&id[..], fp);
    Ok(())
}

/// Initialize the interruption handlers.
#[inline(always)]
pub fn init() {
    putnfp("raw::int::ignore", ign);
    putnfp("raw::int::abort", abrt);
    putnfp("raw::int::catch", intcatch);
}
