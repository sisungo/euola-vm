//!
//! Data structures about VM variables, such as `Var`, and references.
//!

use crate::{
    context::{getfp, ExecUnit, Thread},
    executor::{self, start},
    isa::FuncPtr,
};
use anyhow::anyhow;
use dashmap::DashMap;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    sync::Arc,
};

/// A helper trait to create null references and judge if the reference is null.
pub trait CreateNull {
    /// Create a null references.
    fn null() -> Self;
    /// Judge if this reference is null.
    fn is_null(&self) -> bool;
}
macro_rules! impl_create_null {
    ($n: ident) => {
        impl CreateNull for $n {
            #[inline]
            fn null() -> Self {
                Self(None)
            }
            #[inline]
            fn is_null(&self) -> bool {
                self.0.is_none()
            }
        }
    };
}
macro_rules! impl_veclike {
    ($a: ty, $b: ty, $c: expr) => {
        impl $a {
            /// # Safety
            /// This function is unsafe because it calls get_unchecked. However, it returns `Option<u8>`
            /// not `u8` because it checks if self is NULL.
            #[inline]
            pub unsafe fn get_unchecked(&self, a: usize) -> Option<$b> {
                self.0
                    .as_ref()
                    .map(|x| x.read().get_unchecked(a).to_owned())
            }
            /// # Safety
            /// This is unsafe with the same reason to `BytesRef::get_unchecked`.
            #[inline]
            pub unsafe fn set_unchecked(&self, a: usize, b: $b) -> Result<(), anyhow::Error> {
                match &self.0 {
                    Some(x) => {
                        *x.write().get_unchecked_mut(a) = b;
                        Ok(())
                    }
                    None => Err(anyhow!("raw::fatal::argument_null")),
                }
            }
            /// Set a value.
            #[inline]
            pub fn set(&self, a: usize, b: $b) -> Result<(), anyhow::Error> {
                match &self.0 {
                    Some(x) => match x.write().get_mut(a) {
                        Some(y) => {
                            *y = b;
                            Ok(())
                        }
                        None => Err(anyhow!("raw::fatal::out_of_range")),
                    },
                    None => Err(anyhow!("raw::fatal::argument_null")),
                }
            }
            /// Get a value.
            #[inline]
            pub fn get(&self, a: usize) -> Result<$b, anyhow::Error> {
                match &self.0 {
                    Some(x) => match x.read().get(a) {
                        Some(y) => Ok(y.to_owned()),
                        None => Err(anyhow!("raw::fatal::out_of_range")),
                    },
                    None => Err(anyhow!("raw::fatal::argument_null")),
                }
            }
            /// Resize the collection.
            #[inline]
            pub fn resize(&self, s: usize) -> Result<(), anyhow::Error> {
                self.0
                    .as_ref()
                    .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
                    .write()
                    .resize(s, $c);
                Ok(())
            }
            /// Push a value to the end of the collection.
            #[inline]
            pub fn push(&self, s: $b) -> Result<(), anyhow::Error> {
                self.0
                    .as_ref()
                    .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
                    .write()
                    .push(s);
                Ok(())
            }
            /// Clear the collection.
            #[inline]
            pub fn clear(&self) -> Result<(), anyhow::Error> {
                self.0
                    .as_ref()
                    .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
                    .write()
                    .clear();
                Ok(())
            }
            /// Get the length of the collection.
            #[inline]
            pub fn len(&self) -> Result<usize, anyhow::Error> {
                Ok(self
                    .0
                    .as_ref()
                    .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
                    .read()
                    .len())
            }
            /// Returns `Ok(true)` if this collection is empty, or `Ok(false)` for not empty, or
            /// `Err(_)` if any error are detected.
            #[inline]
            pub fn is_empty(&self) -> Result<bool, anyhow::Error> {
                if self.len()? == 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    };
}
macro_rules! impl_var_as {
    ($a: ident, $b: ident) => {
        pub fn $a(&self) -> Option<$b> {
            match self {
                Var::I8(x) => Some(*x as $b),
                Var::U8(x) => Some(*x as $b),
                Var::I16(x) => Some(*x as $b),
                Var::U16(x) => Some(*x as $b),
                Var::I32(x) => Some(*x as $b),
                Var::U32(x) => Some(*x as $b),
                Var::I64(x) => Some(*x as $b),
                Var::U64(x) => Some(*x as $b),
                Var::Usize(x) => Some(*x as $b),
                _ => None,
            }
        }
    };
}

/// A reference to a VM object.
#[derive(Debug, Clone)]
pub struct ObjectRef(Option<Arc<DashMap<Box<str>, Var, ahash::RandomState>>>);
impl_create_null!(ObjectRef);
impl ObjectRef {
    /// Create a new non-null object with specified type.
    #[inline]
    pub fn new(t: &str) -> Self {
        let unique_obj = DashMap::with_capacity_and_hasher(8, ahash::RandomState::default());
        unique_obj.insert(Box::from("type"), Var::UString(StringRef::from(t)));
        Self(Some(Arc::new(unique_obj)))
    }
    #[inline]
    pub fn get(&self, id: &str) -> Result<Var, anyhow::Error> {
        match self
            .0
            .as_ref()
            .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
            .get(id)
        {
            Some(x) => Ok(x.to_owned()),
            None => Err(anyhow!("raw::fatal::out_of_range")),
        }
    }
    #[inline]
    pub fn set(&self, id: &str, val: Var) -> Result<(), anyhow::Error> {
        self.0
            .as_ref()
            .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
            .insert(Box::from(id), val);
        Ok(())
    }
}
impl PartialEq for ObjectRef {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            Some(x) => match &other.0 {
                Some(y) => {
                    if x.get("type").map(|b| b.as_sr()) == y.get("type").map(|b| b.as_sr()) {
                        if let Some(z) = x.get("eq") {
                            let z = match &*z {
                                Var::UString(a) => a,
                                _ => return false,
                            };
                            let z = match z.borrow() {
                                Ok(a) => a,
                                Err(_) => return false,
                            };
                            let fp = match getfp(&z[..]) {
                                Some(FuncPtr::Virtual(c)) => c,
                                _ => return false,
                            };
                            let mut t = Thread::new(fp);
                            t.sset(100, Var::Object(self.clone())).unwrap();
                            t.sset(101, Var::Object(other.clone())).unwrap();
                            executor::start_noo(&mut t);
                            if let Some(d) = t.sget(100).unwrap().as_u8() {
                                return d != 0;
                            }
                        }
                        false
                    } else {
                        false
                    }
                }
                None => false,
            },
            None => other.0.is_none(),
        }
    }
}
impl Eq for ObjectRef {}
impl Hash for ObjectRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(x) = &self.0 {
            if let Some(y) = x.get("hash") {
                let y = match &*y {
                    Var::UString(a) => a,
                    _ => return,
                };
                let y = match y.borrow() {
                    Ok(a) => a,
                    Err(_) => return,
                };
                let fp = match getfp(&y[..]) {
                    Some(FuncPtr::Virtual(c)) => c,
                    _ => return,
                };
                let mut t = Thread::new(fp);
                t.sset(100, Var::Object(self.clone())).unwrap();
                executor::start_noo(&mut t);
                if let Ok(Var::Bytes(z)) = t.sget(100) {
                    z.hash(state);
                }
            }
        }
    }
}

impl Drop for ObjectRef {
    fn drop(&mut self) {
        if let Ok(x) = self.get("finalize") {
            if Arc::strong_count(self.0.as_ref().unwrap()) > 1 {
                return;
            }
            let x = match x.as_sr() {
                Some(y) => y,
                None => return,
            };
            let x = match x.borrow() {
                Ok(y) => y,
                Err(_) => return,
            };
            let x = match getfp(&x[..]) {
                Some(y) => y,
                None => return,
            };
            self.0.as_ref().unwrap().remove("finalize");
            match x {
                FuncPtr::Virtual(y) => {
                    let mut t = Thread::new(y);
                    t.sset(100, Var::Object(self.clone())).unwrap();
                    start(t);
                }
                FuncPtr::Native(y) => {
                    // NOTE: This will be recursive called, so there shouldn't store too much on
                    // the stack to avoid stack overflowing. That's why there should use `Vec`, not
                    // array or `SmallVec` although the length of `a` is fixed.
                    let mut a = vec![Var::U8(0); 50];
                    *(unsafe { a.get_unchecked_mut(0) }) = Var::Object(self.clone());
                    y(&mut a[..]).ok();
                }
            }
        }
    }
}

/// A reference to a VM vector.
#[derive(Debug, Clone)]
pub struct VectorRef(Option<Arc<RwLock<Vec<Var>>>>);
impl_create_null!(VectorRef);
impl From<&BytesRef> for VectorRef {
    #[inline]
    fn from(r: &BytesRef) -> Self {
        match &r.0 {
            Some(x) => {
                let inner = RwLock::new(Vec::with_capacity(x.read().len()));
                x.read()
                    .iter()
                    .for_each(|y| inner.write().push(Var::U8(*y)));
                Self(Some(Arc::new(inner)))
            }
            None => Self(None),
        }
    }
}
impl From<Vec<Var>> for VectorRef {
    #[inline]
    fn from(v: Vec<Var>) -> Self {
        Self(Some(Arc::new(RwLock::new(v))))
    }
}
impl Eq for VectorRef {}
impl From<RwLock<Vec<Var>>> for VectorRef {
    #[inline]
    fn from(v: RwLock<Vec<Var>>) -> Self {
        Self(Some(Arc::new(v)))
    }
}
impl_veclike!(VectorRef, Var, Var::U8(0));
impl VectorRef {
    #[inline]
    pub fn empty() -> Self {
        Self(Some(Arc::new(RwLock::new(Vec::with_capacity(4)))))
    }
    /// Borrow this reference.
    #[inline]
    pub fn borrow(&self) -> Result<RwLockReadGuard<Vec<Var>>, anyhow::Error> {
        match &self.0 {
            Some(x) => Ok(x.read()),
            None => Err(anyhow!("raw::fatal::argument_null")),
        }
    }
    /// Borrow this reference as mut.
    #[inline]
    pub fn borrow_mut(&self) -> Result<RwLockWriteGuard<Vec<Var>>, anyhow::Error> {
        match &self.0 {
            Some(x) => Ok(x.write()),
            None => Err(anyhow!("raw::fatal::argument_null")),
        }
    }
}
impl PartialEq for VectorRef {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            Some(x) => match &other.0 {
                Some(y) => *x.read() == *y.read(),
                None => false,
            },
            None => other.0.is_none(),
        }
    }
}
impl Hash for VectorRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(x) = &self.0 {
            x.read().hash(state);
        }
    }
}

/// A reference to a VM bytes vector.
#[derive(Debug, Clone)]
pub struct BytesRef(pub Option<Arc<RwLock<Vec<u8>>>>);
impl_create_null!(BytesRef);
impl PartialEq for BytesRef {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            Some(x) => match &other.0 {
                Some(y) => *x.read() == *y.read(),
                None => false,
            },
            None => other.0.is_none(),
        }
    }
}
impl Eq for BytesRef {}
impl Hash for BytesRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(x) = &self.0 {
            x.read().hash(state);
        }
    }
}
impl From<&str> for BytesRef {
    #[inline]
    fn from(s: &str) -> Self {
        Self(Some(Arc::new(RwLock::new(Vec::from(s)))))
    }
}
impl From<String> for BytesRef {
    #[inline]
    fn from(s: String) -> Self {
        Self(Some(Arc::new(RwLock::new(s.into()))))
    }
}
impl From<&StringRef> for BytesRef {
    #[inline]
    fn from(s: &StringRef) -> Self {
        match &s.0 {
            Some(x) => Self(Some(Arc::new(RwLock::new(x.read().to_owned().into())))),
            None => Self(None),
        }
    }
}
impl From<Vec<u8>> for BytesRef {
    #[inline]
    fn from(b: Vec<u8>) -> Self {
        Self(Some(Arc::new(RwLock::new(b))))
    }
}
impl Default for BytesRef {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl BytesRef {
    /// Create an empty one.
    #[inline]
    pub fn empty() -> Self {
        Self(Some(Arc::new(RwLock::new(Vec::with_capacity(16)))))
    }
    /// Borrow this reference.
    #[inline]
    pub fn borrow(&self) -> Result<RwLockReadGuard<Vec<u8>>, anyhow::Error> {
        match &self.0 {
            Some(x) => Ok(x.read()),
            None => Err(anyhow!("raw::fatal::argument_null")),
        }
    }
    /// Borrow this reference as mut.
    #[inline]
    pub fn borrow_mut(&self) -> Result<RwLockWriteGuard<Vec<u8>>, anyhow::Error> {
        match &self.0 {
            Some(x) => Ok(x.write()),
            None => Err(anyhow!("raw::fatal::argument_null")),
        }
    }
}
impl_veclike!(BytesRef, u8, 0);

/// A reference to a VM UTF-8 String(UString).
#[derive(Debug, Clone)]
pub struct StringRef(pub Option<Arc<RwLock<String>>>);
impl From<&str> for StringRef {
    #[inline]
    fn from(s: &str) -> Self {
        Self(Some(Arc::new(RwLock::new(s.to_owned()))))
    }
}
impl Eq for StringRef {}
impl PartialEq for StringRef {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            Some(x) => match &other.0 {
                Some(y) => *x.read() == *y.read(),
                None => false,
            },
            None => other.0.is_none(),
        }
    }
}
impl Hash for StringRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(x) = &self.0 {
            x.read().hash(state);
        }
    }
}
impl From<String> for StringRef {
    #[inline]
    fn from(s: String) -> Self {
        Self(Some(Arc::new(RwLock::new(s))))
    }
}
impl From<RwLock<String>> for StringRef {
    #[inline]
    fn from(s: RwLock<String>) -> Self {
        Self(Some(Arc::new(s)))
    }
}
impl TryFrom<&BytesRef> for StringRef {
    type Error = anyhow::Error;

    #[inline]
    fn try_from(b: &BytesRef) -> Result<Self, anyhow::Error> {
        Ok(Self(Some(Arc::new(RwLock::new(match String::from_utf8(
            b.0.as_ref()
                .ok_or_else(|| anyhow!("raw::fatal::argument_null"))?
                .read()
                .to_owned(),
        ) {
            Ok(x) => Ok(x),
            Err(_) => Err(anyhow!("raw::fatal::not_valid_utf8")),
        }?)))))
    }
}
impl Display for StringRef {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(x) => write!(f, "{}", *x.read()),
            None => write!(f, "(null)"),
        }
    }
}
impl Default for StringRef {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl_create_null!(StringRef);
impl StringRef {
    /// Create an empty `StringRef`.
    #[inline]
    pub fn empty() -> Self {
        Self(Some(Arc::new(RwLock::new(String::new()))))
    }
    /// Borrow this reference.
    #[inline]
    pub fn borrow(&self) -> Result<RwLockReadGuard<String>, anyhow::Error> {
        match &self.0 {
            Some(x) => Ok(x.read()),
            None => Err(anyhow!("raw::fatal::argument_null")),
        }
    }
    /// Borrow this reference as a mutable reference.
    #[inline]
    pub fn borrow_mut(&self) -> Result<RwLockWriteGuard<String>, anyhow::Error> {
        match &self.0 {
            Some(x) => Ok(x.write()),
            None => Err(anyhow!("raw::fatal::argument_null")),
        }
    }
}

/// A value that is used in euolaVM.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Var {
    /// Primitive 8-bit signed integer.
    I8(i8),
    /// Primitive 8-bit unsigned integer.
    U8(u8),
    /// Primitive 16-bit signed integer.
    I16(i16),
    /// Primitive 16-bit unsigned integer.
    U16(u16),
    /// Primitive 32-bit signed integer.
    I32(i32),
    /// Primitive 32-bit unsigned integer.
    U32(u32),
    /// Primitive 64-bit signed integer.
    I64(i64),
    /// Primitive 64-bit unsigned integer.
    U64(u64),
    /// Primitive FFI-used integer that acts like a `pointer`.
    Usize(usize),
    /// A general type for storing lots of bytes.
    Bytes(BytesRef),
    /// A type for storing UTF-8 encoding strings.
    UString(StringRef),
    /// A general type for storing lots of `Var`s.
    Vector(VectorRef),
    /// A general type that organizes data as a `Key-Value Pair`.
    Object(ObjectRef),
}
impl Var {
    /// Get type ID of this value.
    #[inline]
    pub fn typeid(&self) -> Result<String, anyhow::Error> {
        Ok(match self {
            Self::I8(_) => "primitive::i8".to_owned(),
            Self::U8(_) => "primitive::u8".to_owned(),
            Self::I16(_) => "primitive::i16".to_owned(),
            Self::U16(_) => "primitive::u16".to_owned(),
            Self::I32(_) => "primitive::i32".to_owned(),
            Self::U32(_) => "primitive::u32".to_owned(),
            Self::I64(_) => "primitive::i64".to_owned(),
            Self::U64(_) => "primitive::u64".to_owned(),
            Self::Usize(_) => "primitive::ptr".to_owned(),
            Self::Bytes(_) => "raw::bytes".to_owned(),
            Self::UString(_) => "raw::string".to_owned(),
            Self::Vector(_) => "raw::vector".to_owned(),
            Self::Object(x) => match &x.0 {
                Some(y) => match y.get("type") {
                    Some(z) => match &*z {
                        Self::UString(a) => a.to_string(),
                        _ => return Err(anyhow!("raw::fatal::metadata_disorder")),
                    },
                    None => return Err(anyhow!("raw::fatal::metadata_disorder")),
                },
                None => "raw::null".to_owned(),
            },
        })
    }
    /// Unwrap to an ObjectRef.
    #[inline]
    pub fn as_objref(&self) -> Option<&ObjectRef> {
        match self {
            Self::Object(x) => Some(x),
            _ => None,
        }
    }
    impl_var_as!(as_usize, usize);
    /// Get by offset.
    #[inline]
    pub fn offset_get(&self, o: usize) -> Result<Var, anyhow::Error> {
        match self {
            Self::Bytes(x) => match x.get(o) {
                Ok(y) => Ok(Var::U8(y)),
                Err(y) => Err(y),
            },
            Self::Vector(x) => x.get(o),
            _ => Err(anyhow!("raw::fatal::not_a_raw_collection")),
        }
    }
    /// Set by offset.
    #[inline]
    pub fn offset_set(&self, o: usize, val: Var) -> Result<(), anyhow::Error> {
        match self {
            Self::Bytes(x) => x.set(
                o,
                match val {
                    Self::U8(y) => y,
                    Self::I8(y) => y as u8,
                    _ => return Err(anyhow!("raw::fatal::out_of_range")),
                },
            ),
            Self::Vector(x) => x.set(o, val),
            _ => Err(anyhow!("raw::fatal::not_a_raw_collection")),
        }
    }
    /// Get length. Only suitable for `Vector` or `Bytes`.
    #[inline]
    pub fn rcl(&self) -> Result<usize, anyhow::Error> {
        match self {
            Self::UString(x) => Ok(x.borrow()?.len()),
            Self::Bytes(x) => Ok(x.len()?),
            Self::Vector(x) => Ok(x.len()?),
            _ => Err(anyhow!("raw::fatal::not_a_raw_collection")),
        }
    }
    /// Convert to StringRef.
    #[inline]
    pub fn as_sr(&self) -> Option<StringRef> {
        match self {
            Self::UString(x) => Some(x.clone()),
            Self::Bytes(x) => match StringRef::try_from(x) {
                Ok(y) => Some(y),
                Err(_) => None,
            },
            _ => None,
        }
    }
    /// Not-Zero Judge.
    #[inline]
    pub fn is_not_zero(&self) -> Option<bool> {
        match self {
            Self::I8(x) => Some(*x != 0),
            Self::U8(x) => Some(*x != 0),
            Self::I16(x) => Some(*x != 0),
            Self::U16(x) => Some(*x != 0),
            Self::I32(x) => Some(*x != 0),
            Self::U32(x) => Some(*x != 0),
            Self::I64(x) => Some(*x != 0),
            Self::U64(x) => Some(*x != 0),
            _ => None,
        }
    }
    /// Judge if this is null.
    #[inline]
    pub fn is_null(&self) -> Result<bool, anyhow::Error> {
        match self {
            Self::UString(x) => Ok(x.is_null()),
            Self::Bytes(x) => Ok(x.is_null()),
            Self::Vector(x) => Ok(x.is_null()),
            Self::Object(x) => Ok(x.is_null()),
            Self::Usize(x) => Ok(*x == 0),
            _ => Err(anyhow!("raw::fatal::not_an_object")),
        }
    }
    impl_var_as!(as_f64, f64);
    /// Convert to I32.
    #[inline]
    pub fn as_i32(&self) -> Result<i32, anyhow::Error> {
        match self {
            Self::I8(x) => Ok(*x as i32),
            Self::U8(x) => Ok(*x as i32),
            Self::I16(x) => Ok(*x as i32),
            Self::U16(x) => Ok(*x as i32),
            Self::I32(x) => Ok(*x),
            Self::U32(x) => Ok(*x as i32),
            Self::I64(x) => Ok(*x as i32),
            Self::U64(x) => Ok(*x as i32),
            Self::Usize(x) => Ok(*x as i32),
            _ => Err(anyhow!("raw::fatal::not_an_integer")),
        }
    }
    /// Convert to strict u64.
    #[inline]
    pub fn as_u64_strict(&self) -> Option<u64> {
        match self {
            Self::I64(x) => Some(*x as u64),
            Self::U64(x) => Some(*x),
            _ => None,
        }
    }
    impl_var_as!(as_u8, u8);
    impl_var_as!(as_u16, u16);
    impl_var_as!(as_u64, u64);
    impl_var_as!(as_i64, i64);
    impl_var_as!(as_isize, isize);
}
