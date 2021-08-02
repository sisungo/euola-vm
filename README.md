# euolaVM
euolaVM is a general purpose register-based abstract machine. euolaVM
is portable, free and strives to be fast and powerful. This repository provides
the core euolaVM runtime, and a basic euolaVM library: `ceras`.

## Features
The following features are provided by euolaVM runtime:
 - Direct FFI with C libraries
 - Corotines

## Getting Started
The `Hello, world!` program in euolaVM (using a basic wrapper library `system`) is:

    |>main
        v 100 U f"Hello, world!"
        C system::stdout::println
    <|

Run this program with:

```
./target/release/euola-vm hello.s:libsystem.s
```

The you will see `Hello, world` printed on the console.

## Project Plan
The following are plans of this project. Some plans will take a lot of time to implement:
 - GC instead of RC for references
 - Socket in `libraw`
 - euolaVM Executable Ball Binary
 - Print backtrace on `panic!` when `std::backtrace` went stable
 - Encryption in `libraw`

## Portability
The following platforms are tested:

```
x86_64-unknown-linux-musl
```

The following platforms are successfully built:

```
x86_64-pc-windows-gnu
```

The following platforms are planned to test:

```
x86_64-unknown-freebsd
```
