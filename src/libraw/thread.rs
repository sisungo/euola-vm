//!
//! Threading module of `libraw`. This also provides syncing utilities.
//!

use crate::{
    context::{putnfp, ExecUnit, Thread},
    libraw::iohmgr::{self, RawObject},
    vmem::Var,
};
use anyhow::anyhow;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    /// TLS Map of current thread.
    static TLS_MAP: RefCell<HashMap<Box<str>, Var, ahash::RandomState>> = RefCell::new(HashMap::default());
}

/// Set a TLS.
pub fn tls_set(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let key = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let key = key.borrow()?;
    let val = unsafe { a.get_unchecked(1) }.to_owned();
    TLS_MAP.with(|x| x.borrow_mut().insert(Box::from(&key[..]), val));
    Ok(())
}

/// Get a TLS.
pub fn tls_get(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let name = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let name = name.borrow()?;
    TLS_MAP.with(|x| match x.borrow().get(&name[..]) {
        Some(y) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
            *(unsafe { a.get_unchecked_mut(1) }) = y.to_owned();
        }
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0);
        }
    });
    Ok(())
}

/// Delete a TLS.
pub fn tls_del(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let name = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let name = name.borrow()?;
    TLS_MAP.with(|x| x.borrow_mut().remove(&name[..]));
    Ok(())
}

/// Spawn a thread.
pub fn spawn(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use crate::{context::getfp, executor::start, isa::FuncPtr};

    let fp = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let fp = fp.borrow()?;
    let args = match unsafe { a.get_unchecked(1) } {
        Var::Vector(x) => x.borrow()?,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    };
    let mut new_thread = Thread::new(
        match match getfp(&fp) {
            Some(x) => x,
            None => return Err(anyhow!("raw::fatal::segfault")),
        } {
            FuncPtr::Virtual(x) => x,
            _ => return Err(anyhow!("raw::fatal::segfault")),
        },
    );
    if args.len() > 50 {
        return Err(anyhow!("raw::fatal::segfault"));
    }
    for i in 0..args.len() {
        if new_thread
            .sset(100 + i, unsafe { args.get_unchecked(i).to_owned() })
            .is_err()
        {
            unsafe { std::hint::unreachable_unchecked() }
        }
    }
    let builder = std::thread::Builder::new().name("secondary".to_owned());
    drop(args);
    match builder.spawn(move || {
        start(new_thread);
    }) {
        Ok(x) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
            *(unsafe { a.get_unchecked_mut(1) }) = Var::U64(iohmgr::add(RawObject::Thread(x)))
        }
        Err(_) => *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(0),
    }
    Ok(())
}

/// Join a thread.
pub fn join(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let handle = iohmgr::take(id);
    match match handle {
        Some(x) => x,
        None => return Ok(()),
    } {
        RawObject::Thread(x) => {
            x.join().unwrap();
        }
        _ => return Err(anyhow!("raw::fatal::segfault")),
    }
    Ok(())
}

/// Sleep this thread.
pub fn msleep(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let time = unsafe { a.get_unchecked(0) }
        .as_u64()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    std::thread::sleep(std::time::Duration::from_millis(time));
    Ok(())
}

/// Sleep this thread.
pub fn sleep(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let time = unsafe { a.get_unchecked(0) }
        .as_u64()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    std::thread::sleep(std::time::Duration::from_secs(time));
    Ok(())
}

/// Sleep nano.
pub fn nsleep(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let time = unsafe { a.get_unchecked(0) }
        .as_u64()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    std::thread::sleep(std::time::Duration::from_nanos(time));
    Ok(())
}

/// Yield this thread.
pub fn yield_now(_: &mut [Var]) -> Result<(), anyhow::Error> {
    std::thread::yield_now();
    Ok(())
}

/// Count CPUs.
pub fn cpu_count(a: &mut [Var]) -> Result<(), anyhow::Error> {
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(num_cpus::get_physical() as u64);
    Ok(())
}

/// Count threads.
pub fn par_count(a: &mut [Var]) -> Result<(), anyhow::Error> {
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(num_cpus::get() as u64);
    Ok(())
}

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::thrd::tls::get", tls_get);
    putnfp("raw::thrd::tls::set", tls_set);
    putnfp("raw::thrd::tls::del", tls_del);
    putnfp("raw::thrd::spawn", spawn);
    putnfp("raw::thrd::join", join);
    putnfp("raw::thrd::sleep<msec>", msleep);
    putnfp("raw::thrd::sleep<nsec>", nsleep);
    putnfp("raw::thrd::sleep<sec>", sleep);
    putnfp("raw::thrd::yield", yield_now);
    putnfp("raw::thrd::par_count", par_count);
    putnfp("raw::thrd::cpu_count", cpu_count);
}
