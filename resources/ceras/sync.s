|>ceras::cls::open
C raw::hashmap::new
D 100 0
C ceras::cls::_get_tname
D 0 101
C raw::thread::tls_set
r
<|

|>ceras::cls::close
~ raw::coro::getcid
C ceras::cls::_close
r
<|

|>ceras::cls::_close
C ceras::cls::_get_tname_impl
D 100 0
C raw::thread::tls_get
D 101 100
C raw::hashmap::drop
D 0 100
C raw::thread::tls_remove
r
<|

|>ceras::cls::get
~ raw::coro::yield
D 100 0
C ceras::cls::_get_tname
C raw::thread::tls_get
D 101 100
D 0 101
C raw::hashmap::get
~ raw::coro::yield
r
<|

|>ceras::cls::set
~ raw::coro::yield
D 100 0
D 101 1
C ceras::cls::_get_tname
C raw::thread::tls_get
D 101 100
D 0 101
D 1 102
C raw::hashmap::set
~ raw::coro::yield
r
<|

|>ceras::cls::_get_tname_impl
C raw::str::from<auto>
D 100 101
d 100 U f"ceras::cls::_table@"
C raw::str::push_str
r
<|

|>ceras::cls::_get_tname
~ raw::coro::getcid
C ceras::cls::_get_tname_impl
r
<|

|>ceras::ref::new
d 0 ceras::ref n
S 0 val 100
D 0 100
r
<|

|>ceras::ref::get
G 100 val 100
r
<|

|>ceras::ref::set
S 100 val 101
r
<|

|>ceras::ref::fmt
C ceras::ref::get
C ceras::fmt
r
<|

|>ceras::lazy::new
d 0 ceras::lazy n
v 1 9 0
S 0 stat 1
S 0 init 100
D 0 100
r
<|

|>ceras::lazy::get
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

|>ceras::lazy::fmt
C ceras::lazy::get
C ceras::fmt
r
<|

|>ceras::lazy::to_ref
C ceras::lazy::get
C ceras::ref::new
r
<|
