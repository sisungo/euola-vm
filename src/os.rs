//!
//! OS-specified & low-level utilities.
//!

use ansi_term::{Color::Red, Style};
use libc::c_int;

/// Initialize minimalized functions.
#[inline(always)]
pub fn init_minimal() {
    #[cfg(windows)]
    {
        ansi_term::enable_ansi_support().ok();
    }
}

/// Initialize before-vm functions.
#[inline(always)]
pub fn init_pre() {
    unsafe {
        libc::signal(libc::SIGSEGV, segfault_handler as usize);
    }
}

/// Segmention fault handler.
extern "C" fn segfault_handler(_: c_int) {
    eprintln!(
        "{}",
        Style::new().underline().fg(Red).paint(" !!! ABORTED !!! ")
    );
    eprintln!("All units aborted due to a very fatal interruption.");
    eprintln!("Interruption: {}", Red.paint("raw::fatal::unsafe_code"));
    eprintln!("     Message: Received signal `SIGSEGV` from OS. This is probably an abuse of `raw::cffi::*`.");
    eprintln!("              This interruption is unhandlable. If the FFI codes are right, please report this");
    eprintln!("              as a bug of euolaVM.\n");
    eprintln!("  World View: <null>\n");
    eprintln!("Aborting...");
    std::process::abort();
}
