//!
//! Local filesystem I/O module of `libraw`.
//!

use crate::{
    context::putnfp,
    libraw::iohmgr::{self, error as io_error, RawObject},
    vmem::Var,
};
use anyhow::anyhow;
use std::io::Read;

macro_rules! impl_remove {
    ($a: ident) => {
        pub fn $a(a: &mut [Var]) -> Result<(), anyhow::Error> {
            let path = unsafe { a.get_unchecked(0) }
                .as_sr()
                .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
            let path = path.borrow()?;
            match std::fs::$a(&path[..]) {
                Ok(_) => *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(0),
                Err(x) => *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::from(x.kind())),
            }
            Ok(())
        }
    };
}

/// Read full content to a bytes.
pub fn read_to_bytes(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    match iohmgr::get(id) {
        Some(x) => {
            let mut buf = Vec::new();
            match match &*x {
                RawObject::LocalFile(y) => y,
                _ => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::OTHER);
                    return Ok(());
                }
            }
            .read_to_end(&mut buf)
            {
                Ok(_) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(0);
                    *(unsafe { a.get_unchecked_mut(1) }) = Var::Bytes(buf.into());
                }
                Err(y) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::from(y.kind()));
                }
            }
        }
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::OTHER);
        }
    }
    Ok(())
}

/// Write bytes.
pub fn write_bytes(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use std::io::Write;

    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let content = match unsafe { a.get_unchecked(1) } {
        Var::Bytes(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    .clone();
    let content = &content.borrow()?[..];
    match match &iohmgr::get(id) {
        Some(x) => match &**x {
            RawObject::LocalFile(y) => y,
            _ => return Err(anyhow!("raw::fatal::segfault")),
        },
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::OTHER);
            return Ok(());
        }
    }
    .write_all(content)
    {
        Ok(_) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(0);
        }
        Err(y) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::from(y.kind()));
        }
    }
    Ok(())
}

/// Close a file.
pub fn close(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    iohmgr::del(id as u64);
    Ok(())
}

/// Open a file.
pub fn open(a: &mut [Var]) -> Result<(), anyhow::Error> {
    use std::fs::OpenOptions;

    let path = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw:fatal::not_a_buf"))?;
    let path = path.borrow()?;
    let mode = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw:fatal::not_a_buf"))?;
    let mode = mode.borrow()?;
    let mut options = &mut OpenOptions::new();
    for i in mode.as_bytes().iter() {
        if *i == b'r' {
            options = options.read(true);
        } else if *i == b'w' {
            options = options.write(true);
        } else if *i == b'a' {
            options = options.append(true);
        } else if *i == b'c' {
            options = options.create(true);
        } else if *i == b'n' {
            options = options.create_new(true);
        }
    }
    match options.open(&**path) {
        Ok(x) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(0);
            *(unsafe { a.get_unchecked_mut(1) }) = Var::U64(iohmgr::add(RawObject::LocalFile(x)));
        }
        Err(x) => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::from(x.kind()));
        }
    }
    Ok(())
}

/// Read content with specified length to a bytes.
pub fn read(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let bufc = unsafe { a.get_unchecked(1) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    match iohmgr::get(id) {
        Some(x) => {
            let mut buf = vec![0u8; bufc];
            match match &*x {
                RawObject::LocalFile(y) => y,
                _ => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::OTHER);
                    return Ok(());
                }
            }
            .read(&mut buf)
            {
                Ok(y) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(0);
                    *(unsafe { a.get_unchecked_mut(1) }) = Var::Bytes(buf.into());
                    *(unsafe { a.get_unchecked_mut(2) }) = Var::U64(y as u64);
                }
                Err(y) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::from(y.kind()));
                }
            }
        }
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::OTHER);
        }
    }
    Ok(())
}

/// Get file length.
pub fn len(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let id = unsafe { a.get_unchecked(0) }
        .as_u64_strict()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    match iohmgr::get(id) {
        Some(x) => {
            match match &*x {
                RawObject::LocalFile(y) => y,
                _ => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::OTHER);
                    return Ok(());
                }
            }
            .metadata()
            {
                Ok(y) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(0);
                    *(unsafe { a.get_unchecked_mut(1) }) = Var::U64(y.len());
                }
                Err(y) => {
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::from(y.kind()));
                }
            }
        }
        None => {
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U64(io_error::OTHER);
        }
    }
    Ok(())
}

impl_remove!(remove_file);
impl_remove!(remove_dir_all);
impl_remove!(remove_dir);
impl_remove!(create_dir);
impl_remove!(create_dir_all);

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::fs::open", open);
    putnfp("raw::fs::close", close);
    putnfp("raw::fs::len", len);
    putnfp("raw::fs::read_all", read_to_bytes);
    putnfp("raw::fs::write_all", write_bytes);
    putnfp("raw::fs::read", read);
    putnfp("raw::fs::remove", remove_file);
    putnfp("raw::fs::remove_tree", remove_dir_all);
    putnfp("raw::fs::remove_seed", remove_dir);
    putnfp("raw::fs::mkseed", create_dir);
    putnfp("raw::fs::mktree", create_dir_all);
}
