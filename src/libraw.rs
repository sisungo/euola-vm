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
    init_rspanic_handler();
}

/// Initialize the Rust `panic!` handler.
#[inline(always)]
fn init_rspanic_handler() {
    std::panic::set_hook(Box::new(|info| {
        use ansi_term::{
            Color::{Blue, Red},
            Style,
        };

        eprintln!("\n {} euolaVM Panic", Red.paint("!!!"));
        let location = info.location().unwrap();
        let locs = Style::new().italic().underline();
        eprintln!(
            "Location: {}:{}",
            locs.bold().paint(location.file()),
            locs.bold().paint(location.line().to_string())
        );
        eprintln!(
            " Message: {}",
            info.payload()
                .downcast_ref::<&str>()
                .unwrap_or(&&info.to_string()[..])
        );
        eprintln!("\nThis seems to be a bug. The following ways are available to give a feedback:");
        let url = Style::new().italic().underline().fg(Blue);
        eprintln!(
            " - Website <{}>",
            url.paint("https://todo.sr.ht/~sisungo/euola-vm")
        );
        eprintln!(" - Email <{}>", url.paint("sisungo@protonmail.com"));
        std::process::exit(-1);
    }));
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
