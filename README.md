# euolaVM
euolaVM is a general purpose register-based abstract machine. euolaVM
is portable, free and strives to be fast and powerful. This repository provides
the core euolaVM runtime, and a basic euolaVM library: `ceras`.

## Features
The following features are provided by euolaVM runtime:
 - Direct FFI with C libraries
 - Corotines

## Getting Started
The `Hello, world!` program in euolaVM (using `ceras`) is:

    |>main
        v 100 U f"Hello, world!"
        C ceras::stdout::println
    <|

Run this program with:

    EUOLA_VM_EXECUTE=hello.s EUOLA_VM_DEPENDENCIES=ceras/start.s:ceras/stdio.s:ceras/fmt.s:ceras/ffi.s:ceras/coro.s:ceras/sync.s euola-vm

The you will see `Hello, world` printed on the console.

## Project Plan
The following are plans of this project. Some plans will take a lot of time to implement:
 - GC instead of RC for references
 - Socket in `libraw`
 - euolaVM Executable Ball Binary
 - Print backtrace on `panic!` when `std::backtrace` went stable

## Portability
The following platforms are tested:
    x86_64-unknown-linux-musl
    x86_64-pc-windows-gnu
