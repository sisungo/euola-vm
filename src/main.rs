//!
//! euolaVM is a general purpose abstract machine. euolaVM is portable, safety, and strives to be
//! fast and extendable.
//!

pub mod context;
/// euolaVM's core executing engines.
pub mod executor;
/// Data structure for an `Instruction` and function.
pub mod isa;
pub mod libraw;
pub mod os;
/// UTF-8 encoded text executable euolaVM executable ball resolving.
pub mod resolver;
pub mod vmem;

use ansi_term::{
    Color::{Red, Yellow},
    Style,
};
use std::{env, process::exit};

#[cfg_attr(feature = "fast-alloc", global_allocator)]
#[cfg(feature = "fast-alloc")]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Get environment `EUOLA_VM_EXECUTE`.
fn getexecenv() -> String {
    let result = match env::var("EUOLA_VM_EXECUTE") {
        Ok(x) => x,
        Err(_) => {
            eprintln!(
                "{}environment `EUOLA_VM_EXECUTE` is not set or invalid.",
                Style::new().bold().fg(Red).paint("error: ")
            );
            exit(-1);
        }
    };
    env::remove_var("EUOLA_VM_EXECUTE");
    result
}

/// Load dependencies.
fn loads(v: &str, c: &str) {
    if let Ok(x) = env::var(v) {
        for i in x.split(':') {
            if let Err(z) = resolver::resolve(i) {
                eprintln!(
                    "{}cannot resolve byte code file `{}`(load as a {}): {}",
                    Style::new().bold().fg(Yellow).paint("warning: "),
                    i,
                    c,
                    z
                );
            }
        }
    }
}

/// Get symbol `_start`.
fn getstart(bf: &str) -> isa::VirtFuncPtr {
    use context::getfp;
    use isa::FuncPtr;

    let fp = match getfp("_start") {
        Some(x) => x,
        None => {
            eprintln!(
                "{}cannot execute program `{}`: symbol `_start` not found!",
                Style::new().bold().fg(Red).paint("error: "),
                bf
            );
            exit(-1);
        }
    };
    match fp {
        FuncPtr::Virtual(x) => x,
        FuncPtr::Native(_) => {
            eprintln!(
                "{}cannot execute program `{}`: symbol `_start` is not managed!",
                Style::new().bold().fg(Red).paint("error: "),
                bf
            );
            exit(-1);
        }
    }
}

/// The start point of the program.
fn main() {
    os::init_minimal();

    let bf = getexecenv();
    if let Err(x) = resolver::resolve(&bf[..]) {
        eprintln!(
            "{}cannot resolve file `{}`: {}",
            Style::new().bold().fg(Red).paint("error: "),
            &bf[..],
            x
        );
        exit(-1);
    }

    loads("EUOLA_VM_DEPENDENCIES", "dependency");
    env::remove_var("EUOLA_VM_DEPENDENCIES");
    loads("EUOLA_VM_PRELOAD", "preload");
    let vfp = getstart(&bf);

    libraw::init();
    os::init_pre();
    executor::start(context::Thread::new(vfp));
}
