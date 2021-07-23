|>_start
C raw::coro::enter
<|

|>_start_coro
C ceras::_intsetup
C ceras::_staticsetup
C ceras::coro::_init
C main
C ceras::coro::_cleanup
v 100 32 0
C raw::env::exit
<|

|>ceras::_intsetup
C ceras::_intsetup_segfault
C ceras::_intsetup_argnull
C ceras::_intsetup_nofn
C ceras::_intsetup_oor
C ceras::_intsetup_mte
C ceras::_intsetup_nai
C ceras::_intsetup_nab
C ceras::_intsetup_nao
C ceras::_intsetup_invalid
C ceras::_intsetup_enve
C ceras::_intsetup_nap
r
<|

|>ceras::_intsetup_segfault
v 100 U f"raw::fatal::segfault"
v 101 U f"SIL access out-of-range. Valid SIL area is from 0 to 150."
C raw::int::abort
r
<|

|>ceras::_intsetup_argnull
v 100 U f"raw::fatal::argument_null"
v 101 U f"Attempted to invoke a non-null-only feature, but null is passed."
C raw::int::abort
r
<|

|>ceras::_intsetup_nofn
v 100 U f"raw::fatal::no_such_func"
v 101 U f"Attempted to call a function which is not in this space."
C raw::int::abort
r
<|

|>ceras::_intsetup_oor
v 100 U f"raw::fatal::out_of_range"
v 101 U f"Raw collection access out of range."
C raw::int::abort
r
<|

|>ceras::_intsetup_mte
v 100 U f"raw::fatal::math_type_error"
v 101 U f"Different-sized type values are used on math computing, or not an integer."
C raw::int::abort
r
<|

|>ceras::_intsetup_nai
v 100 U f"raw::fatal::not_an_integer"
v 101 U f"Attempted to invoke a feature that requires an integer, but passed a value that is not an integer."
C raw::int::abort
r
<|

|>ceras::_intsetup_nab
v 100 U f"raw::fatal::not_a_buf"
v 101 U f"Attempted to invoke a feature that requires a buffer, but passed a value that is not a buffer, or the buffer is invalid."
C raw::int::abort
r
<|

|>ceras::_intsetup_nao
v 100 U f"raw::fatal::not_an_object"
v 101 U f"Attempted to invoke a feature that requires an object, but passed a non-object."
C raw::int::abort
r
<|

|>ceras::_intsetup_enve
v 100 U f"raw::fatal::environment_error"
v 101 U f"ceras::_enverr"
C raw::int::catch
r
<|

|>ceras::_enverr
v 100 U f"\e[0;31;4m !!! OS Error !!! \e[0m\n"
C raw::cio::print
v 100 U f"Failed to get random number from operating system.\n"
C raw::cio::print
v 100 U f"Please fix your OS random number generator.\n"
C raw::cio::print
v 100 U f"If you\'re sure your OS random number generator is available:\n        Please report this as a bug of euolaVM.\n"
C raw::cio::print
v 100 U f"\nAborting...\n\n"
C raw::cio::print
C raw::env::abort
r
<|

|>ceras::_intsetup_invalid
v 100 U f"raw::fatal::invalid"
v 101 U f"Invalid data is used."
C raw::int::abort
v 100 U f"raw::fatal::invalid_envid"
v 101 U f"Environment variable name is invalid."
C raw::int::abort
r
<|

|>ceras::_intsetup_nap
v 100 U f"raw::fatal::not_a_ptr"
v 101 U f"Attempted to invoke a feature that requires a virtual pointer integer, but other kind of value is passed."
C raw::int::abort
r
<|

|>ceras::_staticsetup
v 100 U f"3.14159265358979323846264338327950288"
C raw::f64::from<str>
s ceras::math::pi 101
v 100 U f"2.71828182845904523536028747135266250"
C raw::f64::from<str>
s ceras::math::e 101
r
<|

|>ceras::coro::_init
C std::cls::open
r
<|

|>ceras::coro::_cleanup
C std::cls::close
r
<|

|>ceras::coro::_fini
C ceras::coro::_cleanup
~ raw::coro::exit
r
<|

|>std::coro::spawn
d 0 v n
D 101 1
D 100 101
D 0 100
C raw::vec::push
D 1 101
C raw::vec::push
D 0 101
v 100 U f"ceras::coro::_coro_body"
~ raw::coro::spawn
r
<|

|>ceras::coro::_coro_body
D 100 0
D 101 1
C ceras::coro::_init
D 1 100
C raw::vhw::expand<topsil>
c 0
C ceras::coro::_fini
<|

|>std::coro::kill
D 100 0
C UniqueCLS::_close
D 0 100
~ raw::coro::kill
r
<|

|>std::cls::open
C raw::hashmap::new
D 100 0
C UniqueCLS::_get_tname
D 0 101
C raw::thread::tls_set
r
<|

|>std::cls::close
~ raw::coro::getcid
C UniqueCLS::_close
r
<|

|>UniqueCLS::_close
C UniqueCLS::_get_tname_impl
D 100 0
C raw::thread::tls_get
D 101 100
C raw::hashmap::drop
D 0 100
C raw::thread::tls_del
r
<|

|>std::cls::get
~ raw::coro::yield
D 100 0
C UniqueCLS::_get_tname
C raw::thread::tls_get
D 101 100
D 0 101
C raw::hashmap::get
~ raw::coro::yield
r
<|

|>std::cls::set
~ raw::coro::yield
D 100 0
D 101 1
C UniqueCLS::_get_tname
C raw::thread::tls_get
D 101 100
D 0 101
D 1 102
C raw::hashmap::set
~ raw::coro::yield
r
<|

|>UniqueCLS::_get_tname_impl
C raw::str::from<auto>
D 100 101
d 100 U f"UniqueCLS::_table@"
C raw::str::push_str
r
<|

|>UniqueCLS::_get_tname
~ raw::coro::getcid
C UniqueCLS::_get_tname_impl
r
<|

|>std::ref::new
d 0 std::ref n
S 0 val 100
D 0 100
r
<|

|>std::ref::get
G 100 val 100
r
<|

|>std::ref::set
S 100 val 101
r
<|

|>std::ref::fmt
C std::ref::get
C std::fmt
r
<|

|>std::lazy::new
d 0 std::lazy n
v 1 9 0
S 0 stat 1
S 0 init 100
D 0 100
r
<|

|>std::lazy::get
G 100 stat 1
j 1 9
D 100 0
G 0 init 2
c 2
v 3 9 1
S 0 stat 3
S 0 val 100
r
G 100 val 100
r
<|

|>std::lazy::fmt
C std::lazy::get
C std::fmt
r
<|

|>std::lazy::to_ref
C std::lazy::get
C std::ref::new
r
<|

|>std::fmt
~ raw::coro::yield
D 100 0
T 0 100
v 101 U f"::fmt"
C raw::str::push_str
D 100 1
D 0 100
c 1
~ raw::coro::yield
r
<|

|>std::fmt_nl
C std::fmt
v 101 9 10
C raw::bytes::push
r
<|

|>primitive::i8::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>primitive::u8::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>primitive::i16::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>primitive::u16::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>primitive::i32::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>primitive::u32::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>primitive::i64::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>primitive::u64::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>primitive::ptr::fmt
C raw::str::from<auto>
C raw::str::to_bytes
r
<|

|>raw::string::fmt
C raw::str::to_bytes
r
<|

|>raw::bytes::fmt
C raw::bytes::deep_clone
r
<|

|>std::stdout::print
~ raw::coro::yield
C std::fmt
C raw::cio::print
C raw::cio::flush
~ raw::coro::yield
r
<|

|>std::stdout::println
~ raw::coro::yield
C std::fmt_nl
C raw::cio::print
~ raw::coro::yield
r
<|

|>std::stderr::print
~ raw::coro::yield
C std::fmt
C raw::cio::eprint
~ raw::coro::yield
r
<|

|>std::stderr::println
~ raw::coro::yield
C std::fmt_nl
C raw::cio::eprint
~ raw::coro::yield
r
<|

|>std::stdin::readline
~ raw::coro::yield
C raw::cio::read<str>
~ raw::coro::yield
r
<|
