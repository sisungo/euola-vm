|>ceras::stdout::print
~ raw::coro::yield
C ceras::fmt
C raw::cio::print
C raw::cio::flush
~ raw::coro::yield
r
<|

|>ceras::stdout::println
~ raw::coro::yield
C ceras::fmt_nl
C raw::cio::print
~ raw::coro::yield
r
<|

|>ceras::stderr::print
~ raw::coro::yield
C ceras::fmt
C raw::cio::eprint
~ raw::coro::yield
r
<|

|>ceras::stderr::println
~ raw::coro::yield
C ceras::fmt_nl
C raw::cio::eprint
~ raw::coro::yield
r
<|

|>ceras::stdin::readline
~ raw::coro::yield
C raw::cio::read<str>
~ raw::coro::yield
r
<|
