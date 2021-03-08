|>ceras::fmt
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

|>ceras::fmt_nl
C ceras::fmt
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
