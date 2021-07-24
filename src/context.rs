//!
//! Context of a VM instance. This contains structures and global status about functions,
//! interruptions, static variables, threads, coroutines, etc.
//!

use crate::{
    executor,
    isa::{FuncPtr, Instruction, InterruptHandler, NativeFuncPtr, VirtFuncPtr},
    vmem::Var,
};
use anyhow::anyhow;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{cell::RefCell, collections::HashMap};

/// Collection of functions. The performance of calling a function is important, so this have
/// a thread-local cache.
static FUNCTIONS: Lazy<DashMap<Box<str>, FuncPtr, ahash::RandomState>> =
    Lazy::new(|| DashMap::with_capacity_and_hasher(256, ahash::RandomState::default()));
/// Handlers when interrupted. If the handler is not set, the executing engine will `Ignore` by
/// default. Interruptions are cold, so this don't have a thread-local cache.
static INTERRUPTIONS: Lazy<DashMap<Box<str>, InterruptHandler, ahash::RandomState>> =
    Lazy::new(|| DashMap::with_capacity_and_hasher(16, ahash::RandomState::default()));
/// Static variable storage. Generally, the variables are used to be shared between threads. So
/// this don't have a thread-local cache.
static VM_STATIC: Lazy<DashMap<Box<str>, Var, ahash::RandomState>> =
    Lazy::new(|| DashMap::with_capacity_and_hasher(16, ahash::RandomState::default()));

std::thread_local! {
    /// Thread-local cache of `FUNCTIONS`.
    static FUNCTIONS_CACHE: RefCell<HashMap<Box<str>, FuncPtr, ahash::RandomState>> = RefCell::new(sync_cache());
}

#[inline(always)]
fn sync_cache() -> HashMap<Box<str>, FuncPtr, ahash::RandomState> {
    let mut result =
        HashMap::with_capacity_and_hasher(FUNCTIONS.capacity(), ahash::RandomState::default());
    for i in FUNCTIONS.iter() {
        result.insert(i.key().to_owned(), i.value().to_owned());
    }
    result
}

#[inline(always)]
pub fn force_sync_cache() {
    FUNCTIONS_CACHE.with(|x| *x.borrow_mut() = sync_cache());
}

/// Context dump.
#[inline(always)]
pub fn dump() -> Box<str> {
    format!(
        "Context {{ Functions {:?}, Interruptions {:?}, BSS {:?} }}",
        *FUNCTIONS, *INTERRUPTIONS, *VM_STATIC
    )
    .into_boxed_str()
}

/// Get a function pointer.
#[inline(always)]
pub fn getfp(name: &str) -> Option<FuncPtr> {
    FUNCTIONS_CACHE.with(|cache| {
        let borrow = cache.borrow();
        match borrow.get(name) {
            Some(x) => Some(x.to_owned()),
            None => FUNCTIONS.get(name).map(|x| {
                drop(borrow);
                *cache.borrow_mut() = sync_cache();
                x.to_owned()
            }),
        }
    })
}
/// Put a function pointer. There won't update the thread-local function cache.
#[inline(always)]
pub fn putfp(name: &str, fp: FuncPtr) {
    FUNCTIONS.insert(Box::from(name), fp);
}
/// Put a native function.
#[inline(always)]
pub fn putnfp(name: &str, fp: NativeFuncPtr) {
    putfp(name, FuncPtr::Native(fp))
}
/// Put a virtual function.
#[inline(always)]
pub fn putvfp(name: &str, fp: VirtFuncPtr) {
    putfp(name, FuncPtr::Virtual(fp))
}
/// Set a static.
#[inline(always)]
pub fn putstatic(name: &str, val: Var) {
    VM_STATIC.insert(Box::from(name), val);
}
/// Get a static.
#[inline(always)]
pub fn getstatic(name: &str) -> Option<Var> {
    VM_STATIC.get(name).map(|x| x.to_owned())
}

/// Make an interruption abort.
#[inline(always)]
pub fn intabort(name: &str, msg: Option<String>) {
    INTERRUPTIONS.insert(Box::from(name), InterruptHandler::Abort(msg));
}
/// Make an interruption ignored.
#[inline(always)]
pub fn intignore(name: &str) {
    INTERRUPTIONS.insert(Box::from(name), InterruptHandler::Ignore);
}
/// Make an interruption catched.
#[inline(always)]
pub fn intcatch(name: &str, fp: VirtFuncPtr) {
    INTERRUPTIONS.insert(Box::from(name), InterruptHandler::Handler(fp));
}
/// Perform an interruption. This will return if the handler is `Ignore`, or never return if the
/// handler is `Abort`.
pub fn int(name: &str) -> bool {
    if let Some(x) = INTERRUPTIONS.get(name) {
        match &*x {
            InterruptHandler::Abort(y) => {
                use ansi_term::{Color::Red, Style};

                eprintln!(
                    "{}",
                    Style::new().underline().fg(Red).paint(" !!! ABORTED !!! ")
                );
                eprintln!("This unit aborted due to an fatal interruption.");
                eprintln!("Interruption: {}", Red.paint(name));
                if let Some(z) = y {
                    eprintln!("     Message: {}\n", z);
                }
                eprint!("  World View: ");

                return true;
            }
            InterruptHandler::Handler(y) => {
                executor::start(Thread::new(y.clone()));
            }
            _ => (),
        }
    }
    false
}

/// A function's context. This is private, because all operations should be through `Thread`, not
/// using `FnContext` directly.
#[derive(Debug)]
struct FnContext {
    /// The function content.
    fp: VirtFuncPtr,
    /// Count of next instruction.
    fc: usize,
    /// Function-specified SIL. The size should be 100.
    sil: Box<[Var]>,
}
impl FnContext {
    /// Create a new `FnContext` with specified function pointer.
    #[inline(always)]
    fn new(fp: VirtFuncPtr) -> Self {
        Self {
            fp,
            fc: 0,
            sil: vec![Var::U8(0); 100].into_boxed_slice(),
        }
    }
    /// Get next instruction.
    #[allow(clippy::should_implement_trait)]
    #[inline(always)]
    fn next(&mut self) -> Option<&Instruction> {
        self.fc += 1;
        self.fp.get(self.fc - 1)
    }
    /// Set FC to specified value.
    #[inline(always)]
    fn jmp(&mut self, t: usize) {
        self.fc = t;
    }
}

pub trait ExecUnit {
    /// Get the next instruction.
    fn next(&mut self) -> Option<Instruction>;

    /// Perform `JMP` operation.
    fn jmp(&mut self, _: usize);

    /// Call another function. Fail if a native function is called and an interruption was thrown.
    fn call(&mut self, _: FuncPtr) -> Result<(), anyhow::Error>;

    /// Make this function return. Returns false if it fails.
    fn ret(&mut self) -> bool;

    /// Get a value from SIL.
    fn sget(&self, _: usize) -> Result<&Var, anyhow::Error>;

    /// Set a value to SIL.
    fn sset(&mut self, _: usize, _: Var) -> Result<(), anyhow::Error>;
}

/// A thread's context that implements `ExecUnit`.
#[derive(Debug)]
pub struct Thread {
    /// The current function executing.
    current: Option<FnContext>,
    /// Call stack.
    callstack: Vec<FnContext>,
    /// Top of SIL for argument passing. The size should be 50.
    topsil: Box<[Var]>,
}
impl Thread {
    /// Create a new `Thread` with specified function pointer.
    #[inline(always)]
    pub fn new(fp: VirtFuncPtr) -> Self {
        Self {
            current: Some(FnContext::new(fp)),
            callstack: Vec::with_capacity(12),
            topsil: vec![Var::U8(0); 50].into_boxed_slice(),
        }
    }
}
impl ExecUnit for Thread {
    #[allow(clippy::should_implement_trait)]
    #[inline(always)]
    fn next(&mut self) -> Option<Instruction> {
        self.current.as_mut().unwrap().next().cloned()
    }
    #[inline(always)]
    fn jmp(&mut self, t: usize) {
        self.current.as_mut().unwrap().jmp(t)
    }
    #[inline]
    fn call(&mut self, fp: FuncPtr) -> Result<(), anyhow::Error> {
        match fp {
            FuncPtr::Virtual(x) => {
                let swps = self.current.take().unwrap();
                self.callstack.push(swps);
                self.current = Some(FnContext::new(x));
                Ok(())
            }
            FuncPtr::Native(x) => x(&mut *self.topsil),
        }
    }
    #[inline]
    #[must_use]
    fn ret(&mut self) -> bool {
        match self.callstack.pop() {
            Some(x) => {
                self.current = Some(x);
                true
            }
            None => {
                self.current = None;
                false
            }
        }
    }
    #[inline]
    fn sget(&self, addr: usize) -> Result<&Var, anyhow::Error> {
        if (0..100).contains(&addr) {
            Ok(unsafe { self.current.as_ref().unwrap().sil.get_unchecked(addr) })
        } else if (100..150).contains(&addr) {
            Ok(unsafe { self.topsil.get_unchecked(addr - 100) })
        } else {
            Err(anyhow!("raw::fatal::segfault"))
        }
    }
    #[inline]
    fn sset(&mut self, addr: usize, val: Var) -> Result<(), anyhow::Error> {
        if (0..100).contains(&addr) {
            unsafe {
                *self.current.as_mut().unwrap().sil.get_unchecked_mut(addr) = val;
            }
            Ok(())
        } else if (100..150).contains(&addr) {
            unsafe {
                *self.topsil.get_unchecked_mut(addr - 100) = val;
            }
            Ok(())
        } else {
            Err(anyhow!("raw::fatal::segfault"))
        }
    }
}
