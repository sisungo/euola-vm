|>ceras::cval::new
~ raw::coro::yield
d 0 ceras::cval n
S 0 raw::cffi::type 100
S 0 val 101
D 0 100
~ raw::coro::yield
r
<|

|>ceras::cval::fmt
G 100 val 100
C ceras::fmt
r
<|

|>ceras::ffi::openlib
~ raw::coro::yield
C raw::cffi::getpath
C raw::cffi::openlib
~ raw::coro::yield
r
<|

|>ceras::ffi::opensym
~ raw::coro::yield
C raw::cffi::opensym
j 100 7
v 100 U f"\e[31merror: \e[0maborted due to a symbol cannot be found on your system dynamic library.\n"
C raw::cio::print
v 100 32 101
C raw::env::exit
D 101 100
~ raw::coro::yield
r
<|

|>ceras::ffi::invoke
~ raw::coro::yield
C raw::cffi::invoke
~ raw::coro::yield
r
<|
