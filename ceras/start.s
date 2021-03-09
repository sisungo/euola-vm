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
v 101 U f"SIL access overflowed. Valid SIL area is from 0 to 150."
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
v 100 U f"3.141592653589793238462643383279502884197169399375105820974944"
C raw::f64::from<str>
s ceras::math::pi 101
r
<|
