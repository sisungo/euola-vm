//!
//! Raw environment API bindings, the `env` module of `libraw`.
//!

use crate::{
    context::{putnfp, Thread},
    executor,
    isa::VirtFuncPtr,
    libraw::iohmgr,
    vmem::Var,
};
use anyhow::anyhow;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

/// Console arguments.
static ARGS: Lazy<Vec<String>> = Lazy::new(|| std::env::args().collect());
/// Handler of Ctrl+C.
static CTRLC_HANDLER: Lazy<Mutex<Option<VirtFuncPtr>>> = Lazy::new(|| Mutex::new(None));

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::env::argc", argc);
    putnfp("raw::env::getarg", argv);
    putnfp("raw::env::getenv", getenv);
    putnfp("raw::env::setenv", setenv);
    putnfp("raw::env::unset", unset);
    putnfp("raw::env::exit", exit);
    putnfp("raw::env::abort", abort);
    putnfp("raw::env::getpid", getpid);
    putnfp("raw::env::catch_ctrlc", catch_sysint);
    putnfp("raw::env::chdir", cd);
    putnfp("raw::env::temp_dir", temp_dir);
}

/// Argument count.
pub fn argc(a: &mut [Var]) -> Result<(), anyhow::Error> {
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(ARGS.len() as u64);
    Ok(())
}

/// Change working directory.
pub fn cd(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let dir = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let dir = dir.borrow()?;
    match std::env::set_current_dir(&dir[..]) {
        Ok(()) => unsafe { *a.get_unchecked_mut(0) = Var::U64(0) },
        Err(x) => unsafe { *a.get_unchecked_mut(0) = Var::U64(iohmgr::error::from(x.kind())) },
    }
    Ok(())
}

/// Get temp directory.
pub fn temp_dir(a: &mut [Var]) -> Result<(), anyhow::Error> {
    *(unsafe { a.get_unchecked_mut(0) }) =
        Var::UString(std::env::temp_dir().to_string_lossy()[..].into());
    Ok(())
}

/// Catch system interruption.
pub fn catch_sysint(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::{context::getfp, isa::FuncPtr};

    let fp = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let fp = match getfp(&fp.borrow()?[..]) {
        Some(FuncPtr::Virtual(x)) => x,
        _ => return Err(anyhow!("raw::fatal::segfault")),
    };
    *CTRLC_HANDLER.lock() = Some(fp);
    unsafe {
        libc::signal(libc::SIGINT, ctrlc_handler as usize);
    }
    Ok(())
}

/// Ctrl-C handler.
extern "C" fn ctrlc_handler(_: libc::c_int) {
    executor::start(Thread::new(
        CTRLC_HANDLER.lock().as_ref().unwrap().clone(),
    ));
}

/// Get argument.
pub fn argv(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::vmem::{CreateNull, StringRef};

    let val = match ARGS.get(
        unsafe { a.get_unchecked(0) }
            .as_usize()
            .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))?,
    ) {
        Some(x) => x.to_owned().into(),
        None => StringRef::null(),
    };
    *(unsafe { a.get_unchecked_mut(0) }) = Var::UString(val);
    Ok(())
}

/// Get environment variable.
pub fn getenv(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::vmem::{CreateNull, StringRef};
    use std::env::var;

    let id = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let id = id.borrow()?;

    if id.contains('=') {
        return Err(anyhow!("raw::fatal::invalid_envid"));
    }

    match var(&**id) {
        Ok(y) => unsafe { *a.get_unchecked_mut(0) = Var::UString(StringRef::from(y)) },
        Err(_) => unsafe { *a.get_unchecked_mut(0) = Var::UString(StringRef::null()) },
    };
    Ok(())
}

/// Set environment variable.
pub fn setenv(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use std::env::set_var;

    let id = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let id = id.borrow()?;

    if id.contains('=') {
        return Err(anyhow!("raw::fatal::invalid_envid"));
    }
    let val = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let val = val.borrow()?;

    set_var(&**id, &**val);
    Ok(())
}

/// Unset an environment variable.
pub fn unset(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use std::env::remove_var;

    let id = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let id = id.borrow()?;
    if id.contains('=') {
        return Err(anyhow!("raw::fatal::invalid_envid"));
    }
    remove_var(&id[..]);
    Ok(())
}

/// Exit this process immediately.
pub fn exit(a: &mut [Var]) -> Result<(), anyhow::Error> {
    std::process::exit(
        unsafe { a.get_unchecked(0) }
            .as_usize()
            .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))? as i32,
    )
}

/// Abort this process immediately.
pub fn abort(_: &mut [Var]) -> Result<(), anyhow::Error> {
    std::process::abort()
}

/// Get PID of current process.
pub fn getpid(a: &mut [Var]) -> Result<(), anyhow::Error> {
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U32(std::process::id());
    Ok(())
}
