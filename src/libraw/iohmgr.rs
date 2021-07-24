//!
//! IO Handler Manager for `libraw`. This provides utilities about using and generating IO
//! handlers. This is for all `libraw`'s IO modules: local filesystem, socket, etc.
//!

use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::{const_mutex, Mutex};
use smallvec::SmallVec;
use std::{
    collections::HashSet,
    hash::{BuildHasher, Hasher},
    ops::Deref,
};

/// Error handling module.
pub mod error {
    use std::io::ErrorKind::*;

    pub const NOT_FOUND: u64 = 1;
    pub const PERMISSION_DENIED: u64 = 2;
    pub const CONNECTION_REFUSED: u64 = 3;
    pub const CONNECTION_RESET: u64 = 4;
    pub const CONNECTION_ABORTED: u64 = 5;
    pub const NOT_CONNECTED: u64 = 6;
    pub const ADDR_IN_USE: u64 = 7;
    pub const ADDR_NOT_AVAILABLE: u64 = 8;
    pub const BROKEN_PIPE: u64 = 9;
    pub const ALREADY_EXISTS: u64 = 10;
    pub const WOULD_BLOCK: u64 = 11;
    pub const INVALID_INPUT: u64 = 12;
    pub const INVALID_DATA: u64 = 13;
    pub const TIMED_OUT: u64 = 14;
    pub const WRITE_ZERO: u64 = 15;
    pub const INTERRUPTED: u64 = 16;
    pub const UNEXPECTED_EOF: u64 = 17;
    pub const OTHER: u64 = 18;

    /// Convert an `error`(u64) to a displayable string.
    pub fn to_string(a: u64) -> &'static str {
        match a {
            0 => "operation succeed",
            NOT_FOUND => "file not found",
            PERMISSION_DENIED => "permission denied",
            CONNECTION_REFUSED => "connection refused",
            CONNECTION_RESET => "connection reset",
            CONNECTION_ABORTED => "connection aborted",
            NOT_CONNECTED => "not connected",
            ADDR_IN_USE => "address is using",
            ADDR_NOT_AVAILABLE => "address is unavailable",
            BROKEN_PIPE => "pipe is broken",
            ALREADY_EXISTS => "file already exists",
            WOULD_BLOCK => "operation would block",
            INVALID_INPUT => "input is invalid",
            INVALID_DATA => "data is invalid",
            TIMED_OUT => "operation timed out",
            WRITE_ZERO => "zero is written",
            INTERRUPTED => "operation is interrupted",
            UNEXPECTED_EOF => "early-eof is detected",
            _ => "unknown error",
        }
    }

    /// Parse an `error`(u64) from an `std::io::Error`.
    pub fn from(e: std::io::ErrorKind) -> u64 {
        match e {
            NotFound => NOT_FOUND,
            PermissionDenied => PERMISSION_DENIED,
            ConnectionRefused => CONNECTION_REFUSED,
            ConnectionReset => CONNECTION_RESET,
            ConnectionAborted => CONNECTION_ABORTED,
            NotConnected => NOT_CONNECTED,
            AddrInUse => ADDR_IN_USE,
            AddrNotAvailable => ADDR_NOT_AVAILABLE,
            BrokenPipe => BROKEN_PIPE,
            AlreadyExists => ALREADY_EXISTS,
            WouldBlock => WOULD_BLOCK,
            InvalidInput => INVALID_INPUT,
            InvalidData => INVALID_DATA,
            TimedOut => TIMED_OUT,
            WriteZero => WRITE_ZERO,
            Interrupted => INTERRUPTED,
            UnexpectedEof => UNEXPECTED_EOF,
            _ => OTHER,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FakeHasher(SmallVec<[u8; 8]>);
impl Default for FakeHasher {
    fn default() -> Self {
        Self(SmallVec::new())
    }
}
impl BuildHasher for FakeHasher {
    type Hasher = FakeHasher;

    fn build_hasher(&self) -> FakeHasher {
        self.to_owned()
    }
}
impl Hasher for FakeHasher {
    fn write(&mut self, a: &[u8]) {
        if self.0.len() + a.len() > 8 {
            panic!("Invalid usage of FakeHasher");
        }
        a.iter().for_each(|x| self.0.push(*x));
    }
    fn finish(&self) -> u64 {
        let mut buf = [0u8; 8];
        let mut cur = 0;
        self.0.iter().for_each(|x| unsafe {
            *buf.get_unchecked_mut(cur) = *x;
            cur += 1;
        });
        u64::from_ne_bytes(buf)
    }
}

/// Global I/O handler ID generator.
static GLOBAL_IDGEN: Mutex<IdGen> = const_mutex(IdGen::new());
/// Object map.
static OBJECTS: Lazy<DashMap<u64, RawObject, FakeHasher>> = Lazy::new(DashMap::default);

/// ID generator.
#[derive(Debug)]
pub struct IdGen {
    top_id: u64,
    freed_id: Vec<u64>,
}
impl Default for IdGen {
    fn default() -> Self {
        Self::new()
    }
}
impl IdGen {
    /// Create a new ID generator.
    #[inline]
    pub const fn new() -> Self {
        Self {
            top_id: 0,
            freed_id: Vec::new(),
        }
    }
    /// Get next number.
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn next(&mut self) -> u64 {
        if let Some(x) = self.freed_id.pop() {
            return x;
        }
        self.top_id += 1;
        self.top_id - 1
    }
    /// Free a number.
    #[inline]
    pub fn free(&mut self, id: u64) {
        self.freed_id.push(id);
    }
}

/// ID generator for complex environment. This is slower, but more unique than `IdGen`.
/// This won't reuse freed IDs immediately.
#[derive(Debug)]
pub struct CeIdGen {
    top_id: u64,
    used_id: HashSet<u64, FakeHasher>,
}
impl Default for CeIdGen {
    #[inline]
    fn default() -> Self {
        Self {
            top_id: 0,
            used_id: HashSet::default(),
        }
    }
}
impl CeIdGen {
    /// Create a new `CeIdGen`.
    #[inline]
    pub fn new() -> Self {
        Self {
            top_id: 0,
            used_id: HashSet::default(),
        }
    }
    /// Get next number.
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn next(&mut self) -> u64 {
        loop {
            if self.used_id.contains(&self.top_id) {
                match self.top_id.checked_add(1) {
                    Some(x) => self.top_id = x,
                    None => self.top_id = 0,
                };
                continue;
            } else {
                match self.top_id.checked_add(1) {
                    Some(x) => self.top_id = x,
                    None => {
                        self.top_id = 0;
                        continue;
                    }
                };
                break self.top_id - 1;
            }
        }
    }
    /// Free a number.
    #[inline]
    pub fn free(&mut self, id: u64) {
        self.used_id.remove(&id);
    }
}

pub enum RawObject {
    LocalFile(std::fs::File),
    Thread(std::thread::JoinHandle<()>),
}

/// Open a handler.
pub fn add(obj: RawObject) -> u64 {
    let id = GLOBAL_IDGEN.lock().next();
    OBJECTS.insert(id, obj);
    id
}

/// Close a handler.
#[inline]
pub fn del(id: u64) {
    take(id);
}

/// Get a handler.
#[inline]
pub fn get(id: u64) -> Option<impl Deref<Target = RawObject>> {
    OBJECTS.get(&id)
}

/// Take a handler.
#[inline]
pub fn take(id: u64) -> Option<RawObject> {
    match OBJECTS.remove(&id) {
        Some((_, x)) => {
            GLOBAL_IDGEN.lock().free(id);
            Some(x)
        }
        None => None,
    }
}
