//!
//! FFI library that uses `libffi` of `libraw`.
//!

use crate::{
    context::putnfp,
    libraw::iohmgr::FakeHasher,
    vmem::{Var, VectorRef},
};
use anyhow::anyhow;
use libc::c_char;
use libffi::high::{call::Arg as FfiArg, CType, CodePtr};
use libloading::{library_filename, Library, Symbol};
use parking_lot::{const_rwlock, RwLock};
use std::{
    collections::HashMap,
    convert::TryFrom,
    ffi::{c_void, CStr, CString},
    ptr::NonNull,
};

/// A safe wrapper to `NonNull` that makes it possible to share between threads.
/// Designed for symbols.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct Sym(NonNull<u8>);
unsafe impl Send for Sym {}
unsafe impl Sync for Sym {}
impl From<Symbol<'_, *mut u8>> for Sym {
    fn from(p: Symbol<*mut u8>) -> Self {
        Self(unsafe { NonNull::new_unchecked(*p) })
    }
}

/// An owned(in Rust) C-FFI object.
#[derive(Debug, Clone)]
enum OwnedFfiObject {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    FfiStr(Option<CString>),
    Pointer(*mut u8),
}
impl TryFrom<&Var> for OwnedFfiObject {
    type Error = anyhow::Error;

    fn try_from(v: &Var) -> Result<Self, anyhow::Error> {
        match v {
            Var::Object(x) => match x
                .get("raw::cffi::type")?
                .as_u8()
                .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?
            {
                12 => match x
                    .get("val")?
                    .as_sr()
                    .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?
                    .borrow()
                {
                    Ok(y) => match CString::new(y.to_owned()) {
                        Ok(z) => Ok(OwnedFfiObject::FfiStr(Some(z))),
                        Err(_) => Ok(OwnedFfiObject::FfiStr(None)),
                    },
                    Err(_) => Ok(OwnedFfiObject::FfiStr(None)),
                },
                14 => Ok(OwnedFfiObject::Pointer(
                    x.get("val")?
                        .as_usize()
                        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?
                        as *mut u8,
                )),
                8 => Ok(OwnedFfiObject::I8(
                    x.get("val")?
                        .as_u8()
                        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?
                        as i8,
                )),
                16 => Ok(OwnedFfiObject::I16(
                    x.get("val")?
                        .as_u16()
                        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?
                        as i16,
                )),
                32 => Ok(OwnedFfiObject::I32(x.get("val")?.as_i32()?)),
                64 => Ok(OwnedFfiObject::I64(
                    x.get("val")?
                        .as_u64()
                        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?
                        as i64,
                )),
                _ => Err(anyhow!("raw::fatal::invalid")),
            },
            _ => Err(anyhow!("raw::fatal::not_an_object")),
        }
    }
}

/// A vector of libraries.
static LIBRARIES: RwLock<Vec<Library>> = const_rwlock(Vec::new());
/// A vector of symbols.
static SYMBOLS: RwLock<Vec<Sym>> = const_rwlock(Vec::new());

/// Open a dynamic library.
pub fn opendll(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let path = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    match unsafe { Library::new(&path.borrow()?[..]) } {
        Ok(x) => {
            let mut guard = LIBRARIES.write();
            let id = guard.len();
            *(unsafe { a.get_unchecked_mut(1) }) = Var::U64(id as u64);
            guard.push(x);
            *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
        }
        Err(_) => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
        },
    }
    Ok(())
}

/// Open a symbol.
pub fn opensym(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let libid = unsafe { a.get_unchecked(0) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))?;
    let symid = unsafe { a.get_unchecked(1) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    match LIBRARIES.read().get(libid) {
        Some(x) => match unsafe { x.get::<*mut u8>(symid.borrow()?.as_bytes()) } {
            Ok(x) => {
                let mut guard = SYMBOLS.write();
                let id = guard.len();
                *(unsafe { a.get_unchecked_mut(1) }) = Var::U64(id as u64);
                guard.push(x.into());
                *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(1);
            }
            Err(_) => unsafe {
                *a.get_unchecked_mut(0) = Var::U8(0);
            },
        },
        None => unsafe {
            *a.get_unchecked_mut(0) = Var::U8(0);
        },
    }
    Ok(())
}

/// Make library filename.
pub fn libpath(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let name = unsafe { a.get_unchecked(0) }
        .as_sr()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_buf"))?;
    *(unsafe { a.get_unchecked_mut(0) }) =
        Var::UString((&library_filename(&name.borrow()?[..]).to_string_lossy()[..]).into());
    Ok(())
}

/// Invoke a dynamic library.
pub fn invoke(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let symid = unsafe { a.get_unchecked(0) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))?;
    let args = match unsafe { a.get_unchecked(1) } {
        Var::Vector(x) => x,
        _ => return Err(anyhow!("raw::fatal::not_a_buf")),
    }
    .clone();
    let rettyp = unsafe { a.get_unchecked(2) }
        .as_u8()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    let sym = SYMBOLS
        .read()
        .get(symid)
        .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))?
        .0;
    match rettyp {
        8 => unsafe {
            *a.get_unchecked_mut(0) = Var::I8(_invoke(sym, args)?);
            Ok(())
        },
        12 => unsafe {
            *a.get_unchecked_mut(0) = _getvarbycstr(_invoke(sym, args)?);
            Ok(())
        },
        14 => unsafe {
            *a.get_unchecked_mut(0) = Var::Usize(_invoke::<usize>(sym, args)?);
            Ok(())
        },
        16 => unsafe {
            *a.get_unchecked_mut(0) = Var::I16(_invoke(sym, args)?);
            Ok(())
        },
        32 => unsafe {
            *a.get_unchecked_mut(0) = Var::I32(_invoke(sym, args)?);
            Ok(())
        },
        64 => unsafe {
            *a.get_unchecked_mut(0) = Var::I64(_invoke(sym, args)?);
            Ok(())
        },
        _ => Err(anyhow!("raw::fatal::invalid")),
    }
}

/// Calculate pointer offset.
pub fn ptroffset(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let rptr = unsafe { a.get_unchecked(0) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))? as *const u8;
    let offs = unsafe { a.get_unchecked(1) }
        .as_isize()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))?;
    *(unsafe { a.get_unchecked_mut(0) }) = Var::Usize(rptr.wrapping_offset(offs) as usize);
    Ok(())
}

/// Get value from pointer.
pub fn ptrget(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let rptr = unsafe { a.get_unchecked(0) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))? as *const u8;
    *(unsafe { a.get_unchecked_mut(0) }) = Var::U8(unsafe { *rptr });
    Ok(())
}

/// Set value to pointer.
pub fn ptrset(a: &mut [Var]) -> Result<(), anyhow::Error> {
    let rptr = unsafe { a.get_unchecked(0) }
        .as_usize()
        .ok_or_else(|| anyhow!("raw::fatal::not_a_ptr"))? as *mut u8;
    let val = unsafe { a.get_unchecked(1) }
        .as_u8()
        .ok_or_else(|| anyhow!("raw::fatal::not_an_integer"))?;
    unsafe { *rptr = val }
    Ok(())
}

/// Initialize the library.
#[inline(always)]
pub fn init() {
    putnfp("raw::cffi::openlib", opendll);
    putnfp("raw::cffi::getpath", libpath);
    putnfp("raw::cffi::opensym", opensym);
    putnfp("raw::cffi::invoke", invoke);
    putnfp("raw::cffi::ptr::offset", ptroffset);
    putnfp("raw::cffi::ptr::get", ptrget);
    putnfp("raw::cffi::ptr::set", ptrset);
}

/// Core of `raw::cffi::invoke`.
#[inline]
unsafe fn _invoke<T: CType>(fp: NonNull<u8>, args: VectorRef) -> Result<T, anyhow::Error> {
    let args = args.borrow()?;
    let mut owned_ffi_objs = Vec::with_capacity(args.len());
    for i in args.iter() {
        owned_ffi_objs.push(OwnedFfiObject::try_from(i)?);
    }
    let mut sptr: HashMap<usize, *const c_char, FakeHasher> = HashMap::default();
    let mut rargs = Vec::with_capacity(args.len());
    for (c, i) in owned_ffi_objs.iter().enumerate() {
        if let OwnedFfiObject::FfiStr(x) = i {
            sptr.insert(
                c,
                match x {
                    Some(y) => y.as_ptr(),
                    None => std::ptr::null(),
                },
            );
        }
    }
    for (c, i) in owned_ffi_objs.iter().enumerate() {
        rargs.push(match i {
            OwnedFfiObject::I8(x) => FfiArg::new(x),
            OwnedFfiObject::I16(x) => FfiArg::new(x),
            OwnedFfiObject::I32(x) => FfiArg::new(x),
            OwnedFfiObject::I64(x) => FfiArg::new(x),
            OwnedFfiObject::FfiStr(_) => FfiArg::new(sptr.get(&c).unwrap()),
            OwnedFfiObject::Pointer(x) => FfiArg::new(x),
        });
    }
    Ok(libffi::high::call::call(
        CodePtr::from_ptr(fp.as_ptr() as *const c_void),
        &rargs,
    ))
}

/// Copy the content of `FFIStr` into a VM Var.
#[inline]
unsafe fn _getvarbycstr(p: *const c_char) -> Var {
    Var::UString(CStr::from_ptr(p).to_string_lossy()[..].into())
}
