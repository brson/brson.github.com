---
layout: post
title: How Rust is tested
tags: [rust]
---

Rust is a systems programming language with massive ambitions. It is
designed for creating the most reliable software, from the tiniest
embedded systems to multi-million line behemoths, and its users need
to have confidence that it is fit for purpose.

In service of this ambition Rust has an extremely thorough testing
regimen, and that is one of the things I am most proud of about Rust.
Rust has a strict continuous integration system that runs a great
number of tests on every pull request, basically guaranteeing that the
Rust master branch always works; which is crucial because Rust
releases nightly builds every night, and stable builds every six
weeks. And Rust further tests every release against its entire open
source library ecosystem.

I've always admired well-tested software projects, like [SQLite], and
aim to place Rust among the pantheon of the best. This document then,
is a catalog of all the ways we test Rust. I hope it provides insight
into what it takes to deliver a production-quality programming
language, a hint at the wide variety of techniques employed in
software validation, and that it reinforces your confidence in Rust's
reliability.

## Summary (2017/07/10)

- All patches built in 58 configurations before landing
- All patches tested in 20 configurations before landing
- 126k tests per PR across all platforms
- Full release artifacts published for 48 platforms every merge
- Testing of all documentation
- Releases every six weeks
- 13k Rust projects regression-tested before releases
- 222k tests in Rust projects tested before releases
- Fuzz testing and formal verification

## Contents

- [The promise of Rust](#s-pr)
- [Continuous integration](#s-ci)
- [Continuous releases](#s-cr)
- [The Rust test suite](#s-ts)
  - [Unit tests](#s-ut)
  - [compiletest](#s-ct) - The Rust test suite test driver
  - [run-pass and run-fail](#s-rpf)
  - [compile-fail](#s-cf)
  - [ui](#s-ui)
  - [run-make](#s-rm)
  - [codegen](#s-cg)
  - [codegen-units](#s-cgu)
  - [pretty](#s-p)
  - [debuginfo](#s-dbg)
  - [incremental](#s-inc)
  - [mir-opt](#s-mir)
  - [rustdoc](#s-rd)
  - [documentation](#s-doc)
  - [linkchecker](#s-link)
  - [cargotest](#s-crt)
  - [check-error-index](#s-err)
  - [distcheck](#s-dist)
  - [tidy](#s-tidy)
  - [bootstrap](#s-boot) - Build system self-check
  - [tools](#s-tools)
- [Other testing](#s-ot)
  - [cargobomb and crater](#s-ds) - Downstream testing
  - [libc](#s-libc)
  - [compiler-builtins](#s-cb)
  - [perf.rust-lang.org](#s-perf)
  - [fuzzing](#s-fuzz) - afl.rs and cargo-fuzz
  - [rust-icci](#s-icci)
  - [Firefox beta testing](#s-fx)
  - [Formal verification](#s-form)
  - [smoke](#s-smoke) - Cross-platform testing
  - [rustup](#s-ru)
  - [clippy](#s-clippy)
- [Notes and future work](#s-f)

<a name="s-pr"></a>

## The promise of Rust

Rust is a systems programming language with massive ambitions. It is
designed for creating the most reliable software, from the tiniest
embedded systems to multi-million line behemoths, and its users need
to have confidence that it is fit for purpose.

Today Rust runs on [many platforms], and tomorrow it will run on many
more. Rust will one day run on any machine with a microprocessor.

And Rust moves fast, with [releases every 6 weeks], and a growing
ecosystem of software depending on it. Rust makes [strong guarantees]
about compatibility and stability, what will, won't, and might break
as the plattform evolves, and ensuring that we fulfill those
guarantees is crucial to maintaining the trust of Rust's users.

It's a big challenge to keep it all from falling apart. This is how we
do it.

We use strong _continuous integration_ to catch many bugs before they
are ever committed to the Rust repository, and _continuous releases_
to enable more extensive testing of nightly and beta builds prior to
the release every 6 weeks.

During CI, all patches must pass the Rust test suite in all supported
configurations before landing. Nightly and beta releases are subject
to further testing using a number of techniques.

<a name="s-ci"></a>

## Continuous integration

Rust relies on [continuous integration], where the code base is tested
as part of the process of reviewing and merging patches.

In Rust we do CI in a very particular way, and one which we are very
proud of. Rust's creator, Graydon, originally described it in a blog
post, ["The Not Rocket Science Rule"][science]. The thing we do
differently from most is that we run the full test suite against every
patch, as if it were merged to master, _before_ committing it into the
master branch, whereas most CI setups test _after_ committing, or if
they do run tests against every PR, they do so before merging, leaving
open the possibility of regressions introduced during the merge.

How does this work precisely? Our integration bot, [bors], maintains a
[queue of all pull requests][borsq] that have been reviewed and
approved for landing. It proceeds through this queue, one at a time:
for each pull request it merges the pull request branch with the
master branch, but into a temporary branch (which we call "auto");
then it runs the entire test suite on that branch, in many
configurations (as of 2017/07/10 there are 59 configurations built,
and 20 tested). If all tests pass in all configurations only then does
that commit become the master branch. Then bors moves onto the next
PR.

The important things to recognize about this arrangement are: first,
the head commit on Rust's master branch is guaranteed to be fully
tested at all times; but as a consequence, landing pull requests to
Rust is completely serialized - only one PR can be under final testing
at a time.

The benefit of this arrangement is that Rust's developers have high
confidence that the master branch works correctly, always. It isn't
perfect - bugs do slip through - but it does provide significant peace
of mind.

There's a big downside though in that landing patches to Rust is
serialized on running the test suite on every patch, and it takes a
particularly long time to Run the Rust test suite in all the
configurations we care about. Today the longest-running configuration
takes over 2 hours. Rust always has a queue of approved patches
waiting to land, and so it can take days for even simple patches to
get through the queue.

This can create an interesting competitive environment where authors
desire high spots in the queue and complain when the queue grows.

While PR authors are waiting in bors's queue, we also have a bot
test their PR in a single configuration as a smoke test. This avoids
the frustration of waiting for one's PR to work through the queue
only to be rejected by a simple mistake.

We do this style of CI not only on [rust-lang/rust] but also on other
key projects, including [cargo] and [rustup].

bors's implementation has gone through several iterations, and today
it is implemented by a script called [homu], which is shared
with Rust's sister project, [servo]. It has also inspired other
ther integration bots used in the Rust ecosystem and beyond,
including [bors-ng].

Test-first CI is the cornerstone of Rust stability.

Today we do our CI on [Travis CI] for Linux and Mac OS, and [AppVeyor]
for Windows. Notably, all of our testing hosts are running x86, and
so, for the non-x86 platforms that we do test, we currently use the
[QEMU] emulator on Linux. Many non-x86 platforms do not yet get test
coverage as part of the CI process (though see the [smoke](#s-smoke)
project). No non-x86 platforms are automatically tested on real
hardware, a major limitation of the current setup.

<a name="s-cr"></a>

## Continuous releases

Since we test so thoroughly, ideally all bugs are caught before
entering the tree, but that is not the case. Once a patch enters the
tree the clock is ticking until it hits the stable release.

Rust publishes releases on three "channels": nightly, beta, and stable.
The nightly and beta channels provide an opportunity to catch bugs
missed by the official CI before they hit a release.

Rust has a 6 week beta cycle, so the minimum a patch will sit in tree
before hitting a stable release is 6 weeks (if it lands on master
right before the next beta), and the maximum is 12 weeks (if it lands
at the beginning of a cycle).

During the 6 week release cycle, regression triage meetings are held
every 2 weeks to keep ahead of breakage and ensure the release stays
on track. Again, the Rust release schedule is incredibly aggressive
and it takes constant vigilence to maintain.

To make releases as simple as possible we tie it directly to the CI
system. Since we are already testing Rust in the same configurations
that we ship releases for, it's a natural extension to simply produce
release artifacts while we're testing. So we do that and publish
them to their own S3 bucket.

From those binaries we have a bot that periodically collects them into
their final form for release and deploys them to static.rust-lang.org.

So not only do we fully test every commit that lands on master, we
also publish the complete release binaries at the same time.

With as many release configurations as Rust has, differences between
the continuous integration configuration and the release build
configuration became a huge source of problems. Making testing and
releasing the same thing eliminated them.

<a name="s-ts"></a>

## The Rust test suite

The Rust test suite covers rustc, std, cargo, rustdoc, and the Rust
documentation, and features a number of special test harnesses to
cover specific classes of bugs relevant to Rust.

The tests are all ultimately run by [libtest], the standard test
crate, and many of them are coordinated by the [compiletest] tool.

As of 2017/07/10 there are a little over 6k tests in the Rust test
suite. This may seem surprisingly few, but keep in mind that Rust's
strong static typing prevents many errors at compile-time, so Rust
projects in general are believed to require fewer test cases than
projects in other languages. All features and bug fixes are
accompanied by test cases. Still, test coverage is currently unknown,
and assumed to be far from complete.

<a name="s-ut"></a>

### Unit tests

Standard Rust unit tests, like any Rust author would write. These are
tests annotated with `#[test]` and run with `cargo test`. This is the
primary method for testing the standard library, and for cargo as
well.

This is the most basic kind of testing in Rust, but the compiler
itself is mostly tested with more specialized tools.

<a name="s-ct"></a>

### compiletest

compiletest is the main test harness of the Rust test suite, one of
the oldest Rust codebases, and the oldest parallel Rust program. It
implements a number of classes of test, each of which generally
corresponds to a directory under [`src/test`]. Most of these tests
involve driving the compiler to compile one or more source programs,
then interpreting the results.

Most tests are represented by a Rust source file that may have
annotations in comments directing compiletest in how to run the test,
as in the following run-fail test, `test-panic.rs`, that tells
compiletest that the string "thread 'test_foo' panicked at" must be
printed to stdout, to compile with the `--test` flag, and to ignore
the test on emscripten.

```rust
// check-stdout
// error-pattern:thread 'test_foo' panicked at
// compile-flags: --test
// ignore-emscripten

#[test]
fn test_foo() {
    panic!()
}
```

Most of the test types in the following sections are implemented
by compiletest.

<a name="s-rpf"></a>

### run-pass and run-fail

Two of the oldest and simplest types of test. These test cases are
Rust source code that should compile successfully. run-pass tests must
run successfully; run-fail tests must run and return the Rust standard
error code, 101.

<a name="s-cf"></a>

### compile-fail

compile-fail tests are some of the most common tests for testing
language features. They must fail to compile, and stderr must
emit errors containing specific text:

```rust
fn main() {
    1 = 2; //~ ERROR invalid left-hand side expression
    1 += 2; //~ ERROR invalid left-hand side expression
    (1, 2) = (3, 4); //~ ERROR invalid left-hand side expression

    let (a, b) = (1, 2);
    (a, b) = (3, 4); //~ ERROR invalid left-hand side expression

    None = Some(3); //~ ERROR invalid left-hand side expression
}
```

While these are the bread and butter of rustc testing, their weakness
is that they only validate some of the text output to stderr - they do
not guarantee that the output actually looks good. For that reason new
tests, or tests of tricky error output, are often written as 'ui'
tests.

<a name="s-ui"></a>

### ui

UI tests are like compile-fail tests, but the output of stdout and
stderr are checked fully. In these tests, instead of annotating the
source with expected output, source files are accompanied by
`*.stdout` and `*.stderr` templates that capture expactly what the
compiler is allowed to output.

So the test `issue-39544.rs`:

```rust
enum X {
    Y
}

struct Z {
    x: X
}

fn main() {
    let z = Z { x: X::Y };
    let _ = &mut z.x;
}
```

must have the output from `issue-39544.stderr`:

```rust
error: cannot borrow immutable field `z.x` as mutable
  --> $DIR/issue-39544.rs:21:18
   |
21 |     let _ = &mut z.x;
   |                  ^^^

error: aborting due to previous error
```

Because these templates are difficult to write by hand, there are
scripts to help keep them updated when the output changes.

<a name="s-rm"></a>

### run-make

This is the kind of test you write when none of the other tests
are sufficient. compiletest simply runs a Makefile, while setting
up a bunch of environment variables that might be useful. Tests
import [tools.mk] to get access to various helpers.

These tests are usually testing the behavior of rustc, like in this
test that is validating something related to target specs:

```Makefile
-include ../tools.mk
all:
	$(RUSTC) foo.rs --target=my-awesome-platform.json --crate-type=lib --emit=asm
	grep -q -v morestack < $(TMPDIR)/foo.s
	$(RUSTC) foo.rs --target=my-invalid-platform.json 2>&1 | grep -q "Error loading target specification"
	$(RUSTC) foo.rs --target=my-incomplete-platform.json 2>&1 | grep 'Field llvm-target'
	RUST_TARGET_PATH=. $(RUSTC) foo.rs --target=my-awesome-platform --crate-type=lib --emit=asm
	RUST_TARGET_PATH=. $(RUSTC) foo.rs --target=x86_64-unknown-linux-gnu --crate-type=lib --emit=asm
	$(RUSTC) -Z unstable-options --target=my-awesome-platform.json --print target-spec-json > $(TMPDIR)/test-platform.json && $(RUSTC) -Z unstable-options --target=$(TMPDIR)/test-platform.json --print target-spec-json | diff -q $(TMPDIR)/test-platform.json -
```

<a name="s-cg"></a>

### codegen

codegen tests verify that rustc produces the expected LLVM IR for a given file.
It is similar to tests used by LLVM to verify its own lowering to assembly.

```rust
// compile-flags: -C no-prepopulate-passes

#![crate_type = "lib"]

pub enum E {
    A,
    B,
}

// CHECK-LABEL: @exhaustive_match
#[no_mangle]
pub fn exhaustive_match(e: E) {
// CHECK: switch{{.*}}, label %[[DEFAULT:[a-zA-Z0-9_]+]]
// CHECK: [[DEFAULT]]:
// CHECK-NEXT: unreachable
    match e {
        E::A => (),
        E::B => (),
    }
}
```

<a name="s-cgu"></a>

### codegen-units

Like codegen tests these tests are validating the internal compiler
behavior, in this case testing how items in Rust source are
partitioned between "codegen units" at translation time. Correctly
subdividing the work of translating Rust to LLVM and then machine code
is required for incremental translation.

```rust
// compile-flags:-Zprint-trans-items=eager

pub static FN : fn() = foo::<i32>;

pub fn foo<T>() { }

//~ TRANS_ITEM fn static_init::foo[0]<i32>
//~ TRANS_ITEM static static_init::FN[0]

fn main() { }

//~ TRANS_ITEM fn static_init::main[0]
//~ TRANS_ITEM drop-glue i8
```

<a name="s-p"></a>

### pretty

pretty tests are testing rustc's "pretty printer", the code that
converts the Rust AST back to Rust syntax. Rust uses the pretty
printer for displaying error messages.

pretty testing is done by asking the compiler to pretty-print source
code, then asking it to pretty-print _that_ source code, and testing
that it reaches a steady state, where further pretty-printing produces
the same soruce, and that it still works.

Historically, rustc has run pretty testing over the entire codebase,
but as the pretty-printer matured, pretty testing was reduced to a
subset of the full tree.

<a name="s-dbg"></a>

### debuginfo

The problem of getting debuggers to understand the compiler's output
is nearly as difficult as just getting the compiler to work in the
first place. Debuggers are so complicated. These tests drive gdb and
lldb to verify they work with the given Rust program.

Here's a simple example from `cross-crate-spans.rs`:

```rust
#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

// min-lldb-version: 310

// aux-build:cross_crate_spans.rs
extern crate cross_crate_spans;

// compile-flags:-g


// === GDB TESTS ===================================================================================

// gdb-command:break cross_crate_spans.rs:24
// gdb-command:run

// gdb-command:print result
// gdbg-check:$1 = {__0 = 17, __1 = 17}
// gdbr-check:$1 = (17, 17)
// gdb-command:print a_variable
// gdb-check:$2 = 123456789
// gdb-command:print another_variable
// gdb-check:$3 = 123456789.5
// gdb-command:continue

// gdb-command:print result
// gdbg-check:$4 = {__0 = 1212, __1 = 1212}
// gdbr-check:$4 = (1212, 1212)
// gdb-command:print a_variable
// gdb-check:$5 = 123456789
// gdb-command:print another_variable
// gdb-check:$6 = 123456789.5
// gdb-command:continue



// === LLDB TESTS ==================================================================================

// lldb-command:b cross_crate_spans.rs:24
// lldb-command:run

// lldb-command:print result
// lldb-check:[...]$0 = (17, 17)
// lldb-command:print a_variable
// lldb-check:[...]$1 = 123456789
// lldb-command:print another_variable
// lldb-check:[...]$2 = 123456789.5
// lldb-command:continue

// lldb-command:print result
// lldb-check:[...]$3 = (1212, 1212)
// lldb-command:print a_variable
// lldb-check:[...]$4 = 123456789
// lldb-command:print another_variable
// lldb-check:[...]$5 = 123456789.5
// lldb-command:continue


// This test makes sure that we can break in functions inlined from other crates.

fn main() {

    let _ = cross_crate_spans::generic_function(17u32);
    let _ = cross_crate_spans::generic_function(1212i16);

}
```

<a name="s-inc"></a>

### incremental

Tests of incremental compilation, a technique rustc uses to track
within a crate which source code needs to be recompiled when any other
source code changes.

In the example below, the test is run twice, first with `--cfg
rpass1`, then again with `--cfg rpass2`. The test instructs the
compiler to verify that the change in source code between the two
invocations results in only mod `x` being recompiled. Other modules
are reused.

```rust
// A first "spike" for incremental compilation: here, we change the
// content of the `make` function, and we find that we can reuse the
// `y` module entirely (but not the `x` module).

// revisions:rpass1 rpass2

#![feature(rustc_attrs)]

#![rustc_partition_reused(module="spike", cfg="rpass2")]
#![rustc_partition_translated(module="spike-x", cfg="rpass2")]
#![rustc_partition_reused(module="spike-y", cfg="rpass2")]

mod x {
    pub struct X {
        x: u32, y: u32,
    }

    #[cfg(rpass1)]
    fn make() -> X {
        X { x: 22, y: 0 }
    }

    #[cfg(rpass2)]
    fn make() -> X {
        X { x: 11, y: 11 }
    }

    pub fn new() -> X {
        make()
    }

    pub fn sum(x: &X) -> u32 {
        x.x + x.y
    }
}

mod y {
    use x;

    pub fn assert_sum() -> bool {
        let x = x::new();
        x::sum(&x) == 22
    }
}

pub fn main() {
    y::assert_sum();
}
```

<a name="s-mir"></a>

### mir-opt

Tests of optimizations on MIR, rustc's internal Rust
representation. These tests check that the MIR looks as expected
before and after given optimization passes.

Note that as of 2017/07/10 MIR optimizations are not enabled by
default.

```rust
fn main() {
    if false {
        println!("hello world!");
    }
}

// END RUST SOURCE
// START rustc.node4.SimplifyBranches-initial.before.mir
// bb0: {
//     switchInt(const false) -> [0u8: bb2, otherwise: bb1];
// }
// END rustc.node4.SimplifyBranches-initial.before.mir
// START rustc.node4.SimplifyBranches-initial.after.mir
// bb0: {
//     goto -> bb2;
// }
// END rustc.node4.SimplifyBranches-initial.after.mir
```

<a name="s-rd"></a>

### rustdoc

This set of tests verifies that the HTML output of rustdoc includes
various properties. Again, they are run by compiletest by interpreting
comments in Rust source files. They look something like:

```rust
#![crate_type="lib"]

#![feature(const_fn)]

pub struct Foo;

impl Foo {
    // @has const/struct.Foo.html '//*[@id="new.v"]//code' 'const unsafe fn new'
    pub const unsafe fn new() -> Foo {
        Foo
    }
}
```

<a name="s-doc"></a>

### documentation

Rust uses Markdown for both API documentation and standalone
documentation. All examples in Rust documentation are tested by
rustdoc, the documentation tool.

<a name="s-link"></a>

### linkchecker

The [linkchecker] ensures that all internal HTML links produced by
rustdoc for the standard library documentation are valid.

<a name="s-crt"></a>

### cargotest

[cargotest] is a small tool that runs the test suite of several
significant out-of-tree Rust projects. The projects are chosen to have
a wide variety of dependencies to maximize the chances of detecting
type system regressions through build failures.

This suite of tests was added in anger after cargo's own build broke
too many times due to Rust regressions.

<a name="s-err"></a>

### check-error-index

rustc's error messages included extended help messages that often
include examples. As with other documentation examples, these are
tested.  A special tool is used to extract them from the compiler,
convert them to markdown, and then run them through rustdoc.

<a name="s-dist"></a>

### distcheck

Every release of Rust is accompanied by a tarball containing the
source. This check verifies that that source tarball unpacks, builds
and tests as expected.

<a name="s-tidy"></a>

### tidy

The [tidy] tool runs a variety of sanity checks that the source tree
conforms to various conventions:

- *bins* - verifies that no binaries are checked in
- *style* - verifies line length, no tabs, no trailing whitespace, no
  CR characters, no TODO or XXX directives, and that each file
  contains a license header
- *errors* - verifies that rustc error codes are not duplicated
- *cargo* - verifies that crates listed in `[dependencies]` are
  actually imported as `extern crate`
- *features* - verifies that various properties of Rust's 'feature'
  definitions are correct
- *pal* - verifies that platform-specific code only occurs in
  specific places
- *unstable_book* - verifies that unstable features are represented
  in the documentation
- *deps* - verifies the license of third-party crate dependencies

<a name="s-boot"></a>

### bootstrap

The Rust build system (called "bootstrap") itself is written in Rust
and has a self-check that runs before the build begins.

<a name="s-tools"></a>

### tools

Rust is distributed with several externally-developed tools, today
cargo and rls (the Rust Language Server). Their test suites are
included as part of the Rust test suite.

<a name="s-ot"></a>

## Other testing

All of the above sections describe the Rust test suite, which is Run
prior to landing any patch. But that is neither the beginning nor the
end of the Rust validation process. Below are yet more tools we use to
test other aspects of the Rust platform. This includes tools and
libraries Rust depends on, and the application of additional
validation to releases during their nightly and beta phases, prior to
their stable release.

<a name="s-ds"></a>

### cargobomb and crater

In would be lovely if the Rust test suite caught all regressions.
Sadly, the authors of the test suite haven't yet figured out how to
anticipate every way in which Rust will be used in the wild.

Fortunately though, Rust has a standard testing facility that most
Rust crates use, and Rust has a standard repository of Rust crates in
crates.io. GitHub further contains repositories of Rust crates that
are not published to crates.io.

These factors allow us to treat the entire world of open source Rust
code as our test suite.

As new nightlies and betas are published, we use the [cargobomb] tool
to test this corpus of Rust code (as of 2017/07/10 over 13,000 crates)
against both the stable release and a nightly or beta release,
comparing the results for regressions.

This type of testing helps us find subtle changes in the type system
as crates fail to build on new releases, errors in code generation or
library behavior that cause tests to fail or crash, and logic errors
in the compiler that cause the compiler to crash.

[Here's an example of a cargobomb run against nightly][cargobomb-run].
Clicking the red "regressed" button reveals the most important
category of results: those where the build against the second
toolchain failed earlier than the first toolchain. Clicking through
the results yields the full logs. In this run, 6972 of the 13055
crates tested passed their test suites against both toolchains; 56
crates regressed; and 3985 crates failed to even build (this can be
for many reasons, but is often simply because the cargobomb
environment is not set up appropriately for the crate).

As of 2017/07/10, each run of cargobomb runs over 222,000 tests, twice
over, across more than 13,000 projects.

When cargbomb detects regressions, they are filed against
rust-lang/rust, and the downstream crate authors alerted to the
issue. Using the binary builds published for every successful PR, the
PR that caused the regression is quickly identified by the
[bisect-rust] tool.

As cargobomb-discovered regressions are fixed, naturally test cases
capturing them are checked in to prevent those regressions occurring
again in the future.

cargobomb is the successor to a similar project called [crater], which
only checks for successful typechecking, and does not run test suites.
It is still in occasional use.

<a name="s-libc"></a>

### libc

The [libc] crate has a special place in the ecosystem: it defines the
FFI definitions of the C library and related systems libraries for
every platform Rust supports, and ends up used in some form by most
every Rust project. So it's important that it be correct.

But writing correct FFI bindings is not a simple thing, since one does
not have the benefit of the Rust type checker verifying the
definitions. So the libc crate has a special testing regime designed
to accomplish this verification.

It uses the [ctest] crate to automatically compare the libc Rust
bindings to the actual C definition.

The basic process involves generating two programs, one in C, and one
in Rust, that each produce metadata about function signatures,
constant values, struct layout, alignment, and more. The Rust program
calls into the C program and then compares that both sides produce the
same metadata values.

<a name="s-cb"></a>

### compiler-builtins

This touches on a little known piece of black magic in comon compiler
toolchains. One might expect that C compilers like gcc and clang
compile your code directly to machine code, but in actuality, there
are a number of constructs defined in the language which do not
necessarily correspond neatly to machine instructions, depending on
the architecture. As a consequence, nearly all C programs silently
link to a tiny runtime library to provide implementations of very low
level operations. In gcc this library is called [libgcc], and in clang
it is called [compiler-rt], which began essentially by
reverse-engineering libgcc.

LLVM will silently lower LLVM IR to calls to functions provided by
compiler-rt, so rustc, like clang, must ensure these functions are
available.

This library is implemented in a combination of assembly and C, and
has over time grown to include a variety of runtime functionality
expected by LLVM for sometimes niche features.

Over the years we've found it quite challenging to keep compiler-rt
working on all platforms Rust (and LLVM) supports, so are in the
gradual process of reimplementing it ourselves (in Rust), in a library
called [compiler-builtins].

The compiler-builtins crate is exclusively limited to basic math
intrinsics, and does not include many of the advanced runtime features
of compiler-rt, notably the sanitizer runtimes (which in Rust we would
expect to be provided in a different crate).

To verify this set of crucial low-level math functions behaves as
expected the compiler-builtin crate has its own custom testing. For
each function there is a test that generates a set of inputs to it,
and passes it to both the upstream compiler-rt, and to our own
compiler-builtins, and verifies their same output. The test suite
additionally compares the symbols exported by each to ensure our
implementation does not miss any functions.

<a name="s-perf"></a>

### perf.rust-lang.org

It's a long-standing desire of the team to get a stronger handle on
rustc's compile times, and to prevent them from increasing.
[perf.rust-lang.org] is a project that uses the CI builds of Rust to
build a variety of representative Rust code bases and record how long
it took, and how much memory was used. The compiler team then uses the
reporting provided by the site to identify performance regressions.

<a name="s-fuzz"></a>

### fuzzing

There are two major fuzzers for Rust: [afl.rs], which uses the
[American Fuzzy Lop][afl] fuzzer; and [cargo-fuzz], which uses LLVM's
[libfuzzer]. Both project collaborate under the [Rust Fuzzing
Authority][rust-fuzz] to improve the Rust crate ecosystem. They
maintain a [trophy case] of bugs discovered through their tools.
Although an important part of the Rust toolset, fuzzing Rust tends to
find fewer and less sever bugs than fuzzing software written in
memory-unsafe languages.

<a name="s-icci"></a>

### rust-icci

The Rust compiler supports a technique called "incremental
compilation", a process by which the compiler tracks, on a very
fine-grained level, dependencies between items in the code, to avoid
re-typechecking and re-translating fetaures that have not changed
since previous compilations. It can be thought of roughly as caching
and reusing previously translated functions. The system is extremely
complex and opportunities for errors are innumerable.

[rust-icci] is a project that does brute-force validation of Rust
incremental compilation by replaying the commit history of Rust
projects with incremental compilation activitated, and without, and
verifies that the results are the same.

<a name="s-fx"></a>

### Firefox beta testing

One of the major initial users of Rust is the Firefox web browser,
which has integrated Rust into a number of subsystems of a large C++
codebase that supports a variety of platforms across millions of
client systems. This real-world exposure quickly began to expose
regressions in Rust, and so Firefox is [built and tested against Rust
betas][fx] as they are produced, providing important feedback during
the Rust beta cycle.

<a name="s-form"></a>

### Formal verification

Ultimately we hope and expect Rust to be adopted by industries that
traditionally require strong guarantees about reliability and
correctness. These industries are conservative and slow to change, and
tend to require large accumulations of evidence of correctness; and
formal proofs provide some of the strongest such
evidence. Furthermore, having formal models of the Rust system gives
the Rust developers a framework to confidently apply advanced
optimizations to Rust that are not yet possible.

The most prominent project applying formal methods to validate Rust is
the [Rust Belt] project, which aims to prove the Rust type system
sound, and to develop a model for reasoning about unsafe code in Rust.
It has thus far [proven the soundness][snd] of the most common
abstractions in the standard library, discovering [one soundness
bug][sndb] in the process.

Previously, Eric Reed formalized a safe subset of Rust called
[Patina], and the [CRUST] project used bounded model checking to find
memory safety violation.

<a name="s-smoke"></a>

### smoke

Many of the platforms that Rust supports, though they may be _built_
by Rust's CI, are not _tested_ by Rust's CI, either because of lack of
capacity, or because it would take too long (often due to emulation).

The [smoke] project is an independent CI project to run tests against
some of these platforms using emulators.

Note that as of 2017/07/10 the smoke project is bitrotted and needs to
be refreshed.

<a name="s-ru"></a>

### rustup

Though not strictly a part of the Rust distribution, [rustup] is a
critical piece of the Rust distribution infrastructure. It is the tool
that installs Rust for most Rust users. While limited in scope, in
performs important network and and file I/O that must be reliable
across all supported platforms. rustup is self-updating, and a
self-update failure would cause major disruption for Rust users.

rustup's primary functionality works by downloading a release
"manifest" from the official Rust servers, and interpreting it
to install the requested Rust releases.

While its test scaffolding includes a mock distribution server, it
notably it does not contain actual network tests. In general, rustup's
testing could be much more thorough.

<a name="s-clippy"></a>

### clippy

[clippy] provides additional static analysis (called "lints") beyond
what is done by the compiler. While not distributed by the compiler,
and rarely run on the Rust codebase itself, it is widely used by the
Rust community to provide an extra level of consistency and polish, so
seems worth mentioning.

<a name="s-f"></a>

## Notes and future work

So Rust receives a lot of testing, but there's more that can be
done. Here we'll say a little bit about other possibilities. Almost
everything here would be an awesome project for an ambitious
contributor to undertake!

An obvious omission is the lack of any testing with tools like
[valgrind] and [address sanitizer]. These tools are incredibly
effective at finding incorrect memory accesses by instrumenting
programs at runtime, so with Rust's focus on memory safety one might
expect them to be employed extensively. They are not though.

Valgrind was a crucial tool in Rust's original development, back when
Rust barely worked. There were several years where the entire test
suite was required to be valgrind-clean, and it's hard to imagine
bringing up Rust without it. At some point though LLVM began doing a
valid optimization that valgrind was unable to recognize as valid, and
that made its application useless for us, or at least too difficult to
maintain. By that point Rust was mature and memory safe, so valgrind
testing was disabled, and eventually it bitrotted and was removed from
the tree.

Unlike valgrind, address sanitizer requires the help of the compiler
to instrument binaries, and that support landed in LLVM long after
Rust had matured.

Today rustc's code generation and the abstractions in the standard
library are mature and well-trusted, and it's rare for either of them
to create the kind of errors these tools detect, so nobody feels it a
pressing matter to reenable this kind of testing.

We do not measure test coverage of the Rust compiler, and have no
conception of how well it is covered. Integration of coverage
reporting into Rust CI is an obvious and easy improvement.

The Rust source tree contains a poorly-maintained definition of the
Rust language [grammar], along with the facilities for testing that
grammar against the compiler's production parser. These tests have
never been activated as part of Rust's CI though and as a result the
grammar is neither complete nor authoritative.

The Rust installers, of which there are several different types, are
not automatically tested, and there is a notable lack of human-driven
QA during the Rust release process. It would be greatly desirable to
automate the deployment and installation of Rust in a variety of
scenarios via a test environment prior to release.

In the future we expect to develop a formal definition of Rust's
memory model, which will define what is and isn't allowed to be done
in unsafe blocks, and also what the compiler is allowed to do during
optimization. The memory model will be defined in such a way that Rust
code can be instrumented to validate conformance, and we will run
conformance tests across the entire ecosystem with the cargobomb tool.

The Rust standard library is intended to provide a superset of the
core library API, but there is no validation that this is true, and
already there are known places where the two crates diverge in their
APIs. There should be a tool that verifies the superset relationship.

Rust would probably benefit from "chaos engineering", that is,
injecting faults into the runtime and confirming that the system
behaves as expected. Panic and error paths are little exercised and
tend to hide bugs, especially around the interaction between `unsafe`
and unwinding. Of course the standard library is small and
well-reviewed, so the returns of such an effort may be small there,
but it would almost certainly be fruitful to apply the appoach more
broadly in the crate ecosystem.

Rust crates use an interpretation of the semver versioning standard.
Among other things it defines what API changes are allowed between
releases of crates. We expect in the future to provide a tool that
crate authors can use to validate that the API changes in their
releases correspond to semver.

We also expect to provide other tools to help the crate ecosystem
manage their evolution. [cargo-crusader] is one such tool, though it
is not fully-realized. It runs the test suites of a crate's reverse
dependencies (that is, the crates that depend on your crate) prior to
publishing a new version.

We have no tracking of metrics around regression rates from release to
release, despite being vigilant about testing downstream crates. We
should expect in the future to be able to provide firm numbers on
known regressions per release.

The cargobomb technique of testing all known Rust code in the
ecosystem is powerful, but currently limited to running crates' test
suites to detect Rust language regressions. There are other types of
analyses we would like to do in this fashion.

The security of the Rust release process is crucial, and lacking.  The
possibility of the compiler being backdoored, or even the subject of a
"[trusting trust]" attack, is real. In the future we expect to have
reproducible builds, and reproducible releases, meaning that
third-parties can produce the exact same binary releases the Rust
project does.  With that capability we would be able to produce a
simple tool that would let others validate that any individual release
has not been compromised with code that does not originate from the
source tree; and ultimately, to validate that the entire bootstrap
chain has not been compromised by a self-replicating, "trusting trust"
-style backdoor.

Along those same lines, _someday_ we would expect to have multiple
implementations of Rust to validate against the reference compiler,
and to provide further assurance against "trusting trust" via [diverse
double compilation].

rustup should do mock testing using a self-contained http server,
whereas today its mock server is just using a filesystem mimicking the
layout of static.rust-lang.org.

rustup should have a set of self-upgrade tests, run against the live
rustup release archives, and run prior to releases, that ensure that
it can self-upgrade from the original release up to the present
release.

<!-- links -->

[AppVeyor]: https://www.appveyor.com/
[QEMU]: https://en.wikipedia.org/wiki/QEMU
[Rust Belt]: http://plv.mpi-sws.org/rustbelt/
[SQLite]: https://sqlite.org/testing.html
[Travis CI]: https://travis-ci.org/
[`src/test`]: https://github.com/rust-lang/rust/tree/37849a002ed91ac2b80aeb2172364b4e19250e05/src/test
[address sanitizer]: https://github.com/google/sanitizers
[afl.rs]: https://github.com/rust-fuzz/afl.rs
[afl]: http://lcamtuf.coredump.cx/afl/
[bisect-rust]: https://github.com/Mark-Simulacrum/bisect-rust
[bors-ng]: https://github.com/bors-ng/bors-ng
[bors]: https://github.com/bors
[borsq]: https://buildbot2.rust-lang.org/homu/queue/rust
[cargo-crusader]: https://github.com/brson/cargo-crusader
[cargo-fuzz]: https://github.com/rust-fuzz/cargo-fuzz
[cargo]: https://github.com/rust-lang/cargo
[cargobomb]: https://github.com/brson/cargobomb
[cargobomb-run]: http://cargobomb-reports.s3-website-us-west-1.amazonaws.com/nightly-2017-06-12/index.html
[cargotest]: https://github.com/rust-lang/rust/tree/37849a002ed91ac2b80aeb2172364b4e19250e05/src/tools/cargotest
[clippy]: https://github.com/Manishearth/rust-clippy
[compiler-builtins]: https://github.com/rust-lang-nursery/compiler-builtins
[compiler-rt]: https://github.com/rust-lang/compiler-rt
[compiletest]: https://github.com/rust-lang/rust/tree/37849a002ed91ac2b80aeb2172364b4e19250e05/src/tools/compiletest
[continuous integration]: https://en.wikipedia.org/wiki/Continuous_integration
[crater]: https://github.com/brson/taskcluster-crater
[ctest]: https://github.com/alexcrichton/ctest
[diverse double compilation]: https://www.dwheeler.com/trusting-trust/
[fx]: https://bugzilla.mozilla.org/show_bug.cgi?id=1337955
[grammar]: https://github.com/rust-lang/rust/tree/37849a002ed91ac2b80aeb2172364b4e19250e05/src/grammar
[homu]: https://github.com/servo/homu
[libc]: https://github.com/rust-lang/libc
[libfuzzer]: http://llvm.org/docs/LibFuzzer.html
[libgcc]: https://gcc.gnu.org/onlinedocs/gccint/Libgcc.html
[libtest]: https://github.com/rust-lang/rust/tree/37849a002ed91ac2b80aeb2172364b4e19250e05/src/libtest
[linkchecker]: https://github.com/rust-lang/rust/tree/37849a002ed91ac2b80aeb2172364b4e19250e05/src/tools/linkchecker
[many platforms]: https://forge.rust-lang.org/platform-support.html
[perf.rust-lang.org]: http://perf.rust-lang.org/
[releases every 6 weeks]: https://github.com/rust-lang/rfcs/blob/master/text/0507-release-channels.md
[rust-fuzz]: https://github.com/rust-fuzz
[rust-icci]: https://github.com/rust-icci
[rust-lang/rust]: https://github.com/rust-lang/rust
[rustup]: https://github.com/rust-lang/rustup.rs
[science]: http://graydon2.dreamwidth.org/1597.html
[servo]: http://github.com/servo
[smoke]: https://github.com/japaric/smoke
[sndb]: https://www.ralfj.de/blog/2017/06/09/mutexguard-sync.html
[snd]: https://people.mpi-sws.org/~dreyer/papers/rustbelt/paper.pdf
[strong guarantees]: https://blog.rust-lang.org/2014/10/30/Stability.html
[tidy]: https://github.com/rust-lang/rust/tree/37849a002ed91ac2b80aeb2172364b4e19250e05/src/tools/tidy
[tools.mk]: https://github.com/rust-lang/rust/blob/37849a002ed91ac2b80aeb2172364b4e19250e05/src/test/run-make/tools.mk
[trophy case]: https://github.com/rust-fuzz/trophy-case
[trusting trust]: https://www.ece.cmu.edu/~ganger/712.fall02/papers/p761-thompson.pdf
[valgrind]: http://valgrind.org/
[Patina]: https://www.cs.washington.edu/tr/2015/03/UW-CSE-15-03-02.pdf
[CRUST]: http://ieeexplore.ieee.org/document/7371997/
