# Status

"It's dead, Jim".png

This project has been inactive since 2017-10-20.

If you are interested in hassle-free cross compilation from any OS to Linux I
would suggest looking into the Linux/MUSL targets:

``` console
$ cargo new --bin hello && cd hello

$ rustup target add x86_64-unknown-linux-musl

$ cargo rustc --target x86_64-unknown-linux-musl -- -C linker=rust-lld

$ file target/x86_64-unknown-linux-musl/debug/hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, with debug_info, not stripped
```

If you are interested in a Rust standard library free of C dependencies for
Linux then I would suggest registering your interested in [rust-lang/rfcs#2610]

[rust-lang/rfcs#2610]: https://github.com/rust-lang/rfcs/issues/2610

If you are interested in taking over the development of this project send me an
e-mail (see my GH profile) and I'll transfer it and its dependencies to you.

-- @japaric, 2018-12-09

---

# `steed`

> [WIP] Rust's standard library, free of C dependencies, for Linux systems

**It's very early days. Very little functionality has been ported over.**

- [Goals](#goals)
- [Non-goals](#non-goals)
- [Features](#features)
- [Supported architectures](#supported-architectures)
- [Usage](#usage)
  - [Using `cross`](#using-cross)
  - [Using `lld`](#using-lld)
  - [Using `gcc`](#using-gcc)
- [Current functionality](#current-functionality)
- [Contributing](#contributing)
- [License](#license)
  - [Contribution](#contribution)

## Goals

The ultimate goal is achieve truly hassle free cross compilation from any system
(macOS, Windows, *BSD, etc.) to any Linux system, be it x86 or ARM hardware.
That is:

```
$ cargo build --target aarch64-unknown-linux-steed
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
// examples/hello.rs
use std::{io, process};
use std::io::Write;

fn main() {
    if io::stdout().write_all(b"Hello, world!\n").is_err() {
        process::exit(1)
    }
}
```

```
# xargo rustc --target x86_64-unknown-linux-steed --release --example hello -- -C lto
$ ./hello
Hello, world!

$ strip -s hello

$ file hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, stripped, with debug_info

$ size hello
   text    data     bss     dec     hex filename
    173       0       0     173      ad hello

$ ls -l hello
-rwxr-xr-x 2 japaric japaric 4712 Apr 11 00:00 hello
```

**Disclaimer** The binary size will inevitably go up after we add missing
[startup/runtime features][rt] like stack overflow protection and unwinding.

[rt]: https://github.com/rust-lang/rust/blob/1.14.0/src/libstd/rt.rs#L42-L60

## Supported architectures

Turns out that writing architecture specific code is tricky! We were trying to
provide "if it compiles, it works" level of support for every architecture that
`std` supports but that stalled development as it's hard to get some features
working on a bunch of different architectures.

So, we have adopted Rust's tier system to unstuck development. Our platform
support is split in two tiers:

### Tier 1

"If it compiles, it works" level of support. PRs won't land if [CI] tests don't
pass on these platforms:

[CI]: https://travis-ci.org/japaric/steed

- `aarch64-unknown-linux-steed`

- `arm-unknown-linux-steedeabi`

- `armv7-unknown-linux-steedeabihf`

- `i686-unknown-linux-steed`

- `x86_64-unknown-linux-steed`

### Tier 2

"Best effort" level of support. Using some features that require architecture
specific code, like threads, may panic at runtime if the work to support that
feature has not been done yet (i.e. `unimplemented!()`). We don't block PRs if
CI tests don't pass on these platforms:

- `mips-unknown-linux-steed`

<!-- - `mips64-unknown-linux-steed` -->

<!-- - `mips64el-unknown-linux-steed` -->

- `mipsel-unknown-linux-steed`

- `powerpc-unknown-linux-steed`

- `powerpc64-unknown-linux-steed`

<!-- - `powerpc64le-unknown-linux-steed` -->

<!-- - `sparc64-unknown-linux-steed` -->

We eventually hope to move all targets into the tier 1 but we'll need help from
people more familiar with the non-x86 architectures. If you'd like to help,
feel free to contact us on IRC (#rust-steed @ irc.mozilla.org) or via the issue
tracker, or directly tackle the architecture specific issues, issue tagged with
e.g. `A-powerpc`, listed on the issue tracker.

## Usage

To compile your library / application against `steed`, follow these steps:

> **DISCLAIMER** `steed` has not achieved feature parity with `std` yet, so it's
> likely that your crate won't compile against `steed`. However, if your crate
> compiles against `steed` and then crashes or doesn't behave as expected at
> runtime, that's a bug and we would appreciate a [bug report][issues].

[issues]: https://github.com/japaric/steed/issues

### Using `cross`

To easiest way to use `steed` is to use the [`cross`] tool:

[`cross`]: https://github.com/japaric/cross#cross

> **NOTE** `cross` depends on Docker and only works on x86_64 Linux

```
# Always use the latest version
$ cargo install cross

# instead of this step, just go to the crate you want to build
$ cargo new --bin hello && cd $_

# Xargo magic to replace `std` with `steed`
# (if you want to run tests, fetch `Xargo.test.toml` instead of `Xargo.std.toml`)
$ curl -L https://raw.githubusercontent.com/japaric/steed/master/Xargo.std.toml > Xargo.toml
```

```
# NOTE `steed` uses its own set of targets; these have `steed` instead of `gnu`
# in their triples
$ cross run --target x86_64-unknown-linux-steed
Hello, world!

$ file target/x86_64-unknown-linux-steed/debug/hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV),statically linked, not stripped, with debug_info
```

You can use `cross` to cross compile your crate to other architectures as well.

```
# continuation from the previous example
$ cross build --target aarch64-unknown-linux-steed

$ file target/aarch64-unknown-linux-steed/debug/hello
hello: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV), statically linked, not stripped, with debug_info
```

`cross` can even transparently execute those cross compiled binaries using QEMU.
QEMU doesn't need to be installed on the host system.

```
$ cross run --target aarch64-unknown-linux-steed -v
       Fresh hello v0.1.0 (file:///project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `/target/aarch64-unknown-linux-steed/debug/hello`
Hello, world!
```

> **NOTE** `cross test` works as well but you have to use the other Xargo.toml

### Using `lld`

If you are not running x86_64 Linux or don't want to install / use Docker. You
can compile `steed` programs using [`xargo`] and [`lld`].

[`xargo`]: https://github.com/japaric/xargo#xargo
[`lld`]: https://lld.llvm.org/

```
# This is for Ubuntu, adjust as necessary
$ sudo apt-get install lld-4.0

# Xargo v0.3.4 or newer is required
$ cargo install xargo

# OMITTED: fetching Xargo.toml

# Fetch the target definition
$ curl -LO https://raw.githubusercontent.com/japaric/steed/master/docker/x86_64-unknown-linux-steed.json

$ xargo run --target x86_64-unknown-linux-steed
Hello, world!
```

Cross compilation works out of the box; there's no need to install a cross C
toolchain:

```
# fetch another target definition
$ curl -LO https://raw.githubusercontent.com/japaric/steed/master/docker/aarch64-unknown-linux-steed.json

$ xargo build --target aarch64-unknown-linux-steed

$ file target/aarch64-unknown-linux-steed/debug/examples/hello
hello: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV), statically linked, not stripped, with debug_info
```

To execute foreign binaries you can use QEMU:

```
# This is for Ubuntu, adjust as necessary
$ sudo apt-get install qemu-user

$ qemu-aarch64 target/aarch64-unknown-linux-steed/debug/examples/hello
Hello, world!
```

### Using `gcc`

If you don't want to install `lld`, you can link `steed` programs using `gcc`,
which you probably already have installed.

```
$ xargo rustc --target x86_64-unknown-linux-steed -- -C linker=gcc -Z linker-flavor=gcc

$ file target/x86_64-unknown-linux-steed/debug/hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, BuildID[sha1]=ac4fce139edd9741b818cd73123be6c934718f78, not stripped, with debug_info
```

## Current functionality

Check the [API docs](https://japaric.github.io/steed/steed/index.html), but to
summarize the functionality that interfaces with the Linux kernel:

- Standard I/O (stdin, stdout, stderr)

- File I/O

- Filesystem operations (`std::fs`)

- `std::net`: TCP, UDP. `lookup_host` is missing.

- Dynamic memory allocation (thanks to [ralloc]!)

- `std::time`

- Minimal thread support.

[ralloc]: https://github.com/redox-os/ralloc

Yup, that's all! I did say it was very early days, didn't I?

## Contributing

There's plenty of work to do (*spoilers* most of it is copy pasting code from
the rust-lang/rust repo) and *you* can help! Check out
the [issue tracker][issues]. We have tagged the issues according to their
difficulty from `D-easy` to `D-hard`. Each issue description has instructions on
how to proceed but keep these general guidelines in mind:

[issues]: https://github.com/japaric/steed/issues

- We have an IRC channel on Mozilla network, #rust-steed, if you have any
  question!

- We can't depend on the `libc` crate because that will make all our programs
  link to `libc`, `libm`, etc.

- We still don't support the `#[test]` attribute so, if you add new
  functionality, please add a smoke test in the form of an example that
  exercises the new functionality to the `examples` directory and to the list in
  `ci/script.sh`.

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
// fs/open.c
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
