//!
//! OS-specified & low-level utilities.
//!

use ansi_term::{Color::Red, Style};
use libc::c_int;
use std::mem::MaybeUninit;

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
    #[cfg(any(target_os = "redox", windows))]
    unsafe {
        libc::signal(libc::SIGSEGV, segfault_handler as usize);
    }

    #[cfg(all(unix, not(target_os = "redox")))]
    unsafe {
        let mut action: MaybeUninit<libc::sigaction> = MaybeUninit::uninit();
        let mut action = action.assume_init_mut();
        action.sa_sigaction = segfault_handler as usize;
        action.sa_flags = libc::SA_SIGINFO;
        let mut place: libc::sigaction = MaybeUninit::uninit().assume_init();
        libc::sigaction(libc::SIGSEGV, action as *const _, &mut place as *mut _);
    }
}

/// Segmention fault handler.
#[cfg(any(target_os = "redox", windows))]
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
    eprintln!("  World View: Not supported on your platform.\n");
    eprintln!("Aborting...");
    std::process::abort();
}

/// Segmention fault handler.
#[cfg(all(not(target_os = "redox"), unix))]
extern "C" fn segfault_handler(_: c_int, info: *mut libc::siginfo_t, _: *const u8) {
    let info = unsafe { *info };
    eprintln!(
        "{}",
        Style::new().underline().fg(Red).paint(" !!! ABORTED !!! ")
    );
    eprintln!("All units aborted due to a very fatal interruption.");
    eprintln!("Interruption: {}", Red.paint("raw::fatal::unsafe_code"));
    eprintln!("     Message: Received signal `SIGSEGV` from OS. This is probably an abuse of `raw::cffi::*`.");
    eprintln!("              This interruption is unhandlable. If the FFI codes are right, please report this");
    eprintln!("              as a bug of euolaVM.\n");
    eprintln!("Fault Memory: {:p}", unsafe { info.si_addr() });
    eprintln!(" Signal Code: {}", info.si_code);
    eprintln!("       Errno: {}\n", info.si_errno);
    eprintln!("Aborting...");
    std::process::abort();
}
