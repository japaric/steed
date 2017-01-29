# `steed`

> [WIP] Rust's standard library, free of C dependencies, for Linux systems

**It's very early days. Very little functionality has been ported over.**

- [Goals](#goals)
- [Non-goals](#non-goals)
- [Features](#features)
- [Supported architectures](#supported-architectures)
- [Usage](#usage)
  - [On x86_64 Linux](#on-x86_64-linux)
  - [On other systems](#on-other-systems)
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
// examples/zero.rs
fn main() {}
```

```
# xargo build --target x86_64-unknown-linux-steed --release --example zero
$ ./hello
Hello, world!

$ strip -s hello

$ file hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, stripped

$ size zero
   text    data     bss     dec     hex filename
    192       0      16     208      d0 hello

$ ls -l hello
-rwxr-xr-x 1 japaric japaric 784 Jan  1 00:00 hello
```

**Disclaimer** The binary size will inevitably go up after we add missing
[startup/runtime features][rt] like stack overflow protection and unwinding.

[rt]: https://github.com/rust-lang/rust/blob/1.14.0/src/libstd/rt.rs#L42-L60

## Supported architectures

`steed` is [continuously tested][ci] on these platforms (using QEMU):

[ci]: https://travis-ci.org/japaric/steed

- `aarch64-unknown-linux-steed`

- `arm-unknown-linux-steedeabi`

- `armv7-unknown-linux-steedeabihf`

- `i686-unknown-linux-steed`

<!-- - `mips-unknown-linux-steed` -->

<!-- - `mips64-unknown-linux-steed` -->

<!-- - `mips64el-unknown-linux-steed` -->

<!-- - `mipsel-unknown-linux-steed` -->

- `powerpc-unknown-linux-steed`

- `powerpc64-unknown-linux-steed`

<!-- - `powerpc64le-unknown-linux-steed` -->

<!-- - `sparc64-unknown-linux-steed` -->

- `x86_64-unknown-linux-steed`

## Usage

To compile your library / application against `steed`, follow these steps:

> **DISCLAIMER** `steed` has not achieved feature parity with `std` yet, so it's
> likely that your crate won't compile against `steed`. However, if your crate
> compiles against `steed` and then crashes or doesn't behave as expected at
> runtime, that's a bug and we would appreciate a [bug report][issues].

[issues]: https://github.com/japaric/steed/issues

### On x86_64 Linux

To easiest way to use `steed` is to use the `cross` tool:

> **NOTE** `cross` depends on Docker and only works on x86_64 Linux

```
# if you don't have cross installed
# (Cross v0.1.8 or newer is required)
$ cargo install cross

# instead of this step, just go to the crate you want to build
$ cargo new --bin hello && cd $_

# this is the part that replaces `std` with `steed`
$ edit Xargo.toml && cat $_
```

``` toml
[dependencies.collections]  # `steed` depends on collections

[dependencies.std]
git = "https://github.com/japaric/steed"  # `path` works too
stage = 1
```

```
# Make sure to always pass `--target`, even if you are compiling for the HOST
# (you could use `cross build` here if you are not building an executable)
# NOTE `steed` uses its own set of targets; these have `steed` instead of `gnu`
# in their triples
$ cross run --target x86_64-unknown-linux-steed
Hello, world!

$ file target/x86_64-unknown-linux-steed/debug/hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, not stripped
```

You can use `cross` to cross compile your crate to other architectures as well.

```
# continuation from the previous example
$ cross build --target aarch64-unknown-linux-steed

$ file target/aarch64-unknown-linux-steed/debug/hello
hello: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV),statically linked, not stripped
```

`cross` can even transparently execute those cross compiled binaries using QEMU.

```
$ cross run --target aarch64-unknown-linux-steed -v
       Fresh hello v0.1.0 (file:///project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `/target/aarch64-unknown-linux-steed/debug/hello`
Hello, world!
```

### On other systems

If you are not using a x86_64 Linux system or don't want to use/install Docker,
then you can use Xargo to compile your program against `steed`.

```
# if you don't have Xargo installed
# (Xargo v0.3.4 or newer is required)
$ cargo install xargo 

# grab the target specification file for the `steed` target
$ curl -OL https://github.com/japaric/steed/raw/master/docker/x86_64-unknown-linux-steed.json

# required to compile some of `steed` dependencies
$ export RUST_TARGET_PATH=$(pwd)

# this is the part that replaces `std` with `steed`
$ edit Xargo.toml && cat $_
```

``` toml
[dependencies.collections]  # `steed` depends on collections

[dependencies.std]
git = "https://github.com/japaric/steed"  # `path` works too
stage = 1
```

```
$ xargo run --target x86_64-unknown-linux-steed
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/x86_64-unknown-linux-steed/debug/hello`
Hello, world!

$ file target/x86_64-unknown-linux-steed/debug/hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, not stripped
```

You can cross compile as well but you'll have to install a cross linker on your
host system:

```
# assuming Ubuntu
$ sudo apt-get install gcc-aarch64-linux-gnu

# point Cargo to the right linker
$ export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_STEED_LINKER=aarch64-linux-gnu-gcc

# grab the target specification file for the `steed` target
$ curl -OL https://github.com/japaric/steed/raw/master/docker/aarch64-unknown-linux-steed.json

# required to compile some of `steed` dependencies
$ export RUST_TARGET_PATH=$(pwd)

$ xargo build --target aarch64-unknown-linux-steed

$ file target/aarch64-unknown-linux-steed/debug/hello
hello: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV),statically linked, not stripped
```

You can execute the cross compiled binaries too but you'll have to install and
explicitly call QEMU:

```
# assuming Ubuntu
$ sudo apt-get install qemu-user

$ qemu-aarch64 target/aarch64-unknown-linux-steed/debug/hello
Hello, world!
```

## Current functionality

Check the [API docs](https://japaric.github.io/steed/steed/index.html), but to
summarize the functionality that interfaces with the Linux kernel:

- Standard I/O (stdin, stdout, stderr)

- File I/O

- Filesystem operations (`std::fs`)

- Dynamic memory allocation (thanks to [ralloc]!)

- `std::time`

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
