//!
//! Process library of `libraw`.
//!

use crate::{context::putnfp, vmem::Var};
use anyhow::anyhow;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::process::Child;

static PROCESSES: Lazy<DashMap<u32, Child, ahash::RandomState>> = Lazy::new(DashMap::default);

/// Spawn a process.
pub fn spawn(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let path = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let path = path.borrow()?;
    let args = unsafe { a.get_unchecked(1) };
    let argc = args.rcl()?;
    let mut argv = Vec::with_capacity(argc);
    for i in 0..argc {
        argv.push(
            args.offset_get(i)?
                .as_sr()
                .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?
                .to_string(),
        );
    }
    let stat = std::process::Command::new(&*path).args(&argv[..]).spawn();
    match stat {
        Ok(x) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U32(x.id());
            PROCESSES.insert(x.id(), x);
        }
        Err(_) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U32(0);
        }
    }
    Ok(())
}
/// Wait a process.
pub fn wait(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::vmem::{CreateNull, ObjectRef};

    let id = unsafe { a.get_unchecked(0) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    match PROCESSES.remove(&(id as u32)) {
        Some((_, mut x)) => match x.wait() {
            Ok(y) => {
                *(unsafe { a.get_unchecked_mut(0) }) = Var::Object({
                    let z = ObjectRef::new("raw::exit_status");
                    z.set("killed", Var::U8(y.code().is_none() as u8))?;
                    z.set("code", Var::I32(y.code().unwrap_or(i32::MAX)))?;
                    z
                })
            }
            Err(_) => *(unsafe { a.get_unchecked_mut(0) }) = Var::Object(ObjectRef::null()),
        },
        None => return Err(anyhow!("raw::fatal::segfault")),
    }
    Ok(())
}

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::proc::wait", wait);
    putnfp("raw::proc::spawn", spawn);
}
