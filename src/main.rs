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
pub mod resolver;
pub mod vmem;

use ansi_term::{
    Color::{Red, Yellow},
    Style,
};
use std::{env, process::exit};

/// Get environment `EUOLA_VM_EXECUTE`.
fn getexec() -> String {
    let result = match env::args().nth(1) {
        Some(x) => x,
        None => {
            eprintln!(
                "{}argument No. 1 is not specified or invalid.",
                Style::new().bold().fg(Red).paint("error: ")
            );
            exit(-1);
        }
    };
    result
}

/// Load dependencies.
fn loadstr(v: &str, c: &str) {
        for i in v.split(':') {
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

/// Load dependencies.
fn loads(v: &str, c: &str) {
    if let Ok(x) = env::var(v) {
        loadstr(&x, c);
    }
}

/// Get symbol `_start`.
fn getstart() -> isa::VirtFuncPtr {
    use context::getfp;
    use isa::FuncPtr;

    let fp = match getfp("_start") {
        Some(x) => x,
        None => {
            eprintln!(
                "{}cannot execute the program: symbol `_start` not found!",
                Style::new().bold().fg(Red).paint("error: "),
            );
            exit(-1);
        }
    };
    match fp {
        FuncPtr::Virtual(x) => x,
        FuncPtr::Native(_) => {
            eprintln!(
                "{}cannot execute the program: symbol `_start` is not managed!",
                Style::new().bold().fg(Red).paint("error: "),
            );
            exit(-1);
        }
    }
}

/// The start point of the program.
fn main() {
    os::init_minimal();

    let bf = getexec();

    loadstr(&bf, "archive");
    loads("EUOLA_VM_PRELOAD", "preload");
    let vfp = getstart();

    libraw::init();
    os::init_pre();
    executor::start(context::Thread::new(vfp));
}
