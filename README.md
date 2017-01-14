# `steed`

> [WIP] Rust's standard library, free of C dependencies, for Linux systems

**It's very early days. Very little functionality has been ported over.**

**Disclaimer** Do not use this crate for any purpose other that running the
examples in this repository. This crate has not been audited or thoroughly
tested.

- [Goals](#goals)
- [Non-goals](#non-goals)
- [Features](#features)
- [Supported architectures](#supported-architectures)
- [Usage](#usage)
- [Current functionality](#current-functionality)
- [Contributing](#contributing)
- [License](#license)
  - [Contribution](#contribution)

## Goals

The ultimate goal is achieve truly hassle free cross compilation from any system
(macOS, Windows, *BSD, etc.) to any Linux system, be it x86 or ARM hardware.
That is:

```
$ cargo build --target aarch64-unknown-linux-gnu
```

Should work without having to install a C toolchain or cross compiled C
libraries, and without having to run the command inside a Docker container / VM.

By removing all the C dependencies, `steed` solves half of the problem. The
other half will be solved by embedding lld, LLVM's multi-arch linker, into
`rustc`.

The short term goal is to provide the exact same functionality and API as the
`std` crate. Hopefully, by the time that's complete, [PAL] and std-aware Cargo
will be around and we'll be able to plug a `pal_steed` crate into the normal
`std` to get C free Rust programs.

[PAL]: https://internals.rust-lang.org/t/refactoring-std-for-ultimate-portability/4301

Although, IMO, it would make more sense (\*) and it would be more ergonomic to
land chunks of `steed` in rust-lang/rust and add a new set of built-in target,
e.g. `aarch64-linux`. That way, compiling C free Rust programs would be as
simple as e.g. `cargo build --target aarch64-linux`.

(\*) Because linking requires different linker arguments and programs are always
statically linked.

## Non-goals

- Supporting other unix like systems, like the *BSD ones. Mainly because we have
  no (easy) way to test them inside Travis and also to reduce the amount of work
  required. After we are done with the C free port of `std` for Linux then we
  can start thinking about supporting other OSes.

## Features

- Zero C code. Zero C dependencies. Not even startup objects are required to
  link a `steed` binary. `steed` programs should still be able to interoperate
  with or link to C code though (This last part is currently untested).

- Small, statically linked binaries:

``` rust
// hello.rs
#![no_std]
#![no_main]

#[macro_use]
extern crate steed;

use steed::io;
use steed::io::Write;

#[no_mangle]
pub fn main() -> i32 {
    io::stdout().write_all(b"Hello, world!\n").map(|_| 0).unwrap_or(1)
}
```

```
# cross build --target x86_64-unknown-linux-gnu --release --example hello
$ ./hello
Hello, world!

$ strip -s hello

$ file hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, stripped

$ size hello
   text    data     bss     dec     hex filename
    131       0       0     131      83 hello

$ ls -l hello
-rwxr-xr-x 1 japaric japaric 592 Jan 01 00:00 hello
```

**Disclaimer** The binary size will inevitably go up after we add missing
[startup/runtime features][rt] like stack overflow protection and unwinding.

[rt]: https://github.com/rust-lang/rust/blob/1.14.0/src/libstd/rt.rs#L42-L60

## Supported architectures

`steed` is [continuously tested][ci] on these platforms (using QEMU):

[ci]: https://travis-ci.org/japaric/steed

- ARM

<!-- - MIPS -->

- PowerPC

<!-- - SPARC -->

- x86

Or in terms of existing Rust targets:

- `aarch64-unknown-linux-gnu`

- `arm-unknown-linux-gnueabi`

- `armv7-unknown-linux-gnueabihf`

- `i686-unknown-linux-gnu`

<!-- - `mips-unknown-linux-gnu` -->

<!-- - `mips64-unknown-linux-gnu` -->

<!-- - `mips64el-unknown-linux-gnu` -->

<!-- - `mipsel-unknown-linux-gnu` -->

- `powerpc-unknown-linux-gnu`

- `powerpc64-unknown-linux-gnu`

- `powerpc64le-unknown-linux-gnu`

<!-- - `sparc64-unknown-linux-gnu` -->

- `x86_64-unknown-linux-gnu`

## Usage

Right now, you can only run the examples in this repository :-).

```
$ cargo install cross

$ cross run --target aarch64-unknown-linux-gnu --example hello
Hello, world!

$ cross run --target powerpc-unknown-linux-gnu --example panic
panicked at 'explicit panic', examples/panic.rs:8
```

## Current functionality

Check the [API docs](https://japaric.github.io/steed/steed/index.html), but to
sum it up:

- Standard I/O (stdin, stdout, stderr)

- File I/O

- Dynamic memory allocation (thanks to [ralloc]!)

[ralloc]: https://github.com/redox-os/ralloc

Yup, that's all! I did say it was very early days, didn't I?

## Contributing

There's plenty of work to do (*spoilers* most of it is copy pasting code from
the rust-lang/rust repo) and *you* can help! Check out the [issue tracker].
We have tagged the issues according to their difficulty from `D-easy` to
`D-hard`. Each issue description has instructions on how to proceed but keep
these general guidelines in mind:

[issue tracker]: https://github.com/japaric/steed/issues

- We can't depend on the `libc` crate because that will make all our programs
  link to `libc`, `libm`, etc.

- We still don't support the `#[test]` attribute so, if you add new
  functionality, please add a smoke test in the form of an example that
  exercises the new functionality.

- Some functionality, like `std::Path`, is *architecture* independent.
  Re-implementing that functionality in `steed` is as simple as copy pasting
  `std`'s code. When that happens, make sure to copy from a *stable* Rust
  release, e.g. 1.14.0, and to also add a comment indicates from which Rust
  version the code came from.

- Try to mimic the layout of rust-lang/rust's [src/libstd] directory as much as
  possible. I expect that, except for the `sys` and `linux` modules, everything
  else will be very similar in layout and contents to the rust-lang/rust repo.

[src/libstd]: https://github.com/rust-lang/rust/tree/1.14.0/src/libstd

- Keep all the code that directly interfaces with the Linux kernel in the
  *private* `linux` module.

- Some code will involve constants / flags like `EBADF` or `O_CLOEXEC`. Get the
  values of those constants from the [Linux kernel source code][linux]. Be
  careful with architecture dependent values! Look for `O_CLOEXEC` inside the
  `linux` module to see how those are handled.

[linux]: https://www.kernel.org/

```
# cargo install ripgrep
$ rg 'define[ \t]+\bO_CLOEXEC[ \t]+'
tools/perf/builtin-trace.c
51:# define O_CLOEXEC           02000000

include/uapi/asm-generic/fcntl.h
62:#define O_CLOEXEC    02000000        /* set close_on_exec */

arch/sparc/include/uapi/asm/fcntl.h
20:#define O_CLOEXEC    0x400000

arch/parisc/include/uapi/asm/fcntl.h
16:#define O_CLOEXEC    010000000 /* set close_on_exec */

arch/alpha/include/uapi/asm/fcntl.h
17:#define O_CLOEXEC    010000000 /* set close_on_exec */
```

- Some code will require doing system calls. Instead of directly using the
  `syscall!` macro, create a wrapper function to add type checking. As for the
  signature of the wrapper function, base it on the signature used in the Linux
  kernel:

```
$ rg '\bSYSCALL_DEFINE.\(open,'
fs/open.c
1066:SYSCALL_DEFINE3(open, const char __user *, filename, int, flags, umode_t, mode)
```

Would become:

``` rust
#[inline(always)]
pub unsafe fn open(filename: *const c_char, flags: c_int, mode: umode_t) { .. }
```

- Some of those signatures will involve type aliases like `umode_t`. Get those
  from the Linux kernel source as well and add the type aliases to the `linux`
  module.

```
$ rg 'typedef.+umode_t'
include/linux/types.h
18:typedef unsigned short               umode_t;
```

- `man 2` is your friend. Most system calls are documented in there. For
  example, `man 2 write`, documents the write system call.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
