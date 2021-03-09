//!
//! Berkeley socket bindings in `libraw`. This provides middle-level bindings to
//! raw berkeley socket APIs.
//!

use crate::{
    context::putnfp,
    libraw::iohmgr::{self, RawObject},
    vmem::Var,
};
use anyhow::anyhow;
use std::net::SocketAddr;

/// Parse socket address.
pub fn parse_sockaddr(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let val = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    let val = val.borrow()?;
    match val.parse::<SocketAddr>() {
        Ok(x) => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(1);
            *a.get_unchecked_mut(1) = Var::U64(iohmgr::add(RawObject::SockAddr(x.into())));
        },
        Err(_) => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
        },
    }
    Ok(())
}

/// Drop socket address.
pub fn drop_addr(a: &mut [Var]) -> Result<(), anyhow::Error> {
    iohmgr::del(
        unsafe { a.get_unchecked(0) }
            .as_u64_strict()
            .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))?,
    );
    Ok(())
}

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::str::parse<sockaddr>", parse_sockaddr);
    putnfp("raw::sock::addr::from<str>", parse_sockaddr);
    putnfp("raw::sock::addr::drop", drop_addr);
}
