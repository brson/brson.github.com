---
layout: post
title: First impressions of the Move programming language
tags: [blockchain, move]
---

[Move](https://github.com/move-language/move)
is a new smart contract language that has been gaining momentum,
with a unique set of design choices.
I have been wanting to learn it since being told by a mentor that it
was a language that interested them.

Described in the paper [_Resources: A Safe Language Abstraction for Money_](https://arxiv.org/pdf/2004.05106.pdf),
Move was originally developed for the defunct Libra / Diem project at Facebook,
and is now used by several blockchains,
most prominently
[Aptos](https://github.com/aptos-labs/aptos-core)
and
[Sui](https://github.com/MystenLabs/sui).

With Solana recently [initiating an effort to port Move to its platform][smove],
and myself having some significant familiarity with the Solana codebase,
and enjoying hacking on compilers and their runtimes,
now seems like a good opportunity to see if I can get into Move.

[smove]: https://github.com/solana-labs/move

What I want to do here:

- check out, build, and test the move compiler
- find the learning resources
- find the standard API documentation
- run through a tutorial

I would also like to learn about how the compiler is architected,
and investigate the obstacles to translating Move to LLVM,
but am not going to get that far today.

The first few sections here will be a summary of my thoughts about Move,
based on this experience,
then the remainder will be a stream-of-consciousness journal of what I did.

All the links in this blog post to the Move repo will refer to
[commit f3cba16754a0b986c24ce06b4bf698f86ccc65b9](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9)
so that they don't break in the future.
People reading this in the future might want to navigate on their own to the equivalent files in the master branch.




## Table of contents

- [Summary](#summary)
- [Learnings](#learnings)
- [Questions](#questions)
- [Move documentation](#move-documentation)
- [Building and testing the compiler](#building-and-testing-the-compiler)
- [Running `dev_setup.sh`](#running-dev_setupsh)
- [Debugging `dev_setup.sh`](#debugging-dev_setupsh)
- [Extending the prover timeout](#extending-the-prover-timeout)
- [Running the tutorial: steps 1 and 2](#running-the-tutorial-steps-1-and-2)
- [Running the tutorial: steps 3 and 4](#running-the-tutorial-steps-3-and-4)
- [Running the tutorial: steps 5 and 5](#running-the-tutorial-steps-5-and-6)
- [Writing a proof!](#writing-a-proof)
- [Solana speculation](#solana-speculation)




## Summary

I had a lot of fun working through the move tutorial!

The tutorial did a pretty great job,
for such a young project,
of leading me through concepts;
the per-chapter exercises great &mdash;
I want to be challenged to learn things on my own,
and these lead me to learn useful things.
There was less hands-on hacking in the tutorial than I expected though.

I submitted a [pull request](https://github.com/move-language/move/pull/488)
fixing minor problems I saw in the tutorial.

Move is obviously influenced by Rust in many ways,
with ownership and borrowing,
but also in its tooling.

Some move source from the tutorial:

```move
    struct Coin<phantom CoinType> has store {
        value: u64
    }

    struct Balance<phantom CoinType> has key {
        coin: Coin<CoinType>
    }

    public fun setup_and_mint(account: &signer, amount: u64) {
        BasicCoin::publish_balance<MyOddCoin>(account);
        BasicCoin::mint<MyOddCoin>(signer::address_of(account), amount, MyOddCoin {});
    }

    public fun transfer(from: &signer, to: address, amount: u64) {
        // amount must be odd.
        assert!(amount % 2 == 1, ENOT_ODD);
        BasicCoin::transfer<MyOddCoin>(from, to, amount, MyOddCoin {});
    }
```

Move looks like a relatively small language,
with a bunch of restrictions &mdash; there is no dynamic dispatch &mdash;
and a carefully-considered collection of type system features &mdash;
generics, linear types, capabilities &mdash;
that to me looks tantalizing to play with,
to see what patterns can or can't be used with Move.
For a Rust programmer, Move looks fun.
I can imagine it being infurating for those not experienced with restrictive static type systems.

The Move tooling is surprisingly polished for such a young language,
with testing, test _coverage_, and a simple _theorem prover_ built in.
It even has a package manager that I did not explore.
Error messages were mostly actionable.
I only had one that wasn't helpful,
and it was a parser error,
parsers often having difficulty suggesting useful remediations.

Check out this error,
which led me straight to the solution to my bug:

```
$ move prove
[INFO] preparing module 0xcafe::BasicCoin
[INFO] transforming bytecode
[INFO] generating verification conditions
[INFO] 6 verification conditions
[INFO] running solver
[INFO] 0.010s build, 0.002s trafo, 0.004s gen, 0.997s verify, total 1.014s
error: abort not covered by any of the `aborts_if` clauses
   ┌─ ./sources/BasicCoin.move:49:5
   │
34 │           borrow_global<Balance<CoinType>>(owner).coin.value
   │           ------------- abort happened here with execution failure
   ·
49 │ ╭     spec transfer {
50 │ │         let from_address = signer::address_of(from);
51 │ │         let from_balance = global<Balance<CoinType>>(from_address).coin.value;
52 │ │         aborts_if from_balance < amount;
53 │ │
54 │ │         aborts_if !exists<Balance<CoinType>>(to);
55 │ │     }
   │ ╰─────^
   │
   =     at ./sources/BasicCoin.move:44: transfer
   =     at ./sources/BasicCoin.move:50: transfer (spec)
   =     at ./sources/BasicCoin.move:51: transfer (spec)
   =     at ./sources/BasicCoin.move:44: transfer
   =         from = signer{0x4823}
   =         to = 0x18be
   =         amount = 0
   =         _witness = <generic>
   =     at ./sources/BasicCoin.move:45: transfer
   =     at ./../../../../move-stdlib/sources/signer.move:12: address_of
   =         s = signer{0x4823}
   =     at ./../../../../move-stdlib/sources/signer.move:13: address_of
   =         result = 0x4823
   =     at ./../../../../move-stdlib/sources/signer.move:14: address_of
   =     at ./sources/BasicCoin.move:66: withdraw (spec)
   =     at ./sources/BasicCoin.move:45: transfer
   =     at ./sources/BasicCoin.move:66: withdraw (spec)
   =     at ./sources/BasicCoin.move:57: withdraw
   =         addr = 0x4823
   =         amount = 0
   =     at ./sources/BasicCoin.move:58: withdraw
   =     at ./sources/BasicCoin.move:33: balance_of
   =         owner = 0x4823
   =     at ./sources/BasicCoin.move:34: balance_of
   =         ABORTED

Error: exiting with verification errors
```

I ended up collecting a bunch of useful doc links
in the ["Move documentation"](#user-content-move-documentation)
section of this post,
and am looking forward to reading them.

I wanted to investigate the Move compiler architecture more,
to understand problems Solana might run into translating Move to LLVM,
but I ran out of time,
and am punting that to the future.

I did quickly read [this recent blog post](https://medium.com/@kklas/smart-contract-development-move-vs-rust-4d8f84754a8f)
about differences between Solana and Move;
and the [Move paper](https://arxiv.org/pdf/2004.05106.pdf),
but only have the barest feel for the challenges ahead,
discussed briefly at the end of this post.




## Learnings

Some of the things I learned from this experience.

- Run `dev_setup.sh -ypt` to get all the build and test prerequisits.
  This installs build tools (`-t`), theorem provers for testing (`-y`),
  and sets up the environment in `~/.profile` (`-p`).
  Then run `source ~/.profile` to configure the environment.
- `dev_setup.sh` stores dotnet and boogie in `~/.dotnet`;
  z3 and cvc5 in `/usr/local/bin`; other tools are installed via the system package manager.
- Modules decide at what address they are published, unlike other smart contract languages.
  This address can be configured in `Move.toml`, but it's not clear to me yet if reconfiguring this
  value is easy or ideomatic.
- Decompile Move bytecode with `move disassemble`.
- Move has separate namespaces for types and modules: e.g. there is a primitive `signer` type
  and a `std::signer` module.
- The final argument to `assert!` is an ["abort code"](https://move-language.github.io/move/abort-and-assert.html).
- Zero is a valid abort code, but does not indicate success (unlike process exit codes).
- An aborted transaction rolls back all state changes, across program calls.
  This is easy to reason about.
- The Move equivalent of crates.io is [movey.net](https://www.movey.net/).
- Move has built-in test coverage: run `move test --coverage` then `move coverage summary`, etc.
- Every address has a type->value map for "resource" storage.
  This is an intuitive dynamically-extensible data structure while still being typed.
- The address of the `std` module is `0x1`.
- Emacs's rust-mode can colorize a Move buffer reasonably well.
  Indentation doesn't work so great.
- The standard library is [partially written in Rust, partially Move](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-stdlib).
- `has foo` indicates a struct has the `foo` _ability_ (not "trait").
  Or is it _capability_? I seem to have seen both words used for this purpose.
- `acquires Balance` means a function accesses the `Balance` type from global storage.
  I don't know why this is necessary, but I like that Move externalizes sided effects.
- Integer overflow aborts.
- Move has a built-in theorem prover that allows the behavior of functions to be verified.
- In specs, `abort_if_is_strict` is the default behavior, but `pragra abort_if_is_partial`,
  allows for only partial abort conditions to be specified.




## Questions

Some of the questions I had but didn't answer.

- Modules have a hard-coded address,
  and they are only published at that address.
  Does that mean that I can't make multiple deployments of my dapp without changing the source?
- Why do tests declare `signer` parameters while also using attributes
  to declare those arguments concrete values?
  Why not create the signer in the test?
  Not possible?
  The signature needs to be simulated by the test runner?
- Does Move have macros? The book says `assert!` is a built-in "macro-like" operation.
- Does Move have error handling ideoms beyond hard-abort?
- Why does Move require `acquires Balance` annotations?
- Where is `MAX_U64` defined? It appears to be is magically imported.
- Why can specs write overflowing expressions that "just work",
  when regular Move has trapped overflows?
  I imagine the prover operates on natural numbers, not machine numbers.




## Move documentation

I want to figure out what documentation is available, then I'll pick a tutorial to follow.
Starting from within the repo I see that within the [`language`] directory,
there is a [`documentation`] directory.
I'll expect this to be the most up to date Move documentation since it's closest to the source code.

[`language`]: https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language
[`documentation`]: https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation

What's here?

- [A tutorial](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/tutorial).
  This looks like the place to start for new devs.
- [A book](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/book).
  Built with [`mdbook`](https://rust-lang.github.io/mdBook/).
  Published at <https://move-language.github.io/move/>. Confusingly there is
  an entirely different unofficial "Move Book" at <https://move-book.com>.
- [Specs!](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/spec).
  Right now it's just for the Move VM. It doesn't look super complete, but definitely a good start.
- [Examples](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/examples).
  The naming of the two subdirs here doesn't give me confidence: `diem-framework` and `experimental`.
  I wonder how up to date these are.

Also:

- [Changes](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/changes).
  Not under `documentation` but seems to be accepted "RFCs" to change the language.
- [std](https://github.com/move-language/move/tree/main/language/move-stdlib/docs).
  Hand-written docs for the standard library.
  Not clear if these are published anywhere or are up to date.
- [Move Prover User Guide](https://github.com/move-language/move/blob/main/language/move-prover/doc/user/prover-guide.md).
  Docs for Move's embedded theorem prover!

Other resources:

- [Resources: A Safe Language Abstraction for Money](https://arxiv.org/pdf/2004.05106.pdf).
  The Move paper.
- [Awesome Move](https://github.com/MystenLabs/awesome-move).
  Seems to be the central collection of all Move stuff at the moment.
- [Move Book](https://github.com/damirka/move-book/).
  Seemingly completely different from the official Move book, and published at <https://move-book.com>.
- [Smart Contract Development - Move vs. Rust](https://medium.com/@kklas/smart-contract-development-move-vs-rust-4d8f84754a8f).
  A recent blog post.
- [Move Patterns](https://www.move-patterns.com/)
- [Programming with Objects](https://docs.sui.io/build/programming-with-objects). A Sui tutorial.
- [Move and Smart Contract Development](https://starcoinorg.github.io/starcoin-cookbook/docs/move/). A StarCoin tutorial.




## The journal

Everything else here is just a journal of my experience.
Probably not of general interest.
Perhaps of interest to Move devs and people searching for problems I hit.




## Building and testing the compiler

The Move compiler is located at

<https://github.com/move-language/move>

The first thing I want to do is learn how to build, test, and run it from source.
Since this is an immature language,
and one that I would like to contribute to,
I expect to deal directly with the source code.
So I want to learn to navigate it from the start,
and not just install the binary as would an end user.

Since this is a Rust project I expect to be able to test it by just running `cargo test`,
From skimming the README I know this project uses Docker,
which I prefer not to bother with,
and that it has developer instructions,
but I'm just going to start the way I expect to start:

```
$ cargo build && cargo test
   Compiling proc-macro2 v1.0.43
   Compiling unicode-ident v1.0.3
   Compiling syn v1.0.99
   Compiling cfg-if v1.0.0
   Compiling libc v0.2.126
...
11:13:54 [INFO] 73 verification conditions
FAILURE proving 10 modules from package `` in 1.304s
test prove ... FAILED

failures:

---- prove stdout ----
thread 'prove' panicked at 'called `Result::unwrap()` on an `Err` value: No boogie executable set.  Please set BOOGIE_EXE', language/tools/move-cli/src/base/prove.rs:132:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    prove

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.31s

error: test failed, to rerun pass '-p move-stdlib --test move_verification_test'
```

So it builds, but doesn't test successfully out of the box.
I expected that, as from perusing the tutorial while I build, I discover there is a [`dev_setup.sh`] script.

[`dev_setup.sh`]: https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/scripts/dev_setup.sh




## Running `dev_setup.sh`

_Note: I am going to spend the next ~1700 words debugging `dev_setup.sh`,
but my entire problem came from not running it &mdash; as the tutorial said to &mdash;
with the `-ypt` arguments._

From reading `dev_setup.sh` I can see why they offer a Docker-based build solution,
as this installs a bunch of dev tools I don't really want:
[Boogie](https://github.com/boogie-org/boogie),
[z3](https://github.com/Z3Prover/z3),
[cvc5](https://github.com/cvc5/cvc5),
and Boogie requires .NET.

Boogie is a "program verifier".
I don't know exactly what that means but it seems to verify the implementation of programming languages,
and it depends on a SAT solver,
either z3 or cvc5.

This is all encouraging for the implementation of the language,
but kinda bleh for me as someone wanting to contribute.

I bite the bullet and run this `dev_setup.sh` script:

```
$ bash scripts/dev_setup.sh
Welcome to Move!

This script will download and install the necessary dependencies needed to
build and run Move.

Based on your selection, these tools will be included:
Build tools (since -t or no option was provided):
  * Rust (and the necessary components, e.g. rust-fmt, clippy)
  * CMake
  * Clang
  * pkg-config
  * libssl-dev
  * if linux, gcc-powerpc-linux-gnu
  * NodeJS / NPM
If you'd prefer to install these dependencies yourself, please exit this script
now with Ctrl-C.
Proceed with installing necessary dependencies? (y/N) >
```

It has the courtesy to tell me what it's going to do and let me cancel.
So it also requires NPM ... and _a PowerPC cross-compiler_. wtf.
Ok, let's do this:

```
<tons of spew>
...
Finished installing all dependencies.

You should now be able to build the project by running:
        cargo build
```

Well, there was a bunch of spew.
It was interesting, but not interesting enough to reproduce here.
I read through it all and it all appeared to work.
It says I should be able to build now, though I could already build before.

Can I test now?

```
$ cargo test
...
failures:

---- prove stdout ----
thread 'prove' panicked at 'called `Result::unwrap()` on an `Err` value: No boogie executable set.  Please set BOOGIE_EXE', language/tools/move-cli/src/base/prove.rs:132:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
...
```

No. No `BOOGE_EXE`. Fortunately I already peeked at the tutorial and know that all the env vars I need are in `~/.profile` and I need to "source" it:

```
source ~/.profile
```

And after that I still can't complete the tests;
`$BOOGIE_EXE` is not set.
The `dev_setup.sh` script didn't modify my `~/.profile` at all!

Which of the required tools do I have now?

- [x] `dotnet`
- [ ] `boogie` - nope. not in `~/.dotnet/tools`
- [ ] `z3` - nope. not in `/usr/local/bin`
- [ ] `cvc5` - nope. not in `/usr/local/bin`

I also see that the script should have installed the Solidity compiler &mdash; for some reason &mdash; and it is also not where it should be in `/usr/local/bin`.

Well, it seems like `dev_setup.sh` just competely blew it, and then said it succeeded.

These types of scripts that modify your devs' local system need to be airtight,
or devs can get pretty upset &mdash;
I once accidentally destroyed a tester's entire Windows installation with some poorly-programed paths;
fortunately they were cool with it.




## Debugging `dev_setup.sh`

Ok, after reading through the script, I think I get why I didn't get any of the theorem provers:
The script only does that if asked:

```sh
if [[ "$INSTALL_PROVER" == "true" ]]; then
  export DOTNET_INSTALL_DIR="${HOME}/.dotnet/"
  if [[ "$OPT_DIR" == "true" ]]; then
    export DOTNET_INSTALL_DIR="/opt/dotnet/"
    mkdir -p "$DOTNET_INSTALL_DIR" || true
  fi
  install_pkg unzip "$PACKAGE_MANAGER"
  install_z3
  install_cvc5
  install_dotnet
  install_boogie
fi

if [[ "$INSTALL_SOLIDITY" == "true" ]]; then
  install_solidity
fi
```

So I need to specify `INSTALL_PROVER=true bash scripts/dev_setup.sh`.
And I didn't get `solc` because I didn't ask for it.
I'll not ask for it until I need it.
This doesn't explain why my `~/.profile` wasn't modified,
but I'll figure that out later.

First, I'll try again with `INSTALL_PROVER=true`:

```
$ INSTALL_PROVER=true bash scripts/dev_setup.sh
Welcome to Move!

This script will download and install the necessary dependencies needed to
build and run Move.
...
<etc>
```

Reading the output, I don't see any mention of the provers...
and they don't exist in the expected paths.
So this didn't work.
I must have done something wrong.

Ok, after reading the script _again_,
I can't just set these env vars myself:

```
$ bash scripts/dev_setup.sh -h
scripts/dev_setup.sh: option requires an argument -- h
Usage:
Installs or updates necessary dev tools for Move.
-b batch mode, no user interactions and minimal output
-p update /home/brian/.profile
-t install build tools
-y installs or updates Move prover tools: z3, cvc5, dotnet, boogie
-d installs the solidity compiler
-g installs Git (required by the Move CLI)
-v verbose mode
-i installs an individual tool by name
-n will target the /opt/ dir rather than the /home/brian dir.  /opt/bin/, /opt/rustup/, and /opt/dotnet/ rather than /home/brian/bin/, /home/brian/.rustup/, and /home/brian/.dotnet/
If no toolchain component is selected with -t, -o, -y, or -p, the behavior is as if -t had been provided.
This command must be called from the root folder of the Move project.
```

This is the "usage" info for `dev_scripts.h`.
Notice that it didn't actually interpret `-h` as expected ("scripts/dev_setup.sh: option requires an argument -- h"),
but it did at least print the help message.

So I need to run `dev_setup.h` with `-y`:

```
$ bash scripts/dev_setup.sh -y
Welcome to Move!

This script will download and install the necessary dependencies needed to
build and run Move.

Based on your selection, these tools will be included:
Move prover tools (since -y was provided):
  * z3
  * cvc5
  * dotnet
  * boogie
If you'd prefer to install these dependencies yourself, please exit this script
now with Ctrl-C.
Proceed with installing necessary dependencies? (y/N) >
...
Installing Z3
Installing cvc5
Installing .Net
scripts/dev_setup.sh: line 235: /home/brian/.dotnet//dotnet: No such file or directory
gettext is already installed
Installing zlib1g.
Reading package lists... Done
Building dependency tree
Reading state information... Done
zlib1g is already the newest version (1:1.2.11.dfsg-2ubuntu1.3).
zlib1g set to manually installed.
The following package was automatically installed and is no longer required:
  libfwupdplugin1
Use 'sudo apt autoremove' to remove it.
0 upgraded, 0 newly installed, 0 to remove and 137 not upgraded.
apt-get install result code: 0
dotnet-install: Note that the intended use of this script is for Continuous Integration (CI) scenarios, where:
dotnet-install: - The SDK needs to be installed without user interaction and without admin rights.
dotnet-install: - The SDK installation doesn't need to persist across multiple CI runs.
dotnet-install: To set up a development environment or to run apps, use installers rather than this script. Visit https://dotnet.microsoft.com/download to get the installer.

dotnet-install: Attempting to download using aka.ms link https://dotnetcli.azureedge.net/dotnet/Sdk/6.0.400/dotnet-sdk-6.0.400-linux-x64.tar.gz
dotnet-install: Extracting zip from https://dotnetcli.azureedge.net/dotnet/Sdk/6.0.400/dotnet-sdk-6.0.400-linux-x64.tar.gz
dotnet-install: Installed version is 6.0.400
dotnet-install: Adding to current process PATH: `/home/brian/.dotnet`. Note: This change will be visible only when sourcing script.
dotnet-install: Note that the script does not resolve dependencies during installation.
dotnet-install: To check the list of dependencies, go to https://docs.microsoft.com/dotnet/core/install, select your operating system and check the "Dependencies" section.
dotnet-install: Installation finished successfully.
Installing boogie
You can invoke the tool using the following command: boogie
Tool 'boogie' (version '2.15.7') was successfully installed.
Finished installing all dependencies.
```

This seems to have worked.

I still don't have any env setup code in my `~/.profile`.
Why is that?

After looking at the tutorial again I see it suggests _this_ command:

```
$ ./scripts/dev_setup.sh -ypt
```

- `-t` installs the basic build tools
- `-y` installs the provers
- `-p` modifies `~/.profile`

So all this confusion is my fault for not following the directions correctly.

Running it with those options looks like:

```
$ bash scripts/dev_setup.sh -ypt
Welcome to Move!

This script will download and install the necessary dependencies needed to
build and run Move.

Based on your selection, these tools will be included:
Build tools (since -t or no option was provided):
  * Rust (and the necessary components, e.g. rust-fmt, clippy)
  * CMake
  * Clang
  * pkg-config
  * libssl-dev
  * if linux, gcc-powerpc-linux-gnu
  * NodeJS / NPM
Move prover tools (since -y was provided):
  * z3
  * cvc5
  * dotnet
  * boogie
Moreover, ~/.profile will be updated (since -p was provided).
If you'd prefer to install these dependencies yourself, please exit this script
now with Ctrl-C.
Proceed with installing necessary dependencies? (y/N) >
```

Now my `~/.profile` is appended with:

```
export DOTNET_ROOT="/home/brian/.dotnet"
export PATH="/home/brian/.dotnet/tools:$PATH"
export Z3_EXE="/home/brian/bin/z3"
export CVC5_EXE="/home/brian/bin/cvc5"
export BOOGIE_EXE="/home/brian/.dotnet/tools/boogie"
```

What happens if I run it again?
Will it mangle my profile with duplicate env-vars?

I try and, no, it is smart enough not to re-add them.
The `dev_setup.sh` function for adding an env var is this:

```
function add_to_profile {
  eval "$1"
  FOUND=$(grep -c "$1" <"${HOME}/.profile" || true) # grep error return would kill the script.
  if [ "$FOUND" == "0" ]; then
    echo "$1" >>"${HOME}"/.profile
  fi
}
```

At least I have learned a lot about how the dev tools for Move are managed.

Now when I run the test suite ... it still fails!

```
test prover unit[default]::functional/nonlinear_arithm.move ... FAILED
Error:
New output differs from baseline!
Call this test with env variable UPBL=1 to regenerate or remove old baseline files.
Then use your favorite changelist diff tool to verify you are good with the changes.

Or check the rudimentary diff below:

= Move prover returns: exiting with verification errors
= ... (186 lines)
=     =         ABORTED
+
+ error: verification out of resources/timeout (global timeout set to 40s)
+     ┌─ tests/sources/functional/nonlinear_arithm.move:111:5
+     │
+ 111 │ ╭     fun overflow_u128_mul_4(a: u128, b: u128, c: u128, d: u128): u128 {
+ 112 │ │         a * b * c * d
+ 113 │ │     }
+     │ ╰─────^
test prover unit[cvc5]::functional/choice.move ... ok
test prover unit[cvc5]::functional/type_reflection.move ... ok
test prover unit[cvc5]::functional/mut_ref.move ... ok
test prover unit[cvc5]::functional/verify_vector.move ... ok

failures:
    prover unit[default]::functional/fixed_point_arithm.move
    prover unit[cvc5]::functional/loops_with_memory_ops.move
    prover unit[default]::functional/nonlinear_arithm.move

test result: FAILED. 193 passed; 3 failed; 0 filtered out

error: test failed, to rerun pass '-p move-prover --test testsuite'
```

The prover is timing out on my slow computer!




## Extending the prover timeout

I would like to complete the test suite,
so I set out to figure out how to extend the prover timeout.

I discover that the tests that run the prover
use a complete custom test harness,
defined in [testsuite.rs](https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-prover/tests/testsuite.rs).
After reading the functions
[`test_runner_for_feature`](https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-prover/tests/testsuite.rs#L104)
and
[`get_flags_and_baseline`](https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-prover/tests/testsuite.rs#L172),
then
[`BoogieOptions::vc_timeout`](https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-prover/boogie-backend/src/options.rs#L99)
and
[`Options::create_from_args`](https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-prover/src/cli.rs#L134)
I find that the move prover CLI
(which is seemingly not the same as the `move prove` command)
has a `--timeout` argument,
and the prover testsuite accepts
an evironment variable, `MVP_TEST_FLAGS`,
that passes arbitrary arguments to the prover.

This command increases the prover timeout for the prover testsuite:

```
$ MVP_TEST_FLAGS=--timeout=500 cargo test -p move-prover --test testsuite
```

The prover test suite [accepts some other env vars](https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-prover/tests/testsuite.rs#L23)
that I haven't looked into.

Even after extending the timeout,
one of the tests,
`loops_with_memory_ops` _still_ fails for reasons I don't understand.

I'm done with this for now.




## Running the tutorial: steps 1 and 2

Now I am going to run [the official tutorial](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/tutorial).

The first thing this teaches me is that modules have a hard-coded address,
and they are only published at that address.
Does that mean that I can't make multiple deployments of my dapp without changing the source?

These docs are hyperlinked well and I already have a bunch of background tabs open to read later.

The very first 10-line example has already introduced a bunch of concepts:

- [modules](https://move-language.github.io/move/modules-and-scripts.html)
- [addresses](https://move-language.github.io/move/address.html)
- [structs](https://move-language.github.io/move/structs-and-resources.html)
- [abilities](https://move-language.github.io/move/abilities.html) via the `has` keyword
- the [`signer`](https://move-language.github.io/move/signer.html) type,
  which I think is a primitive
- the [`move_to`](https://move-language.github.io/move/abilities.html?highlight=move_to#key) operator

from just this source code:

```move
module 0xCAFE::BasicCoin {
    struct Coin has key {
        value: u64,
    }

    public fun mint(account: signer, value: u64) {
        move_to(&account, Coin { value })
    }
}
```

Lots to read about; I'll bother to read the book later.
Following the tutorial I build `BasicCoin`,
which produces a new directory, `build`,
and in that directory:

```
$ tree build
build
└── BasicCoin
    ├── BuildInfo.yaml
    ├── bytecode_modules
    │   └── BasicCoin.mv
    ├── source_maps
    │   └── BasicCoin.mvsm
    └── sources
        └── BasicCoin.move

4 directories, 4 files
```

That's refreshingly little to understand compared to what's in a typical Rust `target` directory!
We've got the original source, compiled bytecode, a source-map,
and something called `BuildInfo.yaml`.

`BasicCoin.mv` is bytecode so I can't look at it directly,
but I know I've seen reference to a decompiler in the Move tools directory.
Can I decompile the bytecode?

I can't decompile it by passing the path to `BasicCoin.mv` but
I can by passing the name `BasicCoin`:

```
$ move disassemble --name BasicCoin
// Move bytecode v5
module cafe.BasicCoin {
struct Coin has key {
        value: u64
}

public mint(account: signer, value: u64) {
B0:
        0: ImmBorrowLoc[0](account: signer)
        1: MoveLoc[1](value: u64)
        2: Pack[0](Coin)
        3: MoveTo[0](Coin)
        4: Ret
}
}
```

This is apparently the serialized form of Move bytecode.
Not sure what I can do with it yet.

What about the source map?
It's a binary.
I don't see anything in the move CLI that will let me inspect it.

What's in `BuildInfo.yaml`?

```yaml
---
compiled_package_info:
  package_name: BasicCoin
  address_alias_instantiation: {}
  source_digest: 943AB2329682C41C2EDFBCDE4062BC695786FAC1E296A21890191003DECECB3F
  build_flags:
    dev_mode: false
    test_mode: false
    generate_docs: false
    generate_abis: false
    install_dir: ~
    force_recompilation: false
    additional_named_addresses: {}
    architecture: ~
    fetch_deps_only: false
dependencies: []
```

Stuff.

Step 1 of this tutorial did not require writing actual code.

Onto step 2.

Move uses Rust's attribute syntax:

```move
    #[test(account = @0xC0FFEE)]
    fun test_mint_10(account: signer) acquires Coin {
        let addr = signer::address_of(&account);
        mint(account, 10);
        // Make sure there is a `Coin` resource under `addr` with a value of `10`.
        // We can access this resource and its value since we are in the
        // same module that defined the `Coin` resource.
        assert!(borrow_global<Coin>(addr).value == 10, 0);
    }
```

I implemented (and probably designed) that syntax in Rust :)

The output of `move test` looks attractive:

```
$ move test
INCLUDING DEPENDENCY MoveStdlib
BUILDING BasicCoin
Running Move unit tests
[ PASS    ] 0xcafe::BasicCoin::test_mint_10
Test result: OK. Total tests: 1; passed: 1; failed: 0
```

On a console it has colors.

I wonder why this test declares a parameter,
only to use an attribute to declare the value of that parameter:

```move
    #[test(account = @0xC0FFEE)]
    fun test_mint_10(account: signer) acquires Coin {
```

Maybe it's impossible to create signers and the runtime has to do it.

Like Rust,
Move has separate namespaces for types and modules.
This example imports `std::signer`,
which has the same name as primitive type `signer`,
and accesses the function `signer::address_of`.

From this line:

```move
        assert!(borrow_global<Coin>(addr).value == 10, 0);
```

I gather that every address acts as a key-value store,
where types are the keys,
and thus every address can store a single value of any type.
The `borrow_global` operator accesses these key/vaules.

Presumably `<...>` is the syntax for generics.

Move seems to have some kind of macros,
using Rust's `!` syntax.
(The book says `assert!` is a built-in, "macro-like").

What is this trailing "`, 0`" in the assertion?!
The tutorial doesn't say.
From [the book's page on assert and abort](https://move-language.github.io/move/abort-and-assert.html),
the trailing `0` is an "abort code".

This page also indicates that aborting a transaction
reverts all state changes.
That is great.
Some smart contract platforms,
e.g. (I think) the EVM, and ink!,
allow cross-contract calls to fail
and for execution to resume,
making it complex to reason about partial commits.
So much easier to be confident that either the entire transaction succeeds
or it fails.

I follow the exercise for this tutorial to make the test fail,
and it produces this output:

```
$ move test
INCLUDING DEPENDENCY MoveStdlib
BUILDING BasicCoin
Running Move unit tests
[ FAIL    ] 0xcafe::BasicCoin::test_mint_10

Test failures:

Failures in 0xcafe::BasicCoin:

┌── test_mint_10 ──────
│ error[E11001]: test failure
│    ┌─ ./sources/FirstModule.move:24:9
│    │
│ 18 │     fun test_mint_10(account: signer) acquires Coin {
│    │         ------------ In this function in 0xcafe::BasicCoin
│    ·
│ 24 │         assert!(borrow_global<Coin>(addr).value == 11, 0);
│    │         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Test was not expected to abort but it aborted with 0 here
│
│
└──────────────────

Test result: FAILED. Total tests: 1; passed: 0; failed: 1
```

This is gorgeous. Similar to Rust.
I wonder if they are using the same error reporting crates.
On the console this is in color.

I am liking the "exercises" in the tutorial:
good prompts to dig in and learn on your own.

The next exercise is:
"Find a flag that allows you to gather test coverage information, and then play around with using the move coverage command to look at coverage statistics and source coverage."

I first try:

```
$ move coverage source --module BasicCoin
Error: No such file or directory (os error 2): Coverage map file '"./.coverage_map.mvcov"' doesn't exist
```

So this is for reporting coverage,
to gather coverage I need to

```
$ move test --coverage
INCLUDING DEPENDENCY MoveStdlib
BUILDING BasicCoin
Running Move unit tests
[ PASS    ] 0xcafe::BasicCoin::test_mint_10
Test result: OK. Total tests: 1; passed: 1; failed: 0
```

This generates two files:

- `.coverage_map.mvcov`
- `.trace`

Then I can run

```
$ move coverage source --module BasicCoin
```

The output is just the entire source of `BasicCoin`, but green.
Green must mean "covered".

`move coverage bytecode --module BasicCoin` is similar.

I can also run `move coverage summary`:

```
$ move coverage summary
+-------------------------+
| Move Coverage Summary   |
+-------------------------+
Module 0000000000000000000000000000cafe::BasicCoin
>>> % Module coverage: 100.00
+-------------------------+
| % Move Coverage: 100.00  |
+-------------------------+
```

100% coverage.




## Running the tutorial: steps 3 and 4

Starting now at [tutorial step 3](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/tutorial#step-3-designing-my-basiccoin-module),
"Designing my `BasicCoin` module".

The function signatures for this step reveal that `address` is probably a primitive type:

```move
/// Mint `amount` tokens to `mint_addr`. Mint must be approved by the module owner.
public fun mint(module_owner: &signer, mint_addr: address, amount: u64) acquires Balance { ... }
```

The [address docs](https://move-language.github.io/move/address.html)
indicate that addresses can also be considered accounts,
and that they are storage locations.
Sounds like Solana and probably other platforms.
The `signer` type then is an address plus some kind of write capability.

This function's signature also introduces the `acquires` keyword,
which appears to be a constraint of some kind on the function: `aquires Balance`.
I can't guess offhand what this means,
but it suggests movement and ownership and linear types,
which I love!

The tutorial explains that "global storage" looks conceptually like:

```rust
struct GlobalStorage {
    resources: Map<address, Map<ResourceType, ResourceValue>>
    modules: Map<address, Map<ModuleName, ModuleBytecode>>
}
```

There are typed resources, and named code modules,
and addresses hold maps of both.
So addresses can hold multiple modules:
this seems different than most blockchains,
where an address would hold a single smart contract,
but maybe in Move a "single smart contract" is composed of multiple modules.

The tutorial contains this helpful illustration:

![Move blockchain state illustration](https://raw.githubusercontent.com/move-language/move/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/tutorial/diagrams/move_state.png "move blockchain state illustration")

Which it contrasts to how Solidity maps data to addresses:

![Solidity blockchain state illustration](https://raw.githubusercontent.com/move-language/move/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/tutorial/diagrams/solidity_state.png "solidity blockchain state illustration")

(From https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/tutorial)

Great comparison.
Solana is different from both &mdash; in Solana one would probably
derive a unique address from the combination of a program's address and a user's address.
Move similar to Solidity than Move, but still different;
and of course in Solana all the complexity of deriving addresses is forced on the caller
so that they can specify every operable address as an argument to the call.

---

This example replaces the hard-coded `0xCAFE` namespace with
what looks like a variable, `NamedAddr`:

```move
module NamedAddr::BasicCoin {
    struct Coin has store {
        value: u64
    }
}
```

So it would seem that each module is not hard-coded to a specific address.

Also in this example, the type that is stored as a "resource" in an address
is annotated with `has key`:

```move
    struct Balance has key {
        coin: Coin
    }
```

and `Coin` is now `store`:

```move
    struct Balance has key {
        coin: Coin
    }
```

So probably something that `has key` can be used as the type-key of an address resource,
and something that `has store` can be stored as part of a key.

That's where tutorial step 3 ends.
No actual coding or action needed in this step. Pft.
Onto [step 4](https://github.com/move-language/move/tree/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/documentation/tutorial#step-4-implementing-my-basiccoin-module)

---

The previous step didn't contain a buildable `Move.toml`.
In this step it's a buildable project again,
and I see the `Move.toml` now contains a hard-coded definition of `NamedAddr`:

```toml
[package]
name = "BasicCoin"
version = "0.0.0"

[addresses]
NamedAddr = "0xCAFE"

[dependencies]
MoveStdlib = { local = "../../../../move-stdlib/", addr_subst = { "std" = "0x1" } }
```

So this probably allows the source to avoid duplication of `0xCAFE`,
but still suggests that all modules have a single instantation at a single address.
So curious.

Also, note that `std` lives at address `0x1`.

_Aside: at this point the `BasicCoin.move` example code is getting pretty long,
and the lack of color-coding in my emacs buffer is challenging to read.
I am turning on rust-mode to see if it reasonably approximates a hypothetical move-mode._

In this example we see how fatal errors are handled as aborts:

```move
    public fun mint(module_owner: &signer, mint_addr: address, amount: u64) {
        // Only the owner of the module can initialize this module
        assert!(signer::address_of(module_owner) == MODULE_OWNER, ENOT_MODULE_OWNER);

        // Deposit `amount` of tokens to `mint_addr`'s balance
        deposit(mint_addr, Coin { value: amount });
    }
```

Every module defines its own abort codes.
This means that understanding why something aborted is going to require module-specific context.
I can see it getting pretty confusing to debug when there are many inter-module calls in a transaction
and the failure info you get is an integer abort code.
Hopefully failures also come with some kind of stack trace.

The tutorial mentions an `errors` module with common "error categories" for this purpose,
but doesn't link to it.

After some searching I find the [`error` docs](https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-stdlib/docs/error.md).
They are handwritten Markdown files, seemingly not generated.
It does have some [predifined abort codes for common cases](https://github.com/move-language/move/blob/f3cba16754a0b986c24ce06b4bf698f86ccc65b9/language/move-stdlib/docs/error.md#constants).

This tutorial step ends with an exercise to fill in this "TODO":

```move
    /// Publish an empty balance resource under `account`'s address. This function must be called before
    /// minting or transferring to the account.
    public fun publish_balance(account: &signer) {
        // TODO: add an assert to check that `account` doesn't already have a `Balance` resource.
        let empty_coin = Coin { value: 0 };
        move_to(account, Balance { coin: empty_coin });
    }
```

I don't think I have the tools to do this yet &mdash;
I know how to `borrow_global<Balance>` but don't know how to handle if the `Balance` key doesn't exist.
Guess I need to go read the book.

I read ["Storing Resources in Global Storage"](https://move-language.github.io/move/structs-and-resources.html#storing-resources-in-global-storage)
from the book and learn that `has key` indicates the key _ability_;
and follow links to the ["Global Storage Operators"](https://move-language.github.io/move/global-storage-operators.html#global-storage---operators) chapter,
to find the `exists` operator.

So the finished function is

```move
    /// Publish an empty balance resource under `account`'s address. This function must be called before
    /// minting or transferring to the account.
    public fun publish_balance(account: &signer) {
        assert!(!exists<Balance>(signer::address_of(account)), EALREADY_HAS_BALANCE);
        let empty_coin = Coin { value: 0 };
        move_to(account, Balance { coin: empty_coin });
    }
```

This chapter didn't explain the `acquires Balance` function annotation,
but I guess it means that the function accesses `Balance` types from global storage.
Not obvious why Move needs this annotation.

The exercise for this tutorial cleverly needed me to add and propagate
`acquires Balance` effect annotations.





## Running the tutorial: steps 5 and 6

Now it is time to write unit tests.


This step comes with example unit tests and the exercise challenges us
to write a unit test for the case where the `balance_of` function is called
for an account that doesn't have a balance.

This is my test:

```move
    #[test]
    #[expected_failure]
    fun balance_of_dne() acquires Balance {
        balance_of(@0x1);
    }
```

It works the first try.
Everything I needed to crib was in the example unit tests in the tutorial module.
Nice.
This is also the exact source given in the solution.
Great.

That's it for tutorial step 5.
In step 6 we use generics to make `BasicCoin` reusable for any coin type.
Knowing that Move does not have dynamic dispatch,
I am guessing that generics are going to be uniquely important to Move,
and that's why they are being introduced so prominently.

This step adds a second module, `MyOddCoin`, that uses `BasicCoin`.
Both modules are declared at the same address, `NamedAddr`:

```move
module NamedAddr::BasicCoin {
```

```move
module NamedAddr::MyOddCoin {
```

The `Coin` and `Balance` types get phantom typarams:

```move
    struct Coin<phantom CoinType> has store {
        value: u64
    }

    struct Balance<phantom CoinType> has key {
        coin: Coin<CoinType>
    }
```

This allows any instantiations with differing typarams to have different types,
so e.g. multiple `Balance` instances can be stored in a single account.

In this example `BasicCoin` is used as a library &mdash;
it is not the interface others interact with to mint `MyOddCoin`.
Users call `MyOddCoin`,
which has its own ad-hoc interface:

```move
    public fun setup_and_mint(account: &signer, amount: u64) {
        BasicCoin::publish_balance<MyOddCoin>(account);
        BasicCoin::mint<MyOddCoin>(signer::address_of(account), amount, MyOddCoin {});
    }

    public fun transfer(from: &signer, to: address, amount: u64) {
        // amount must be odd.
        assert!(amount % 2 == 1, ENOT_ODD);
        BasicCoin::transfer<MyOddCoin>(from, to, amount, MyOddCoin {});
    }
```

This suggests that, at least with what I know so far,
one can't generically call any Move program that implements `BasicCoin` &mdash;
it has to have concrete knowledge of `MyOddCoin`.
That would make sense based on my understanding that there is no dynamic dispatch in Move,
but there must be some mechanism to require that a e.g. uniswap
does not need to code explicitly for every coin in supports.

This example uses a "witness" pattern,
treating an empty `MyOddCoin` token as a _capability_ by
which only the `MyOddCoin` module can call `BasicCoin::transfer<MyOddCoin>`:

```move
    public fun transfer(from: &signer, to: address, amount: u64) {
        // amount must be odd.
        assert!(amount % 2 == 1, ENOT_ODD);
        BasicCoin::transfer<MyOddCoin>(from, to, amount, MyOddCoin {});
    }
```

That last parameter to `BasicCoin::transfer` is using a `MyOddCoin` instance
as a capability. No other module can instantiate `MyOddCoin`,
and so no other module can transfer it by calling `BasicCoin` &mdash;
they have to call `MyOddCoin::transfer`.

It would seem important then that the `MyOddCoin` module never lets any unpriviledged
module access to an instance of the type or it could bypass the extra
condition on transfers that `amount` must be odd.

Step 6 doesn't have any exercises.




## Writing a proof!

In step 7 we get to use the Move prover ourselves!

That is exciting.
I assumed the prover was only for Move's own test suite.

Statements to be proved are writting in `spec` blocks:

```move
    spec balance_of {
        pragma aborts_if_is_strict;
    }
```

This says, I think, that we need to explicitly
identify all cases that `balance_of` might abort.

Specs are proved with a new command, `move prove`,
which presently fails on the example:

```
$ move prove
[INFO] preparing module 0xcafe::BasicCoin
[INFO] transforming bytecode
[INFO] generating verification conditions
[INFO] 6 verification conditions
[INFO] running solver
[INFO] 0.417s build, 0.014s trafo, 0.017s gen, 1.765s verify, total 2.213s
error: abort not covered by any of the `aborts_if` clauses
   ┌─ ./sources/BasicCoin.move:37:5
   │
34 │           borrow_global<Balance<CoinType>>(owner).coin.value
   │           ------------- abort happened here with execution failure
   ·
37 │ ╭     spec balance_of {
38 │ │         pragma aborts_if_is_strict;
39 │ │     }
   │ ╰─────^
   │
   =     at ./sources/BasicCoin.move:33: balance_of
   =         owner = 0x29
   =     at ./sources/BasicCoin.move:34: balance_of
   =         ABORTED

Error: exiting with verification errors
```

We make it work by specifying the conditions that cause aborts:

```move
    spec balance_of {
        pragma aborts_if_is_strict;
        aborts_if !exists<Balance<CoinType>>(owner);
    }
```

Kinda magical.
I wonder how expressive these `aborts_if` expressions can be.

Step 8 has some hands-on proving.

The first example in the step focuses on proving the abort conditions of the `withdraw` method.
I notice that `assert` and `aborts_if` are the logical opposites of each other.
`withdraw` contains this assertion:

```
        assert!(balance >= amount, EINSUFFICIENT_BALANCE);
```

and the spec contains this `aborts_if`:

```
        aborts_if balance < amount;
```

This is an awkward relationship,
and I wonder if there is a reason for it
beyond it reading naturally to make opposite
assertions in these different contexts.

It reads pretty strange,
saying approximately the same thing in two different places,
and not even saying them the same way.

**Note that for quite a while here I make a huge mistake
by editing and testing the _step 7_ code,
when I should be on _step 8_.**
I would likely not have been so confused had I been editing the correct code.

The tutorial ends by asking us to write the `aborts_if` specs for `transfer`,
which is defined as

```move
    /// Transfers `amount` of tokens from `from` to `to`. This method requires a witness with `CoinType` so that the
    /// module that owns `CoinType` can  decide the transferring policy.
    public fun transfer<CoinType: drop>(from: &signer, to: address, amount: u64, _witness: CoinType) acquires Balance {
        let check = withdraw<CoinType>(signer::address_of(from), amount);
        deposit<CoinType>(to, check);
    }
```

I think this should abort if:

- `from` has less than `amount` balance
- `to` has not been initialized with a balance

I write this spec:

```move
    spec transfer {
        let from_balance = global<Balance<CoinType>>(from).coin.value;
        abort_if from_balance < amount;

        abort_if !exists<Balance<CoinType>>(to);
    }
```

I get an error:

```
$ move prove
error: unexpected token
   ┌─ ./sources/BasicCoin.move:51:18
   │
51 │         abort_if from_balance < amount;
   │                  ^^^^^^^^^^^^
   │                  │
   │                  Unexpected 'from_balance'
   │                  Expected ':'

Error: exiting with model building errors
```

This is because `abort_if` is not the correct name.
Should be `aborts_if`.
Unhelpful error message.

I fix the name and try again, get an error:

```
$ move prove
error: no matching declaration of `global`
   ┌─ ./sources/BasicCoin.move:50:28
   │
50 │         let from_balance = global<Balance<CoinType>>(from).coin.value;
   │                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   │
   = outruled candidate `global(address): #0` (expected `address` but found `signer` for argument 1)

error: type `?2` cannot be resolved as a struct
   ┌─ ./sources/BasicCoin.move:50:28
   │
50 │         let from_balance = global<Balance<CoinType>>(from).coin.value;
   │                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: type `?1` cannot be resolved as a struct
   ┌─ ./sources/BasicCoin.move:50:28
   │
50 │         let from_balance = global<Balance<CoinType>>(from).coin.value;
   │                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

This is because I passed the `global` builtin a `signer`,
not an `address`.
Good error message.

Can I call `signer::address_of` from a spec?
Can I assign it in a `let` binding?

My new spec:

```move
    spec transfer {
        let from_address = signer::address_of(from);
        let from_balance = global<Balance<CoinType>>(from_address).coin.value;
        aborts_if from_balance < amount;

        aborts_if !exists<Balance<CoinType>>(to);
    }
```

It compiles.
So specs can call regular Move functions.
That's a relief.

It doesn't prove though:

```
$ move prove
[INFO] preparing module 0xcafe::BasicCoin
[INFO] transforming bytecode
[INFO] generating verification conditions
[INFO] 6 verification conditions
[INFO] running solver
[INFO] 0.010s build, 0.002s trafo, 0.004s gen, 0.997s verify, total 1.014s
error: abort not covered by any of the `aborts_if` clauses
   ┌─ ./sources/BasicCoin.move:49:5
   │
34 │           borrow_global<Balance<CoinType>>(owner).coin.value
   │           ------------- abort happened here with execution failure
   ·
49 │ ╭     spec transfer {
50 │ │         let from_address = signer::address_of(from);
51 │ │         let from_balance = global<Balance<CoinType>>(from_address).coin.value;
52 │ │         aborts_if from_balance < amount;
53 │ │
54 │ │         aborts_if !exists<Balance<CoinType>>(to);
55 │ │     }
   │ ╰─────^
   │
   =     at ./sources/BasicCoin.move:44: transfer
   =     at ./sources/BasicCoin.move:50: transfer (spec)
   =     at ./sources/BasicCoin.move:51: transfer (spec)
   =     at ./sources/BasicCoin.move:44: transfer
   =         from = signer{0x4823}
   =         to = 0x18be
   =         amount = 0
   =         _witness = <generic>
   =     at ./sources/BasicCoin.move:45: transfer
   =     at ./../../../../move-stdlib/sources/signer.move:12: address_of
   =         s = signer{0x4823}
   =     at ./../../../../move-stdlib/sources/signer.move:13: address_of
   =         result = 0x4823
   =     at ./../../../../move-stdlib/sources/signer.move:14: address_of
   =     at ./sources/BasicCoin.move:66: withdraw (spec)
   =     at ./sources/BasicCoin.move:45: transfer
   =     at ./sources/BasicCoin.move:66: withdraw (spec)
   =     at ./sources/BasicCoin.move:57: withdraw
   =         addr = 0x4823
   =         amount = 0
   =     at ./sources/BasicCoin.move:58: withdraw
   =     at ./sources/BasicCoin.move:33: balance_of
   =         owner = 0x4823
   =     at ./sources/BasicCoin.move:34: balance_of
   =         ABORTED

Error: exiting with verification errors
```

This stack trace that bounces between the spec and the implementation is pretty rad.
It is even relying on the spec of the `withdraw` function
while testing the spec of the `transfer` function.

I wonder if that means I will run into problems having declined
to implement the `deposit` spec previously-mentioned in the tutorial,
since `transfer` calls both `withdraw` and `deposit`.

I add in the `deposit` spec to be safe,
and it doesn't fix this problem.

So this error tells me this is where the abort happened:

```
34 │           borrow_global<Balance<CoinType>>(owner).coin.value
   │           ------------- abort happened here with execution failure
```

This is within the `balance_of` method,
and the stack trace indicates it was called by `withdraw`.

This error message tells me everything I need to figure out the problem.
Impressive!

Of course I didn't specify that if `from` has _no_ balance
then that's an abort.
Obvious mistake, and the prover caught it.

I enhance my spec

```move
    spec transfer {
        let from_address = signer::address_of(from);

        aborts_if !exists<Balance<CoinType>>(from_address);
        aborts_if !exists<Balance<CoinType>>(to);

        let from_balance = global<Balance<CoinType>>(from_address).coin.value;
        aborts_if from_balance < amount;
    }
```

It still fails

```
$ move prove
[INFO] preparing module 0xcafe::BasicCoin
[INFO] transforming bytecode
[INFO] generating verification conditions
[INFO] 6 verification conditions
[INFO] running solver
[INFO] 0.010s build, 0.003s trafo, 0.004s gen, 1.055s verify, total 1.072s
error: abort not covered by any of the `aborts_if` clauses
   ┌─ ./sources/BasicCoin.move:49:5
   │
49 │ ╭     spec transfer {
50 │ │         let from_address = signer::address_of(from);
51 │ │
52 │ │         aborts_if !exists<Balance<CoinType>>(from_address);
   · │
56 │ │         aborts_if from_balance < amount;
57 │ │     }
   │ ╰─────^
   · │
81 │           *balance_ref = balance + value;
   │                                  - abort happened here with execution failure
   │
   =     at ./sources/BasicCoin.move:44: transfer
   =     at ./sources/BasicCoin.move:50: transfer (spec)
   =     at ./sources/BasicCoin.move:55: transfer (spec)
   =     at ./sources/BasicCoin.move:44: transfer
   =         from = signer{0x6784}
   =         to = 0x6785
   =         amount = 6335
   =         _witness = <generic>
   =     at ./sources/BasicCoin.move:45: transfer
   =     at ./../../../../move-stdlib/sources/signer.move:12: address_of
   =         s = signer{0x6784}
   =     at ./../../../../move-stdlib/sources/signer.move:13: address_of
   =         result = 0x6784
   =     at ./../../../../move-stdlib/sources/signer.move:14: address_of
   =     at ./sources/BasicCoin.move:68: withdraw (spec)
   =     at ./sources/BasicCoin.move:45: transfer
   =     at ./sources/BasicCoin.move:68: withdraw (spec)
   =     at ./sources/BasicCoin.move:59: withdraw
   =         addr = 0x6784
   =         amount = 6335
   =     at ./sources/BasicCoin.move:60: withdraw
   =     at ./sources/BasicCoin.move:33: balance_of
   =         owner = 0x6784
   =     at ./sources/BasicCoin.move:34: balance_of
   =         result = 6335
   =     at ./sources/BasicCoin.move:35: balance_of
   =         balance = 6335
   =     at ./sources/BasicCoin.move:61: withdraw
   =     at ./sources/BasicCoin.move:62: withdraw
   =         balance_ref = &6335
   =     at ./sources/BasicCoin.move:63: withdraw
   =     at ./sources/BasicCoin.move:64: withdraw
   =         result = BasicCoin.Coin{value = 6335}
   =     at ./sources/BasicCoin.move:65: withdraw
   =         check = BasicCoin.Coin{value = 6335}
   =     at ./sources/BasicCoin.move:85: deposit (spec)
   =     at ./sources/BasicCoin.move:86: deposit (spec)
   =     at ./sources/BasicCoin.move:46: transfer
   =     at ./sources/BasicCoin.move:85: deposit (spec)
   =     at ./sources/BasicCoin.move:86: deposit (spec)
   =     at ./sources/BasicCoin.move:77: deposit
   =         addr = 0x6785
   =         check = BasicCoin.Coin{value = 6335}
   =     at ./sources/BasicCoin.move:78: deposit
   =     at ./sources/BasicCoin.move:33: balance_of
   =         owner = 0x6785
   =     at ./sources/BasicCoin.move:34: balance_of
   =         result = 18446744073709545281
   =     at ./sources/BasicCoin.move:35: balance_of
   =         balance = 18446744073709545281
   =     at ./sources/BasicCoin.move:79: deposit
   =         balance_ref = &18446744073709545281
   =     at ./sources/BasicCoin.move:80: deposit
   =         value = 6335
   =     at ./sources/BasicCoin.move:81: deposit
   =         ABORTED

Error: exiting with verification errors
```

Just looking at the function on the top of the stack,
`deposit`,
I guess that I didn't account for overflow.

I am stumped on how to state
"aborts if `to_balance` + `amount` overflows".
I could write this if I had a constant for `u64::MAX`.

I want to solve this without looking at the solution.
Did the previous tutorial examples hint at the solution?

It did! Look at this part of the `deposit` spec:

```move
        aborts_if balance + check_value > MAX_U64;
```

`MAX_U64` must just exist in the default namespace.
Also, this spec expression is somehow able to overflow a `u64`,
expressing `balance + check_value > MAX_U64` naturally,
instead of something like `MAX_U64 - check_value < balance`.
Convenient ... but magical.

Where is `MAX_U64` defined?
Why can specs write overflowing expressions that "just work"?

I'll have to answer these some other time.

My new spec is:

```
    spec transfer {
        let from_address = signer::address_of(from);

        aborts_if !exists<Balance<CoinType>>(from_address);
        aborts_if !exists<Balance<CoinType>>(to);

        let from_balance = global<Balance<CoinType>>(from_address).coin.value;
        let to_balance = global<Balance<CoinType>>(to).coin.value;

        aborts_if from_balance < amount;
        aborts_if to_balance + amount > MAX_U64;
    }
```

It still doesn't work:

```
$ move prove
[INFO] preparing module 0xcafe::BasicCoin
[INFO] transforming bytecode
[INFO] generating verification conditions
[INFO] 6 verification conditions
[INFO] running solver
[INFO] 0.010s build, 0.003s trafo, 0.004s gen, 1.013s verify, total 1.030s
error: function does not abort under this condition
   ┌─ ./sources/BasicCoin.move:59:9
   │
59 │         aborts_if to_balance + amount > MAX_U64;
   │         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   │
   =     at ./sources/BasicCoin.move:44: transfer
   =     at ./sources/BasicCoin.move:50: transfer (spec)
   =     at ./sources/BasicCoin.move:55: transfer (spec)
   =     at ./sources/BasicCoin.move:56: transfer (spec)
   =     at ./sources/BasicCoin.move:44: transfer
   =         from = signer{0x0}
   =         to = 0x0
   =         amount = 2
   =         _witness = <generic>
   =     at ./sources/BasicCoin.move:45: transfer
   =     at ./../../../../move-stdlib/sources/signer.move:12: address_of
   =         s = signer{0x0}
   =     at ./../../../../move-stdlib/sources/signer.move:13: address_of
   =         result = 0x0
   =     at ./../../../../move-stdlib/sources/signer.move:14: address_of
   =     at ./sources/BasicCoin.move:71: withdraw (spec)
   =     at ./sources/BasicCoin.move:45: transfer
   =     at ./sources/BasicCoin.move:71: withdraw (spec)
   =     at ./sources/BasicCoin.move:62: withdraw
   =         addr = 0x0
   =         amount = 2
   =     at ./sources/BasicCoin.move:63: withdraw
   =     at ./sources/BasicCoin.move:33: balance_of
   =         owner = 0x0
   =     at ./sources/BasicCoin.move:34: balance_of
   =         result = 18446744073709551615
   =     at ./sources/BasicCoin.move:35: balance_of
   =         balance = 18446744073709551615
   =     at ./sources/BasicCoin.move:64: withdraw
   =     at ./sources/BasicCoin.move:65: withdraw
   =         balance_ref = &18446744073709551615
   =     at ./sources/BasicCoin.move:66: withdraw
   =     at ./sources/BasicCoin.move:67: withdraw
   =         result = BasicCoin.Coin{value = 2}
   =     at ./sources/BasicCoin.move:68: withdraw
   =         check = BasicCoin.Coin{value = 2}
   =     at ./sources/BasicCoin.move:88: deposit (spec)
   =     at ./sources/BasicCoin.move:89: deposit (spec)
   =     at ./sources/BasicCoin.move:46: transfer
   =     at ./sources/BasicCoin.move:88: deposit (spec)
   =     at ./sources/BasicCoin.move:89: deposit (spec)
   =     at ./sources/BasicCoin.move:80: deposit
   =         addr = 0x0
   =         check = BasicCoin.Coin{value = 2}
   =     at ./sources/BasicCoin.move:81: deposit
   =     at ./sources/BasicCoin.move:33: balance_of
   =         owner = 0x0
   =     at ./sources/BasicCoin.move:34: balance_of
   =         result = 18446744073709551613
   =     at ./sources/BasicCoin.move:35: balance_of
   =         balance = 18446744073709551613
   =     at ./sources/BasicCoin.move:82: deposit
   =         balance_ref = &18446744073709551613
   =     at ./sources/BasicCoin.move:83: deposit
   =         value = 2
   =     at ./sources/BasicCoin.move:84: deposit
   =     at ./sources/BasicCoin.move:85: deposit
   =     at ./sources/BasicCoin.move:47: transfer
   =     at ./sources/BasicCoin.move:52: transfer (spec)
   =     at ./sources/BasicCoin.move:53: transfer (spec)
   =     at ./sources/BasicCoin.move:58: transfer (spec)
   =     at ./sources/BasicCoin.move:59: transfer (spec)
```

These stack traces annotated with relevant values are awesome.

I am stumped about what I did wrong though.
I am writing the same check as in the (working) `deposit` spec.

Is `18446744073709551613 + 2` (the values used by the prover to disprove my spec)
greater than `MAX_U64`?

... no? The max `u64` value is 1844674407370955161_5_ so `18446744073709551613 + 2`
should be _equal to_ `MAX_U64`, not greater.

I don't want to look at the solution...

I restructure my spec to not rely on overflowing math,
just to be sure:

```
        aborts_if MAX_U64 - amount < to_balance;
```

Still fails the same way.

I don't get it.
I'm going to look at the solution.

... and in the process I discover I made a big mistake:
I have been editing and testing the _step 7_ code,
when I should be working on the _step 8_ code.

Yikes.

The actual step 8 `transfer` function:

```move
    /// Transfers `amount` of tokens from `from` to `to`. This method requires a witness with `CoinType` so that the
    /// module that owns `CoinType` can decide the transferring policy.
    public fun transfer<CoinType: drop>(from: &signer, to: address, amount: u64, _witness: CoinType) acquires Balance {
        let from_addr = signer::address_of(from);
        assert!(from_addr != to, EEQUAL_ADDR);
        let check = withdraw<CoinType>(from_addr, amount);
        deposit<CoinType>(to, check);
    }
```

This additional line is crucial,
and the reason my proof was failing in step 7:

```
        assert!(from_addr != to, EEQUAL_ADDR);
```

If `to` and `from` are the same then there is no possibility of overflow &mdash;
the amount is subtracted then added to the same account.
I quickly am able to finish the proof.

Embarassing.

Ok, that's as much Move as I am going to write just now.



## Solana speculation

From this brief experience I am hopeful that the Move storage model
can be translated to the Solana storage model:

Solana needs addresses of all storage to be calculated
prior to calling a program.
With Move having only static dispatch,
and using types to identify locations,
and all read effects to be annotated with `acquires TYPE`,
one can probably arrange to automatically derive
storage addresses from account addresses and "key" types.
That would seem to be a nice bit of serendipity between the Solana and Move models.

Solana's plan to integrate Move involves translating MVIR to LLVM IR.
I know from skimming [this blog post comparing Move and Solana](https://medium.com/@kklas/smart-contract-development-move-vs-rust-4d8f84754a8f)
that the author thinks that compiling Move via LLVM is not as good a solution as
running the Move VM directly,
I think because there is some required runtime validation the Move VM performs on Move bytecode.

I'd like to understand more about the validation the Move VM does,
so I can have a preliminary opinion about the problems Solana is going to run into
with the compile-to-LLVM approach.

After reading that article my understanding is that the Move bytecode verifyer
is performing various checks that what the bytecode is doing obeys the Move type system.

I have to hope that there is a solution to do the static checks of the bytecode verifyer dynamically.
For example, operations on types from outside of a module that must maintain invariants
could be translated to cross-program calls,
so that a trusted program,
either representing the foreign module,
or a centralized "runtime"
can do the verification.

At the moment I don't understand the problem enough to speculate.

