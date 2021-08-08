|>_start
C raw::coro::enter
<|
|>_start_coro
C system::_eh_init
C system::patches::init
C system::_env_init
C system::coro::_init
C system::_start_main
v 100 32 0
C system::proc::exit
<|
|>system::_start_main
v 100 U f"main"
C raw::vhw::locate_func
D 100 0
? 0 1
j 1 7
c 0
r
v 100 U f"\e[31merror: \e[0mfunction `main` not found."
C system::stdout::println
v 100 32 1
C system::proc::exit
<|
|>system::_eh_init
C system::_raw_eh_segfault
C system::_raw_eh_argnull
C system::_raw_eh_nofn
C system::_raw_eh_oor
C system::_raw_eh_mte
C system::_raw_eh_nai
C system::_raw_eh_nab
C system::_raw_eh_nao
C system::_raw_eh_invalid
C system::_raw_eh_enve
C system::_raw_eh_nap
C system::_manual_eh
r
<|
|>system::_raw_eh_segfault
v 100 U f"raw::fatal::segfault"
v 101 U f"SIL access out-of-range. Valid SIL area is from 0 to 150."
C raw::int::abort
r
<|
|>system::_raw_eh_argnull
v 100 U f"raw::fatal::argument_null"
v 101 U f"Attempted to invoke a non-null-only feature, but null is passed."
C raw::int::abort
r
<|
|>system::_manual_eh
v 100 U f"system::fatal::manual_eh"
v 101 U f"Attempting to dump the thread-specificed context manually."
C raw::int::abort
r
<|
|>system::_raw_eh_nofn
v 100 U f"raw::fatal::no_such_func"
v 101 U f"Attempted to call a function which is not in this space."
C raw::int::abort
r
<|
|>system::_raw_eh_oor
v 100 U f"raw::fatal::out_of_range"
v 101 U f"Raw collection access out of range."
C raw::int::abort
r
<|
|>system::_raw_eh_mte
v 100 U f"raw::fatal::math_type_error"
v 101 U f"Different-sized type values are used on math computing, or not an integer."
C raw::int::abort
r
<|
|>system::_raw_eh_nai
v 100 U f"raw::fatal::not_an_integer"
v 101 U f"Attempted to invoke a feature that requires an integer, but passed a value that is not an integer."
C raw::int::abort
r
<|
|>system::_raw_eh_nab
v 100 U f"raw::fatal::not_a_buf"
v 101 U f"Attempted to invoke a feature that requires a buffer, but passed a value that is not a buffer, or the buffer is invalid."
C raw::int::abort
r
<|
|>system::_raw_eh_nao
v 100 U f"raw::fatal::not_an_object"
v 101 U f"Attempted to invoke a feature that requires an object, but passed a non-object."
C raw::int::abort
r
<|
|>system::_raw_eh_enve
v 100 U f"raw::fatal::environment_error"
v 101 U f"Unknown environment error (module: OS Random Number Generator)"
C raw::int::abort
r
<|
|>system::_raw_eh_invalid
v 100 U f"raw::fatal::invalid"
v 101 U f"Invalid data is used."
C raw::int::abort
v 100 U f"raw::fatal::invalid_envid"
v 101 U f"Environment variable name is invalid."
C raw::int::abort
r
<|
|>system::_raw_eh_nap
v 100 U f"raw::fatal::not_a_ptr"
v 101 U f"Attempted to invoke a feature that requires a virtual pointer integer, but other kind of value is passed."
C raw::int::abort
r
<|
|>system::_env_init
v 100 U f"3.14159265358979323846264338327950288"
C raw::f64::from<str>
s system::math::pi 101
v 100 U f"2.71828182845904523536028747135266250"
C raw::f64::from<str>
s system::math::e 101
r
<|
|>system::coro::_init
C system::cls::open
r
<|
|>system::coro::_cleanup
C system::cls::close
r
<|
|>system::coro::_fini
C system::coro::_cleanup
~ raw::coro::exit
r
<|
|>system::coro::exit
C system::coro::_fini
<|
|>system::coro::spawn
d 0 v n
D 101 1
D 100 101
D 0 100
C raw::vec::push
D 1 101
C raw::vec::push
D 0 101
v 100 U f"system::coro::_coro_body"
~ raw::coro::spawn
r
<|
|>system::coro::_coro_body
D 100 0
D 101 1
C system::coro::_init
D 1 100
C raw::vhw::expand<topsil>
c 0
C system::coro::_fini
<|
|>system::coro::kill
D 100 0
C system::UniqueCLS::_close
D 0 100
~ raw::coro::kill
r
<|
|>system::cls::open
C raw::hashmap::new
D 100 0
C system::UniqueCLS::_get_tname
D 0 101
C raw::thrd::tls::set
r
<|
|>system::cls::close
~ raw::coro::getcid
C system::UniqueCLS::_close
r
<|
|>system::UniqueCLS::_close
C system::UniqueCLS::_get_tname_impl
D 100 0
C raw::thrd::tls::get
D 101 100
C raw::hashmap::drop
D 0 100
C raw::thrd::tls::del
r
<|
|>system::cls::get
~ raw::coro::yield
D 100 0
C system::UniqueCLS::_get_tname
C raw::thrd::tls::get
D 101 100
D 0 101
C raw::hashmap::get
~ raw::coro::yield
r
<|
|>system::cls::set
~ raw::coro::yield
D 100 0
D 101 1
C system::UniqueCLS::_get_tname
C raw::thrd::tls::get
D 101 100
D 0 101
D 1 102
C raw::hashmap::set
~ raw::coro::yield
r
<|
|>system::cls::del
~ raw::coro::yield
D 100 0
C system::UniqueCLS::_get_tname
C raw::thrd::tls::get
D 101 100
D 0 101
C raw::hashmap::remove
~ raw::coro::yield
r
<|
|>system::UniqueCLS::_get_tname_impl
C raw::str::from<auto>
D 100 101
d 100 U f"system::UniqueCLS::_table@"
C raw::str::push_str
r
<|
|>system::UniqueCLS::_get_tname
~ raw::coro::getcid
C system::UniqueCLS::_get_tname_impl
r
<|
|>system::ref::new
d 0 system::ref n
S 0 val 100
D 0 100
r
<|
|>system::ref::get
G 100 val 100
r
<|
|>system::ref::set
S 100 val 101
r
<|
|>system::ref::fmt
C system::ref::get
C system::fmt
r
<|
|>system::lazy::new
d 0 system::lazy n
v 1 9 0
S 0 stat 1
S 0 init 100
D 0 100
r
<|
|>system::lazy::get
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
|>system::lazy::fmt
C system::lazy::get
C system::fmt
r
<|
|>system::lazy::to_ref
C system::lazy::get
C system::ref::new
r
<|
|>system::fmt
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
|>system::fmt_nl
C system::fmt
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
|>system::stdout::print
~ raw::coro::yield
C system::fmt
C raw::cio::print
C raw::cio::flush
~ raw::coro::yield
r
<|
|>system::stdout::println
~ raw::coro::yield
C system::fmt_nl
C raw::cio::print
~ raw::coro::yield
r
<|
|>system::stderr::print
~ raw::coro::yield
C system::fmt
C raw::cio::eprint
~ raw::coro::yield
r
<|
|>system::stderr::println
~ raw::coro::yield
C system::fmt_nl
C raw::cio::eprint
~ raw::coro::yield
r
<|
|>system::stdin::readline
~ raw::coro::yield
C raw::cio::read
C raw::str::from<bytes>
~ raw::coro::yield
r
<|
|>system::coro::sleep<sec>
v 4 9 1
t 100 65 100
D 100 0
C raw::time::get<utc,sec>
D 100 1
~ raw::coro::yield
C raw::time::get<utc,sec>
- 100 1 2
> 2 0 3
^ 3 4 3
j 3 5
r
<|
|>system::coro::sleep<msec>
v 4 9 1
t 100 65 100
D 100 0
C raw::time::get<utc,msec>
D 100 1
~ raw::coro::yield
C raw::time::get<utc,msec>
- 100 1 2
> 2 0 3
^ 3 4 3
j 3 5
r
<|
|>system::coro::sleep<nsec>
v 4 9 1
t 100 65 100
D 100 0
C raw::time::get<utc,nsec>
D 100 1
~ raw::coro::yield
C raw::time::get<utc,nsec>
- 100 1 2
> 2 0 3
^ 3 4 3
j 3 5
r
<|
|>system::coro::milestone::new
d 0 system::coro::milestone n
v 1 9 0
S 0 stat 1
D 0 100
r
<|
|>system::coro::milestone::reach
v 1 9 1
S 100 stat 1
~ raw::coro::yield
r
<|
|>system::coro::milestone::unreach
v 1 9 0
S 100 stat 1
~ raw::coro::yield
r
<|
|>system::coro::milestone::wait
v 2 9 0
D 100 0
~ raw::coro::yield
G 0 stat 1
= 1 2 3
j 3 2
r
<|
|>system::coro::mutex::new
C system::coro::milestone::new
C system::coro::milestone::reach
r
<|
|>system::coro::mutex::lock
C system::coro::milestone::wait
C system::coro::milestone::unreach
r
<|
|>system::coro::mutex::unlock
C system::coro::milestone::reach
r
<|
|>system::proc::crash
D 100 0
v 100 U f"\e[31merror: \e[0mprogram crashed with message `"
C system::stderr::print
D 0 100
C system::stderr::print
v 100 U f"`"
C system::stderr::println
v 100 U n
C system::stderr::println
v 100 U f"ENVIRONMENT REPORT:"
C system::stderr::println
C raw::vhw::dump<context>
C system::stderr::println
~ system::fatal::manual_eh
<|
|>system::proc::add_library
~ raw::coro::yield
C raw::dl::load_file
~ raw::coro::yield
r
<|
|>system::proc::exit
D 100 0
C system::coro::_cleanup
D 0 100
C raw::env::exit
<|
|>system::patches::abort
v 100 U f"\e[31merror: \e[0mfunction `raw::env::abort` cannot be called with `libsystem`."
C system::stdout::println
v 100 32 1
C system::proc::exit
<|
|>system::patches::init
v 100 U f"raw::env::abort"
v 101 U f"system::patches::abort"
C raw::vhw::patch_func
v 100 U f"_start"
v 101 U f"system::patches::_start"
C raw::vhw::patch_func
r
<|
|>system::patches::_start
v 100 U f"\e[31merror: \e[0mfunction `_start` cannot be called inside of `libsystem` runtime."
C system::stdout::println
v 100 32 1
C system::proc::exit
r
<|
|>system::locale::get_config
v 100 U f"LANG"
C raw::env::getenv
? 100 0
j 0 5
C system::locale::_id_process
r
<|
|>system::locale::_id_process
v 101 U f"."
C raw::str::split
v 0 33 0
[ 100 0 1
D 1 100
v 101 U f"_"
v 102 U f"-"
C raw::str::replace
r
<|
|>system::locale::install
v 101 U f"r"
C raw::fs::open
j 100 8
D 101 100
C raw::fs::read_all
j 100 8
D 101 100
C system::locale::_parse
r
<|
|>system::locale::_parse
C raw::str::from<bytes>
? 100 0
j 0 5
C system::locale::_parse_inner
r
v 100 65 18
r
<|
|>system::locale::_parse_inner
v 101 U f"\n"
C raw::str::split
D 100 0
L 0 1
C raw::hashmap::new
D 100 3
v 2 65 0
< 2 1 4
v 5 65 1
^ 4 1 6
j 6 17
[ 0 2 100
D 3 101
C system::locale::_parse_line
v 7 65 1
+ 2 7 2
J 7
r
<|
|>system::locale::_parse_line
D 100 0
D 101 1
v 101 U f"\0"
C raw::string::split
D 1 101
C system::locale::_parse_line_inner
r
<|
|>system::locale::_parse_line_inner
D 100 0
D 101 1
L 0 2
v 3 65 1
v 4 65 0
= 1 2 4
j 4 12
D 1 100
[ 0 4 101
[ 0 3 102
C raw::hashmap::set
r
D 1 100
[ 0 4 101
d 102 U N
C raw::hashmap::set
r
<|
|>system::locale::get
C raw::hashmap::get
j 100 4
v 100 U N
r
D 101 100
r
<|
|>system::locale::uninstall
C raw::hashmap::drop
r
<|
