|>ceras::coro::_init
C ceras::cls::open
r
<|

|>ceras::coro::_cleanup
C ceras::cls::close
r
<|

|>ceras::coro::_fini
C ceras::coro::_cleanup
~ raw::coro::exit
r
<|

|>ceras::coro::spawn
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
C raw::vhw::expand
c 0
C ceras::coro::_fini
<|

|>ceras::coro::kill
D 100 0
C ceras::cls::_close
D 0 100
~ raw::coro::kill
r
<|
