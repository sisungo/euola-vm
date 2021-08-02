//!
//! The raw bindings to native API.
//!
//! The core designing goal of `libraw` is to be minimal and powerful. `libraw` is cross-platform,
//! and it doesn't contain any features that don't support one of Windows or UNIX.
//!

pub mod bytes;
pub mod cio;
pub mod deque;
pub mod dl;
pub mod env;
pub mod floatpoint;
pub mod fs;
pub mod hashmap;
pub mod interruptions;
pub mod intvec;
pub mod iohmgr;
pub mod proc;
pub mod rng;
pub mod string;
pub mod thread;
pub mod time;
pub mod vector;

#[cfg(feature = "cffi")]
pub mod cffi;

/// Initialize the libraw library. This will register libraw functions to the function table and
/// initialize some runtime-known values.
#[inline(always)]
pub fn init() {
    init_functions();
}

/// Initialize the functions.
#[inline(always)]
fn init_functions() {
    cio::init();
    string::init();
    dl::init();
    env::init();
    proc::init();
    rng::init();
    vector::init();
    fs::init();
    floatpoint::init();
    thread::init();
    hashmap::init();
    deque::init();
    bytes::init();
    time::init();
    interruptions::init();
    intvec::init();

    #[cfg(feature = "cffi")]
    cffi::init();
}
