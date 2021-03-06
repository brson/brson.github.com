---
layout: post
title: First impressions of Rust programming on Solana
tags: [rust, blockchain]
---

Since trying out Rust programming on several other Rust blockchains,
[Aimee] and I have been looking forward to test-driving Solana.
And with Solana [hosting a hackathon][hackdocs] during May,
it was a good opportunity to give it a try.

[Aimee]: https://github.com/Aimeedeer
[hackdocs]: https://github.com/solana-labs/solana-season

During the hackathon we attempted to create a "sync bot",
that would sync the data from our in-development web app,
[Treasure Tree],
to the Solana blockchain.
Though it has a web server and centralized storage backend,
the app was developed with a simple data model
that was intended to be blockchain-compatible,
such that the whole thing could be implemented
without a web server.

[Treasure Tree]: https://github.com/brson/treasuretree


## Summaries

This blog is long and meandering,
but I've attempted to sum up useful observations here.

In general
we enjoyed the experience,
and it presented relatively few
frustrating obstacles to writing the code we wanted.
We felt excited about the code we were writing
and looked forward to writing more.


### Things we liked

- The main documentation at [docs.solana.com] is a good entry point for
  most Solana subjects, is up to date, and interesting to read.
- The [Solana Program Library][spl] is a great resource for learning
  Solana programming the hard way.
  It contains a bunch of production Solana code,
  but is also presented in a way that can be used as a learning tool.
  Smart use of resources.
- The Solana toolset and custom toolchain installed easily and
  didn't cause us any grief.
- Setting up a devnet is easy.
- Solana programs are called "programs", not "smart contracts".
- The Solana programming model doesn't impose typical smart contract abstractions
  on the implementation; it just gives you a low-level entry point
  function, a buffer of data for your program to interpret,
  and an SDK full of tools. No DSLs here.
- The developer channels on the Solana Discord are active.
- Although at the time it _felt_ like a lot of the questions we
  asked in Discord went unanswered, in retrospect, many of them
  did get some kind of answer. A healthy sign I think.


### Things we learned

- In general,
  you'll be ripgrepping the [Solana Program Library][spl] a lot
  to figure out how things work.
- For writing Rust client code,
  crib off of the [`feature_proposal` client][fpc].
  It is relatively simple.
- For Rust RPC clients, of the seemingly multiple options in the `solana_client`
  crate, use `RpcClient`. That is the type used in the SPL. We initially tried to use
  `ThinClient`, but that doesn't seem to be the right type to use.
- The [`solana_cli_config`] crate will help you read the contents of Solana's config files.
- Call `solana_logger::setup_with("solana=debug");` before your program starts,
  or set the envvar `RUST_LOG=solana_client=debug`. This will show the logs
  from your program when it errors.
- When your program succeeds the logs show up in the output of the `solana logs`
  command.
- Use [`try_from_slice_unchecked`] to deserialize from buffers that are
  larger than the exact size of the serialized object. [`try_from_slice`]
  panics when the buffer is not exactly the right size.
- The instruction budget is very limited &mdash;
  we were not able to verify a single K-256 ECDSA signature on-chain
  within the limit, as such:
- Doing your own signature verification in-program is probably not
  the way to use Solana. The Solana runtime can verify multiple account signatures
  before your program ever runs, so a Solana program seemingly needs to be designed
  to leverage that capability when signature verification is needed.
- Solana programs have a pre-allocated and bounded storage size.
  This means it is probably an anti-pattern to store key-value maps
  directly. What seems to be an intended pattern for key-value storage
  is to derive the keys as Solana accounts from a single base account,
  and store the values in those accounts.

[docs.solana.com]: https://docs.solana.com
[fpc]: https://github.com/solana-labs/solana-program-library/blob/master/feature-proposal/cli/src/main.rs
[spl]: https://github.com/solana-labs/solana-program-library
[solscript]: https://gist.github.com/brson/29f82547df862161dda8cbc92de6ab37#file-solana-script-sh-L19
[`try_from_slice_unchecked`]: https://docs.rs/solana-sdk/1.6.9/solana_sdk/borsh/fn.try_from_slice_unchecked.html
[`try_from_slice`]: https://docs.rs/borsh/0.8.2/borsh/de/trait.BorshDeserialize.html#method.try_from_slice

### Things that annoyed us

- The Rust API documentation is insufficient.
- Solana programs have access to the standard library,
  but it is a not-quite compatible version of the standard library.
  This was the source of multiple confusions, including:
- `HashMap` seems to just fail on any operation,
  which manifests as a mysterious access violation.
  We spent hours looking for our bug when we should have just not
  used `HashMap`; and
- `time` and `anyhow` don't build against Solana's `std`.
  `anyhow` can be built with the `std` feature off, but that loses
  compatibility with the `Error` trait.
- At least one function, `read_keypair_file`, returned `Box<dyn Error>`,
  which can't be trivially converted to `anyhow::Error` with the `?` operator.
  Errors in Rust really need to be `Error + Send + Sync + 'static` unless there
  is great reason not to.
- 4K BPF stack frames means some crates don't work,
  including `ed25519-dalek`.
  Big stack frames trigger an access violation at runtime.
  First time I've ever encountered a limit like this.
- `cargo build-bpf` behaves differently than `cargo-build` in several situations,
  including missing the `-p` flag, and not working with libraries
  containing hyphens in their name.
- As with previous experiences, we found the online hackathon format
  uninspiring, and mostly didn't participate.
- It felt like many questions on Discord went unanswered.
  This was annoying,
  but also understandable, and not out of the ordinary for an
  open source project. Sometimes nobody steps up to help.
  It just happens, but is discouraging.


### Questions we still have

- How do unimplemented features of `std` manifest at runtime?
  Errors? Panics? Aborts? Is it consistent?
- What is `ThinClient` in `solana_client` for?
  We tried to use it but that seemed to be a wrong choice.
- What does "spinner" mean in the context of
  [`RpcClient::send_and_confirm_transaction_with_spinner`]?
- What is the best pattern for initializing a Solana program's state?
  We reserved a single byte at the beginning of our account data
  to use as an "initialized" flag.


### The outcome

It went pretty well!

We managed to create a "sync bot" that was able to mirror
the state of our centralized web app on the blockchain,
and set it up on a server at [treasuretree.org],
running against a local devnet.

[treasuretree.org]: https://treasuretree.org

We did find that some of our assumptions about the types of code
we could run on-chain were wrong. In particular we:

- planned to verify signatures ourselves
- planned to store unbounded key-value maps

Neither of these are easy or proper to do on Solana:
signature verification just requires too many CPU cycles;
and programs have fixed storage space.

As it stands our Solana program does not properly verify the
signatures it needs to &mdash;
it just accepts its input as valid.
To fix this we need to restructure our cryptographic code within the rest
of our application to do more Solana-specific signing operations.
This is a bit of a bummer, as we were hoping to keep
the application logic blockchain-agnostic.

We also didn't explore making our in-game "treasures" work as NFTs.
Just ran out of time.

We plan to continue hacking on this until we get
it running on the mainnet.


## Table of contents

- [The first day's plan](#user-content-the-first-days-plan)
- [Installing the Solana SDK](#user-content-installing-the-solana-sdk)
- [Looking at the install script](#user-content-looking-at-the-install-script)
- [Running a devnet](#user-content-running-a-devnet)
- [Why eBPF?](#user-content-why-ebpf)
- [The Helloworld application](#user-content-the-helloworld-application)
- [A look at the Rust contract](#user-content-a-look-at-the-rust-contract)
- [Integrating Solana into our project](#user-content-integrating-solana-into-our-project)
- [Writing a Solana program in Rust](#user-content-writing-a-solana-program-in-rust)
- [Writing a Solana client in Rust](#user-content-writing-a-solana-client-in-rust)
- [Reproducing the Helloworld example client but in Rust](#user-content-reproducing-the-helloworld-example-client-but-in-rust)
- [Building a Solana instruction](#user-content-building-a-solana-instruction)
- [Getting beneath the instruction limit](#user-content-getting-beneath-the-instruction-limit)
- [A few more obstacles](#user-content-a-few-more-obstacles)
- [Wrapping it up](#user-content-wrapping-it-up)


## The first day's plan

Today is the 17th.
The hackathon began on the 15th,
but we're just starting now after moving Airbnbs.

Our plan today is to join the Solana hackathon Discord channels,
install the Solana SDK,
and run some hello world example.

While we get started,
we open [one of the video sessions we missed][tw],
this one an intro to "the Solana programming model".

[tw]: https://www.twitch.tv/videos/1021435633

We learn

- that "everything is an account"
- storage costs rent
- accounts can be pre-paid for 2 years
- contracts don't hold state (not sure what that means yet)
- at least some tokens don't require creating new contracts
- program-derived accounts don't have a knowable private key,
  but can still sign transactions (or something).
  This sounds like it solves a problem I have been having
  understanding how to build distributed applications,
  but I'll have to investigate further.
- the [solana-program-library] is something I should get to know.

[solana-program-library]: https://github.com/solana-labs/solana-program-library


## Installing the Solana SDK

I want to install the Solana SDK, tools, or whatever I need.
I assume there's something SDK-like I need.

The [hackathon docs][hackdocs] contains some educational links,
and I folow the first to the [Solana docs website][soldocs].

[soldocs]: https://docs.solana.com/

The first link is ["Start Building"][start],
and that sounds good.

[start]: https://docs.solana.com/developing/programming-model/overview

This page says "To start developing immediately you can build, deploy, and run one of the examples."
That's exactly what we want to do.

So now we're following the [helloworld example][he] docs.

[he]: https://docs.solana.com/developing/on-chain-programs/examples

I run

```
$ git clone https://github.com/solana-labs/example-helloworld.git
```

And am directed to continue by following the [hello-world README][hwrm],
so I do so.

[hwrm]: https://github.com/solana-labs/example-helloworld/blob/master/README.md

This flow is a bit confusing,
as the "examples" page proceeds directly from "Helloworld",
to "Break",
which is an entirely different example.
Though the instructions do say to continue to the Helloworld readme,
Aimee succumbs to this confusion,
and proceeds to try to follow "Break".
The correct thing to do is stop here,
navigate to the Helloworld readme,
and continue there.

This example is so well documented
that the README has a table of contents.
I am encouraged by this.

This example requires node > 14.
Aimee immediately runs into problems on mac:
she seems to have node 10,
tries to upgrade with brew,
but brew complains because her node and yarn
are not from brew.
This happens all the time with her computer &mdash;
she has forgotten how she installed node last time.
We discover she has nvm installed and sort things out.
We're both on node v16.1.0.

We run these things:

```
$ nvm install stable
$ nvm use stable
$ rustup update stable
```

And now, "install Solana v1.6.6 or later",
the instructions for which are
[back on the Solana docs site][tooldocs].

[tooldocs]: https://docs.solana.com/cli/install-solana-cli-tools

I run

```
$ sh -c "$(curl -sSfL https://release.solana.com/v1.6.9/install)"
```


## Looking at the install script


While that is running,
I download and read the script,
which I [have gisted here][shgist].

[shgist]: https://gist.github.com/brson/29f82547df862161dda8cbc92de6ab37

I open it and am _so pleased_.

At the top I see:

```sh
# Copyright 2016 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
```

and then

```sh
# This is just a little script that can be downloaded from the internet to
# install solana-install. It just does platform detection, downloads the installer
# and runs it.
```

This is a fork of [rustup's install script][rustup-init],
which I wrote,
and which,
[as I discovered previously][dfin],
Dfinity also uses.

[rustup-init]: https://github.com/rust-lang/rustup/blob/0eaf2f92b6016447210f2defccaf4bfcc1f4e9ff/rustup-init.sh
[dfin]: https://brson.github.io/2021/01/30/dfinity-impressions

I see a peculiar comment at the top:

```sh
{ # this ensures the entire script is downloaded #
```

This isn't in the `rustup-init.sh` script.
I haven't seen this technique before of just throwing a set
of braces around the entire shell script,
and wonder how compatible it is.

Scripts downloaded from the internet and immediately piped through
an interpreter need to guard against issues where the script
fails to download fully and so is only partially executed.
That's what this brace is about.

`rustup-init.sh` attempts to achieve this by essentially not executing
any code until a single function call at the end of the script.
And so I am slightly afronted that somebody just threw braces
around the whole thing,
as though saying my original work was insufficient.
Alas,
I can imagine a scenario where the brace-around-everything technique
could catch an error that `rustup-init.sh` doesn`t,
so this is something to consider if I ever find myself writing
a shell script to be piped from the internet again;
which I hope I never do.


## Running a devnet

The install succeeds and I have the Solana tools:

```
$ solana --version
solana-cli 1.6.9 (src:9e42883d; feat:2960423209)
```

I continue to configure the CLI per the Helloworld readme:

```
$ solana config set --url localhost
Config File: /home/brian/.config/solana/cli/config.yml
RPC URL: http://localhost:8899
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /home/brian/.config/solana/id.json
Commitment: confirmed
```

What does this tell us?

- We have a global config file,
  for me it's in the XDG-standard `~/.config/solana` directory,
- Solana's default RPC port is 8899,
- Nodes can also communicate over WebSockets
- We have an account keypair set up for us,
  the exact purpose for which is not clear,
  but I assume is a dev keypair
- "Commitment: confirmed" &mdash; I have no idea.

The next instruction is to create a keypair,
but `solana config` says I already have a keypair.
I am guessing I have previously installed the Solana SDK,
and it is an old keypair.
I begin to generate a new one:

```
$ solana-keygen new
Generating a new keypair

For added security, enter a BIP39 passphrase

NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text

BIP39 Passphrase (empty for none):
```

This is going to be a dev key and I don't care about recovering it.
I leave the passphrase blank and just hit "enter".

```
Wrote new keypair to /home/brian/.config/solana/id.json
==================================================================================
pubkey: C4NEJZc432PWEDvYR6LiBWCv7wvfdJWDppLscL42R3aD
==================================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
engage possible prison twin control language talk cactus hobby vehicle allow blush
==================================================================================
```

Again, this is a garbage dev key.
I am fine printing the seed phrase on the internet.

I run a devnet node in one terminal:

```
$ solana-test-validator
Ledger location: test-ledger
Log: test-ledger/validator.log
Identity: GZr7zHFUxA7kjGgzUsUuRfQtNASBCGurynEg7yUDcfvP
Genesis Hash: F945qQyeHDUXN58eUWuLHLogAZ7Qgkpucc7xe8LisQnR
Version: 1.6.9
Shred Version: 54687
Gossip Address: 127.0.0.1:1025
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
⠒ 00:00:08 | Processed Slot: 16 | Confirmed Slot: 16 | Finalized Slot: 0 | Snapshot Slot: - | Transaction
```

And the log monitor in another:

```
$ solana logs
Streaming transaction logs. Confirmed commitment
```

Nother further happens.
Probably because there are no transactions on my devnet.

That's as far as I get for the night.

I like everything I'm seeing so far:
the docs have directed me well,
the tooling has worked cleanly.


## Why eBPF?

After reading [about Solana's use of BPF][bpfdoc],
I ask in their Discord a question that I have been wondering for a few weeks:

[bpfdoc]: https://docs.solana.com/developing/on-chain-programs/overview

> Can somebody from Solana remind me why Solana uses BPF as its instruction set?
  I thought it was because BPF was not turing complete and programs could be
  verified to terminate, but I've recently been informed that is not the case

Nobody answers,
but I may have asked in the wrong channel.

A ask again in `#hack-blockchain-support`:

> What advantages does solana get from targeting eBPF, vs any other instruction
  set, for its VM? Been trying to get an answer to this for awhile.

Somebody responds by pinging "chase || solana",
and asking them to ask a Solana dev to explain,
but that explanation never comes.

Some days later I ask in `#developer-support`:

> What advantages does Solana get from targeting BPF vs some other VM?

"starry" says

> fast JIT compiling

I am frustrated.
This is an insufficient answer.

I ask starry

> Are there any blogs or benchmarks about solana's jit compiler I can check out?

They say

> not yet, we haven't enabled it yet

After some shower-based ruminating,
this answer though does jog a memory.
I have spoken with Anatoly about this subject in the distant past,
and I recall now that the BPF instruction set has a design that is easy to translate to x86.
So it might be that simple JITting and performance is the reason for choosing BPF.

Interestingly,
I believe this is also part of the reason [Nervos]
uses RISC-V &mdash;
its simple register and instruction set is easy to map to x86.

[Nervos]: https://github.com/nervosnetwork




## The Helloworld application

It's 5/19 now.
[Next step][nshw] in the Helloworld tutorial is to build and run the application.
The build is driven by npm, but includes cargo components as well.

[nshw]: https://github.com/solana-labs/example-helloworld/blob/master/README.md#install-npm-dependencies

I run `npm install` and see this warning:

```
: ~/solana/example-helloworld
$ npm install
npm WARN EBADENGINE Unsupported engine {
npm WARN EBADENGINE   package: 'helloworld@0.0.1',
npm WARN EBADENGINE   required: { node: '12.x' },
npm WARN EBADENGINE   current: { node: 'v15.10.0', npm: '7.5.3' }
npm WARN EBADENGINE }
```

There is a problem here:
I have node v15, and the build gives me a warning that v12 is required,
but the readme for this demo says "v14 recommended".
So it appears that either the package should be updated to not warn on later versions,
or the docs should say "v12 recommended".

The install succeeds.

Next step is to build the Rust program:

```
$ npm run build:program-rust
```

And as part of this,
the build runs `cargo build-bpf`.
I am guessing the `build-bpf` subcommand was installed
during `npm install`,
which ran a cargo build.
`cargo build-bpf` downloads the Solana BPF SDK:

```
$ npm run build:program-rust
> helloworld@0.0.1 build:program-rust
> cargo build-bpf --manifest-path=./src/program-rust/Cargo.toml --bpf-out-dir=dist/program

BPF SDK: /home/brian/.local/share/solana/install/releases/1.6.9/solana-release/bin/sdk/bpf
```

I deploy the program:

```
$ solana program deploy dist/program/helloworld.so
Program Id: 2jW9jdWSwqkM2rznH5MwL65obzMoHUHZsxpUmjAULsmq
```

And the log fills up with transactions:

```
...
Transaction executed in slot 2006:
  Signature: 2jP1sCQ2CVzizHzc7zgajwf2E8yU2x4RhSyeL59mvFio8UvkyS4rcxyVaNyan5w7UCm3cZBsYefoETBD7DuZbbBt
  Status: Ok
  Log Messages:
    Program 11111111111111111111111111111111 invoke [1]
    Program 11111111111111111111111111111111 success
    Program 11111111111111111111111111111111 invoke [2]
    Program 11111111111111111111111111111111 success
    Deployed program 2jW9jdWSwqkM2rznH5MwL65obzMoHUHZsxpUmjAULsmq
```

I run the client JavaScript app,
and run into an error:

```
$ npm run start

> helloworld@0.0.1 start
> ts-node src/client/main.ts

node:internal/modules/cjs/loader:926
  throw err;
  ^

Error: Cannot find module 'arg'
Require stack:
- /home/brian/solana/example-helloworld/node_modules/ts-node/dist/bin.js
    at Function.Module._resolveFilename (node:internal/modules/cjs/loader:923:15)
    at Function.Module._load (node:internal/modules/cjs/loader:768:27)
    at Module.require (node:internal/modules/cjs/loader:995:19)
    at require (node:internal/modules/cjs/helpers:92:18)
    at Object.<anonymous> (/home/brian/solana/example-helloworld/node_modules/ts-node/dist/bin.js:8:13)
    at Module._compile (node:internal/modules/cjs/loader:1091:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1120:10)
    at Module.load (node:internal/modules/cjs/loader:971:32)
    at Function.Module._load (node:internal/modules/cjs/loader:812:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12) {
  code: 'MODULE_NOT_FOUND',
  requireStack: [
    '/home/brian/solana/example-helloworld/node_modules/ts-node/dist/bin.js'
  ]
}
npm ERR! code 1
npm ERR! path /home/brian/solana/example-helloworld
npm ERR! command failed
npm ERR! command sh -c ts-node src/client/main.ts

npm ERR! A complete log of this run can be found in:
npm ERR!     /home/brian/.npm/_logs/2021-05-19T01_48_31_041Z-debug.log
```

Hm, "cannot find module 'arg'".

I google "cannot find module arg".
Nothing.
I run `git pull` to check for bugfixes.
Nothing.
I see that I have some dirty files in my tree.
So I decide to start over,
run `git reset --hard origin/master && rm node_modules && rm dist`.

I go through the `npm install` / `npm run build:program-rust` / `solana program deploy` / `npm run start`
sequence again.

This time it works:

```
$ npm run start

> helloworld@0.0.1 start
> ts-node src/client/main.ts

Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 2960423209, 'solana-core': '1.6.9' }
Using account C4NEJZc432PWEDvYR6LiBWCv7wvfdJWDppLscL42R3aD containing 499999998.27048796 SOL to pay for fees
Using program 7NMkTRVNtBvuC68BVeamnXmuqcpiVeLQgNDGzQrPceTN
Creating account 6GeXM3KjbPJ7pXQoDri2f2YPzZXGts2ewHiVRFGuaZWt to say hello to
Saying hello to 6GeXM3KjbPJ7pXQoDri2f2YPzZXGts2ewHiVRFGuaZWt
6GeXM3KjbPJ7pXQoDri2f2YPzZXGts2ewHiVRFGuaZWt has been greeted 1 time(s)
Success
```

Ok.
I'll never know what I did wrong.
Love it when that happens,
but that's hacking.



## A look at the Rust contract

The Rust contract [is nice and short][hct].
Here's the whole thing minus imports and tests:

[hct]: https://github.com/solana-labs/example-helloworld/blob/master/src/program-rust/src/lib.rs

```rust
// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}
```

I like this a lot:
there's very little magic here &mdash;
you define an entry point,
and you get a blob of instruction data,
and its up to you to use the SDK to interpret it.

The `entrypoint!` macro [does very little][epm].
This is all of it:

```rust
#[macro_export]
macro_rules! entrypoint {
    ($process_instruction:ident) => {
        /// # Safety
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            let (program_id, accounts, instruction_data) =
                unsafe { $crate::entrypoint::deserialize(input) };
            match $process_instruction(&program_id, &accounts, &instruction_data) {
                Ok(()) => $crate::entrypoint::SUCCESS,
                Err(error) => error.into(),
            }
        }
        $crate::custom_heap_default!();
        $crate::custom_panic_default!();
    };
}
```

[epm]: https://github.com/solana-labs/solana/blob/a911ae00baa9bb7031041add14c5185c86376afb/sdk/program/src/entrypoint.rs#L42

Personally, I like to understand what is happening under the hood,
and dislike hiding magic in macros where reasonable,
so I'm encouraged by this.

This readme has [a section explaining what the program does][hwex]!

[hwex]: https://github.com/solana-labs/example-helloworld/blob/master/README.md#entrypoint

This is awesome.

We spend some time reading through the Rust and TypeScript source code on GitHub,
but I am not outlining our thoughts about it here.

All of the links to code in the "learn about the client" section of the readme
link to old commits,
and it is a bit confusing.

I [submit a PR][docpr] to update the docs.

[docpr]: https://github.com/solana-labs/example-helloworld/pull/217

I also discover that part of the `npm install` process runs `cargo update`,
which leaves my lockfile dirty.
I submit [a PR to update the lockfile][lockfile].

[lockfile]: https://github.com/solana-labs/example-helloworld/pull/216


## Integrating Solana into our project

Now that we have the tools,
and a basic understanding of how to set up a Solana program and client,
let's think about integrating Solana into our own project.

The project is called [Treasure Tree][treasuretree],
and it is a real-world treasure hunt where the treasures are NFTs.
In it,

- treasure _planters_ physically plant QR codes containing secret keys,
- treasure _claimers_ find and scan those QR codes to claim them on the network, as NFTs.

As such, there are only two verbs in our app,
_plant_, and _claim_, and executing either
involves the creation and verification of a few cryptographic signatures.

[treasuretree]: https://github.com/brson/treasuretree

We have already prototyped the application as a conventional webapp using [Rocket].
Our goal for this hackathon is to implement the two verbs, plant and claim,
in a Solana program;
make the treasures transactable as NFTs;
and to create a service that syncs the state of these treasures
from the centralized service onto the blockchain.

[Rocket]: https://rocket.rs

Note that internally the project is called "geonft",
which code snippits herein will reference.


### Writing a Solana program in Rust

Aimee is responsible for writing our Solana program
that handles the "plant" and "claim" actions.
She has written these two functions previously in our Rocket backend,
and now she is reimplementing them on-chain.

We immediately run into a surprising error:

```
$ cargo build-bpf
Failed to obtain package metadata: Error during execution of `cargo metadata`: error: failed to parse manifest at `/<local_path>/geonft/src/solanaprogram/Cargo.toml`

Caused by:
  library target names cannot contain hyphens: solana-program
  ```

Seems like a limitation of `cargo build-bpf` &mdash;
standard cargo doesn't do this.
Fixing it is simple &mdash;
just change the crate name to not contain a hyphen.

We run into a problem when adding our `geonft_solana` program to a workspace.
Because we are trying to use the `anyhow` crate in multiple crates,
and with multiple targets,
some no-std,
we hit a problem where running `cargo build-bpf`
ends up running cargo in such a way that the `anyhow` crate
incorrectly gets resolved to use its `std` feature,
and so doesn't build,
seemingly because of missing backtraces
in the Solana implementation of `std`.

(That `anyhow` doesn't build with `cargo build-bpf`
without disabling the `std` feature is also surprising,
as the Solana BPF target includes `std`.)

We add `resolver = "2"` to our workspace to fix this,
but find an additional wrinkle that the bpf toolchain is still
on Rust 1.50,
where the v2 crate resolver was unstable,
so have also add `carge-features = ["resolver"]`.

Futthermore,
`cargo build-bpf` doesn't accept a `-p` parameter,
so it seems like a bpf program probably shouldn't be part of a workspace.
For now we do have it in our workspace though,
and just `cd` into the `geonft_solana` directory to build it.

When we add ed25519-dalek to our dependencies we start seeing errors
about large stack frames,
but the build still succeeds.

They look like this:

```
$ cargo build-bpf
BPF SDK: /<local_path>/solana/install/releases/1.6.9/solana-release/bin/sdk/bpf
Running: rustup toolchain list -v
Running: cargo +bpf build --target bpfel-unknown-unknown --release
warning: /<local_path>/geonft/src/geonft/Cargo.toml: unused manifest key: global
   Compiling serde v1.0.126
   Compiling curve25519-dalek v3.1.0
   Compiling ed25519-dalek v1.0.1
Error: Function _ZN209_$LT$curve25519_dalek..window..NafLookupTable8$LT$curve25519_dalek..backend..serial..curve_models..ProjectiveNielsPoint$GT$$u20$as$u20$core..convert..From$LT$$RF$curve25519_dalek..edwards..EdwardsPoint$GT$$GT$4from17hbabee800aa927908E Stack offset of -10920 exceeded max offset of -4096 by 6824 bytes, please minimize large stack variables

```

I ask in `#hack-rust-support`

> After adding ed25519-dalek crate to my solana program, I get (non-fatal) errors when building that say "Stack
 offset of -7680 exceeded max offset of -4096 by 3584 bytes, please minimize large stack variables". What can I do about this?

I am thinking the `#hack-*` channels are low volume and not the place
to be asking Solana dev questions.
All the normal dev channels require proper permissions to talk in them,
and I don't see how to get those permissions,
so I ask in `#hack-questions`:

> Can I get access to the #developer-support channel?

Somebody named "Metric O'Forte" explains to me that I need to go to the `#welcome` channel
and follow the instructions from Carl-bot to become a developer. So I do that.

Still waiting on solutions to the stack offset problem,
but now that I'm a developer and can talk in `#developer-support` I ask there:

> After adding ed25519-dalek crate to my solana program, I get errors when building that say "Stack
  offset of -7680 exceeded max offset of -4096 by 3584 bytes, please minimize large stack variables".
  What can I do about this?

We'll get back to the Solana program later.






## Writing a Solana client in Rust

I am writing a program whose job is to sync the application state
from our centralized Rocket application to Solana.
I put it in a new crate, [`geonft_sync`].

[`geonft_sync`]: https://github.com/brson/geonft/blob/master/src/geonft_sync/src/main.rs

I gather that I'm going to need the [`solana-sdk`] and [`solana-client`]
crates.
The documentation for these is not great,
and there's not an obvious example of a Rust Solana client:
the Helloworld example uses a TypeScript client.

[`solana-sdk`]: https://docs.rs/solana-sdk/1.6.9/solana_sdk/
[`solana-client`]: https://docs.rs/solana-client/1.6.9/solana_client/

I ask in `#hack-rust-support`:

> Are there any examples writing a Solana client in Rust, using solana_sdk
  and/or solana_client?

"jon" tells me

> Sure! You can take a look at the token CLI as an
> example:
>
> https://github.com/solana-labs/solana-program-library/tree/master/token/cli
>
> or even the feature-proposal program:
>
> https://github.com/solana-labs/solana-program-library/blob/master/feature-proposal/cli/src/main.rs

The `solana_client` crate has several clients,
but I am eyeing [`ThinClient`] as the one I "should" use,
just on a hunch.

[`ThinClient`]: https://docs.rs/solana-client/1.6.9/solana_client/thin_client/index.html

The constructor though has an argument I don't intuitively know what do do with:

```rust
pub fn create_client_with_timeout(
    (rpc, tpu): (SocketAddr, SocketAddr),
    range: (u16, u16),
    timeout: Duration
) -> ThinClient
```

The RPC ond TPU ("transaction processing unit") sockets
are printed by `solana-test-validator` on startup,
but I don't know what the `range` tuple is.
Clicking through the docs to [the source of `create_client_with_timeout`][ccwtos],
I see this tuple is passed to [`solana_net_utils::bind_in_range`][bin]
to create a UDP socket,
and that is passed to the underlying `ThinClient` constructor.

[ccwtos]: https://docs.rs/solana-client/1.6.9/src/solana_client/thin_client.rs.html#619
[bin]: https://docs.rs/solana-net-utils/1.6.9/src/solana_net_utils/lib.rs.html#412-428

So this range is just a UDP port range to attempt listening on.

I hack this together and it works:

```rust
    let rpc_addr = "127.0.0.1:8899";
    let tpu_addr = "127.0.0.1:1027";
    let tx_port_range = (10_000_u16, 20_000_u16);
    let timeout = 1000;

    info!("connecting to solana node, RPC: {}, TPU: {}, tx range: {}-{}, timeout: {}ms",
          rpc_addr, tpu_addr, tx_port_range.0, tx_port_range.1, timeout);

    let rpc_addr: SocketAddr = rpc_addr.parse().expect("");
    let tpu_addr: SocketAddr = tpu_addr.parse().expect("");

    let client = thin_client::create_client_with_timeout(
        (rpc_addr, tpu_addr),
        tx_port_range,
        Duration::from_millis(timeout));

    let epoch = client.get_epoch_info()?;

    info!("{:?}", epoch);
```

It prints

```
[2021-05-22T02:36:59Z INFO  geonft_sync] EpochInfo { epoch: 0, slot_index: 32145, slots_in_epoch: 432000, absolute_slot: 32145, block_height: 32144, transaction_count: Some(32143) }
```

So even without adequate docs it was pretty easy to figure
out how to connect to a solana node and query something.



## Reproducing the Helloworld example client but in Rust

While there are Rust client programs in the SPL,
there isn't a Helloworld equivalent client written in Rust.

I decide to proceed by following the [Helloworld typescript client][tsc],
and trying to do what it does step by step,
while using Rust APIs.

[tsc]: https://github.com/solana-labs/example-helloworld/tree/master/src/client

The first thing it does is "establish a connection",
and while I've already written code for that,
the TypeScript code also calls the `getVersion` RPC method,
which seems like a better way to smoke-test the connection than
calling `get_epoch_info` like I am now.

So I refactor my code to create an `establish_connection` method,
and look for how to call `getVersion` from Rust.
I don't see it on the `SyncClient` trait,
but [searching "version" in the `solana_client`][versearch] API docs
reveals a `get_version` method on the `RpcClient` struct.
I have a `ThinClient`. How do I get an `RpcClient` from that?

[versearch]: https://docs.rs/solana-client/1.6.9/solana_client/index.html?search=version

I assume a `ThinClient` encapsulates an `RpcClient`,
and I can get a reference to its `RpcClient` somehow.

I read the code again for `ThinClient`.
It contains multiple `RpcClient`s.
I wonder if I should be using `RpcClient` directly and not `ThinClient`,
though `RpcClient` doesn't implement the `SyncClient` trait that it
seems like I would want access to.
I just don't see a way to access `ThinClient`'s `RpcClient`,
and since some of the methods on `ThinClient` are just
delegating to `RpcClient` I suspect the API is a bit underbaked.
I think maybe I can't submit transactions with just `RpcClient`,
but for now I need access to `RpcClient` so I am going
to create that instead of `ThinClient`.

I end up with this for `establish_client`:

```rust
pub fn establish_connection() -> Result<RpcClient> {
    let rpc_addr = "127.0.0.1:8899";
    let timeout = 1000;

    info!("connecting to solana node, RPC: {}, timeout: {}ms",
          rpc_addr, timeout);

    let rpc_addr: SocketAddr = rpc_addr.parse().expect("");

    let client = RpcClient::new_socket_with_timeout(rpc_addr, Duration::from_millis(timeout));

    let version = client.get_version()?;
    info!("RPC version: {:?}", version);

    Ok(client)
}
```

Next step is to make an equivalent of `establishPayer`.
The Helloworld tries to read a keypair from the CLI config,
and if that doesn't exist creates a new account.
For my purpsose I don't want to be creating arbitrary accounts,
so I'll just fail if the CLI config doesn't contain an acocunt.
Furthermore,
my CLI config lists the keypair path as `/home/brian/.config/solana/id.json`,
and that seems like a pretty stable name,
so I'm just going to avoid parsing the CLI config,
and try to load from there.

The TypeScript code contains an `Account` type that can parse the keypair
file more-or-less directly,
but I don't see such a thing in the Rust SDK.
Oh, wait, `id.json` is only barely JSON &mdash;
it just contains an array of bytes,
and there's an undocumented `deserialize_data` function on [`Account`]
which will presumably parse that blob.

[`Account`]: https://docs.rs/solana-sdk/1.6.9/solana_sdk/account/struct.Account.html

After some further hacking I realize that `Account` is deserializeble via serde,
but I'm beginning to think this `Account` is not the same as the `Account` type
in the TypeScript API.

I'm doing something wrong.

I take a look at the source for the ["SPL Token program command line utility"][splt].
From this I immediately discover the [`solana_cli_config`] crate,
which seems useful,
but upon inspection doesn't obviously get me from a keypair file to something usable in-memory.

[splt]: https://github.com/solana-labs/solana-program-library/tree/master/token/cli
[`solana_cli_config`]: https://docs.rs/solana-cli-config/1.6.9/solana_cli_config/

OK, there are a lot of concepts to digest in this token program.
I am feeling overwhelmed.
I look at the ["feature proposal" CLI][fpc] in hopes it is smaller.

[fpc]: https://github.com/solana-labs/solana-program-library/blob/master/feature-proposal/cli/src/main.rs

Yeah, this is easier to crib from.
And now I gather that the structure of the TypeScript Helloworld client
isn't really appropriate for a Rust client.
I basically need to start over.
This time I'm going to use the SDK to load the CLI config file,
pull the RPC address and keypair file out of that,
and use the SDK to load the keypair.

I run into problems with the error types returned by the SDK.
The function `read_keypair_file` returns `Box<dyn Error>`:

```rust
pub fn read_keypair_file<F: AsRef<Path>>(
    path: F
) -> Result<Keypair, Box<dyn Error>>
```

`Error` is too weak of a trait bound to be compatible with common
error handling patterns like those from the `anyhow` crate,
and when I try to trivially use `?` I get this error:

```
error[E0277]: `dyn std::error::Error` cannot be shared between threads safely
  --> src/geonft_sync/src/solana.rs:23:62
   |
23 |     let keypair = read_keypair_file(&cli_config.keypair_path)?;
   |                                                              ^ `dyn std::error::Error` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `dyn std::error::Error`
   = note: required because of the requirements on the impl of `Sync` for `Unique<dyn std::error::Error>`
   = note: required because it appears within the type `Box<dyn std::error::Error>`
   = note: required because of the requirements on the impl of `From<Box<dyn std::error::Error>>` for `anyhow::Error`
   = note: required because of the requirements on the impl of `FromResidual<Result<std::convert::Infallible, Box<dyn std::error::Error>>>` for `Result<solana::Config, anyhow::Error>`
   = note: required by `from_residual`
```

Errors in Rust that are not `Error + Send + Sync + 'static` are a pain.

After some hacking,
cribbing from both the "feature proposal" Rust client
and the "Helloworld" TypeScript client,
I arrive at three functions
that: load the CLI config and the paying account keypair,
connect to the Solana node and run a sanity check,
and load the program keypair and verify the program is deployed:

```rust
pub struct Config {
    json_rpc_url: String,
    keypair: Keypair,
}

pub fn load_config() -> Result<Config> {
    let config_file = solana_cli_config::CONFIG_FILE.as_ref().ok_or_else(|| anyhow!("config file path"))?;
    let cli_config = solana_cli_config::Config::load(&config_file)?;
    let json_rpc_url = cli_config.json_rpc_url;
    let keypair = read_keypair_file(&cli_config.keypair_path).map_err(|e| anyhow!("{}", e))?;
    Ok(Config {
        json_rpc_url,
        keypair,
    })
}

pub fn connect(config: &Config) -> Result<RpcClient> {

    info!("connecting to solana node at {}", config.json_rpc_url);
    let client = RpcClient::new_with_commitment(config.json_rpc_url.clone(), CommitmentConfig::confirmed());

    let version = client.get_version()?;
    info!("RPC version: {:?}", version);

    Ok(client)
}

static DEPLOY_PATH: &str = "target/deploy";
static PROGRAM_SO_PATH: &str = "geonft_solana.so";
static PROGRAM_KEYPAIR_PATH: &str = "geonft_solana-keypair.json";

pub fn get_program_keypair(client: &RpcClient) -> Result<Keypair> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let deploy_path = format!("{}/../../{}", manifest_dir, DEPLOY_PATH);
    let program_so_path = format!("{}/{}", deploy_path, PROGRAM_SO_PATH);
    let program_keypair_path = format!("{}/{}", deploy_path, PROGRAM_KEYPAIR_PATH);

    info!("loading program keypair from {}", program_keypair_path);

    let program_keypair = read_keypair_file(&program_keypair_path)
        .map_err(|e| anyhow!("{}", e))
        .context("unable to load program keypair")?;

    let program_id = program_keypair.pubkey();

    info!("program id: {}", program_id);

    let account = client.get_account(&program_id)
        .context("unable to get program account")?;

    if !account.executable {
        bail!("solana account not executable");
    }

    Ok(program_keypair)
}
```

The next step in the Helloworld program is to
"derive the address of a greeting account from the program so that it's easy to find later".
This seems like something I need to do as well.
Based on experience with other blockchains,
I suspect that just having the program uploaded to the blockchain isn't enough to use it,
but that it needs to be instantiated into one or more accounts before it can run,
and I think that is what this Helloworld code is doing.

I slowly work through the problem by reading the Helloworld TypeScript code,
search the `solana_sdk` and `solana_client` API docs for similarly-named methods.
I reach a point where I am calling [`get_minimum_balance_for_rent_exemption`],
but that method is failing with

[`get_minimum_balance_for_rent_exemption`]: https://docs.rs/solana-client/1.6.9/solana_client/rpc_client/struct.RpcClient.html#method.get_minimum_balance_for_rent_exemption

```
Error: RPC request error: Failed to deserialize RPC error response: {"code":-32600,"message":"Invalid request"} [missing field `data`]
```

I think maybe I am using this method incorrectly,
and want to look at some example Rust code.
The `feature-proposal` client doesn't use this method,
so I ripgrep the `solana-program-library` repo.

The [`token` CLI] uses it,
but not in a way that is really different from what I'm doing.
The big difference is that I am fudging the `data_len` parameter
by just passing in `1_000_000_000` and hoping it is large enough,
while the examples are calculating the data length exactly.
One billion is way larger than the data lengths used by the `token` program,
so maybe I just picked too big a number.

[`token` CLI]: https://github.com/solana-labs/solana-program-library/blob/master/token/cli/src/main.rs#L275

I try the same call with `data_len` as 100.

And that works. I was just specifying a data length that was too large.

Strange error,
but I think I understand:
there were two errors here.
The RPC client itself had an error,
where the server returned a response with a missing `data` field,
so deserializing the response failed;
and the server returned an "invalid request" error,
presumably because a data length of one billion bytes is bogus.

I know my program is going to require more than 100 bytes of data,
so I set it to 10_000 for now,
and verify that I am allowed to store at least that many bytes.


## Building a Solana instruction

Today my task is to build an instruction to execute the "plant" verb
from the client.
Something I think I am going to like about Solana is that
the on-chain program just recieves a blob of bytes,
called an "instruction",
and it's up to the program to interpret those bytes how it wants.
This gives me freedom to structure my program how I want &mdash;
I don't have to write a contract object that looks like Solidity.

One problem I anticipate is determining how much space our instructions use.
Solana programs and instructions seem to have to declare an exact amount of space
they occupy,
and determining that amount for an arbitrary serialized data structure
is not simple;
the length calculations in the SPL seem to be hardcoded,
so somebody has pre-calculated how much space their serilialized structures need,
which seems brittle.

For now I'll probably just say we need "a lot" of space.

I'm cribbing off the [`token` program][tpg] from the Solana Program Library.

[tpg]: https://github.com/solana-labs/solana-program-library/blob/master/token/program/src/instruction.rs


I discover the token program client is calling [`RpcClient::send_and_confirm_transaction_with_spinner`]
to execute its transaction,
and am intrigued by what "spinner" means in this context.

[`RpcClient::send_and_confirm_transaction_with_spinner`]: https://docs.rs/solana-client/1.6.9/solana_client/rpc_client/struct.RpcClient.html#method.send_and_confirm_transaction_with_spinner

After I write my instruction-building and transaction-executing `upload_plant`
method,
when I run it I see this error:

```
RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: incorrect program id for instruction [4 log messages]
```

What does "4 log messages" mean?
I ask in `#developer-support`:

> When I try to submit a transaction to my program, I see the error
>
> > RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: incorrect program id for instruction [4 log messages]
>
> How can I see those "4 log messages"? The don't show up in my 'solana logs' output.

Sometime later "dovacrow" responds with:

> call solana_logger::setup_with("solana=debug"); before your program starts

And "trent" with:

> Set the envvar `RUST_LOG=solana_client=debug`

I figure out that I passed the wrong pubkey to the `accounts` vector in my instructions.
I had

```rust
fn create_plant_instruction(plant_request: PlantRequestHash,
                            program_id: &Pubkey,
                            payer: &Pubkey,) -> Result<Instruction> {
    let data = GeonftRequest::PlantTreasure(plant_request).try_to_vec()?;
    Ok(Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*payer, true),
        ],
        data,
    })
}
```

passing the _payer_ account in the `accounts` vector.
Changing it to

```rust
fn create_plant_instruction(plant_request: PlantRequestHash,
                            program_id: &Pubkey,
                            program_instance: &Pubkey,) -> Result<Instruction> {
    let data = GeonftRequest::PlantTreasure(plant_request).try_to_vec()?;
    Ok(Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*program_instance, true),
        ],
        data,
    })
}
```

where `program_instance` is the account my program is running under,
makes progress,
though after making that change I still had another error:

```
[2021-05-28T15:33:35Z ERROR geonft_sync] not enough signers
```

It is interesting that this error displays as simply "not enough signers",
with no "RPC response error ..." preceding it.
I wonder why that is.
After some investigation,
I find out that the error comes from the `try_sign` method on `Transaction`,
not from an RPC call.

I try adding the only other keypair I have,
for the program ID,
to the list of signers:

```rust
    tx.try_sign(&[&config.keypair, program], blockhash)?;
```

And get another error:

```
[2021-05-28T15:44:26Z ERROR geonft_sync] keypair-pubkey mismatch
```

I am clearly just flailing at the code,
and don't understand what I am supposed to do when it comes
to accounts and signing.

At this point my `upload_plant` function looks like

```rust
pub fn upload_plant(plant_key: &str,
                    config: &Config,
                    client: &RpcClient,
                    program: &Keypair,
                    program_account: &Pubkey) -> Result<()> {
    let plant_request = io::get_plant(plant_key)?;
    let hash = crypto::get_hash(&plant_request.image)?;
    let plant_request = PlantRequestHash {
        account_public_key: plant_request.account_public_key,
        treasure_public_key: plant_request.treasure_public_key,
        treasure_hash: hash,
        account_signature: plant_request.account_signature,
        treasure_signature: plant_request.treasure_signature,
    };
    let inst = create_plant_instruction(plant_request,
                                        &program.pubkey(),
                                        program_account)?;
    let mut tx = Transaction::new_with_payer(
        &[inst], Some(&config.keypair.pubkey()));
    let blockhash = client.get_recent_blockhash()?.0;
    tx.try_sign(&[&config.keypair, program], blockhash)?;
    client.send_and_confirm_transaction_with_spinner(&tx)?;

    Ok(())
}
```

and it doesn't work.
I think I'll go read some solana docs about accounts
and transaction signing.

I figure out just by hunch that I was incorrectly
telling Solana to require a signature from my program account key,
which was derived from my program ID,
and for which I don't have the secret key to sign with.
I change my corresponding `AccountMeta` declaration:

```rust
     Ok(Instruction {
         program_id: *program_id,
         accounts: vec![
-            AccountMeta::new(*program_instance, true),
+            AccountMeta::new(*program_instance, false),
         ],
         data,
     })
```

and the transaction finally executes,
but with a new error:

```
[2021-05-28T16:19:52Z INFO  geonft_sync] executing step UploadPlantToSolana for gtp1kgx9nljqxjlm3vrz235xt669uy8da93d6lepshpkyjamh4etrets2xg5v3
[2021-05-28T16:19:52Z DEBUG solana_client::rpc_client] -32002 Transaction simulation failed: Error processing Instruction 0: Program failed to complete
[2021-05-28T16:19:52Z DEBUG solana_client::rpc_client]   1: Program EeyTCHgpCgn9NDZeiwtnbzXbN5ojw4wptorvF5uLXM5d invoke [1]
[2021-05-28T16:19:52Z DEBUG solana_client::rpc_client]   2: Program log: Geonft_solana entrypoint.
[2021-05-28T16:19:52Z DEBUG solana_client::rpc_client]   3: Program log: plant info: PlantRequestHash { account_public_key: "gap1g4gdvt6dnjrjwz6aeutcku4c39hc
qyd3vw5hxg85cspq20vzazhq5ccfyk", treasure_public_key: "gtp1kgx9nljqxjlm3vrz235xt669uy8da93d6lepshpkyjamh4etrets2xg5v3", treasure_hash: "9d3ca96c3e22ec303b988
cf64df62488c4488611fa83f04e9a898bcd374a193b", account_signature: "p/UeDqqJv5g2zaJKuZepx2EKdqKSphMUxxZ8oUiDWTvlZ22ZIzTw2i70EayaIUfJoiSWLVGq7sd8dxRNV8eRBg==",
treasure_signature: "tmpFTeXQ13E2ZcL8+PyHZeelAdbxqbcf43jG3vVe44sKsatWtbuuR0oF5PxjDNdokL9nSsW9AR/xkaEkyaDCBw==" }
[2021-05-28T16:19:52Z DEBUG solana_client::rpc_client]   4: Program EeyTCHgpCgn9NDZeiwtnbzXbN5ojw4wptorvF5uLXM5d consumed 124283 of 200000 compute units
[2021-05-28T16:19:52Z DEBUG solana_client::rpc_client]   5: Program failed to complete: Access violation in stack frame 13 at address 0x20000de70 of size 8 b
y instruction #29814
[2021-05-28T16:19:52Z DEBUG solana_client::rpc_client]   6: Program EeyTCHgpCgn9NDZeiwtnbzXbN5ojw4wptorvF5uLXM5d failed: Program failed to complete
[2021-05-28T16:19:52Z DEBUG solana_client::rpc_client]
[2021-05-28T16:19:52Z ERROR geonft_sync] RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: Program failed to complete
 [6 log messages]
```

This definitely looks like our old compiler warning
about our solana program having too-large stack offsets
finally causing us a problem.

Now we need to figure out what to do about our large stack frames,
which were introduced when we added the `ed25519-dalek` dependency
to our Solana program.

I ask again in `#developer-support`

> After adding ed25519-dalek to my solana program, I get non-fatal build errors like
>
> > Error: Function ZN209$LT$curve25519_dalek..window.. Stack offset of -15848 exceeded max offset of -4096 by 11752 bytes, please minimize large stack variables
>
> And these seem to manifest as access violations when running the program. How can I deal with this? Can a tell the compiler to give me bigger maximum stack frames?

And I go on:

> Seems 4k frames are a hard limit.
>
> I'm not tied to ed25519. Is there another assymetric crypto crate that definitely doesn't create huge stack frames?
>
> I guess the sdk's keypair is ed25519 so I could use that
>
> But it is using ed25519-dalek internally! How does it do that?

I discover then that the `solana_sdk` crate,
while it can be included in on-chain programs,
needs a bunch of features turned off for that purpose,
including `ed25519-dalek`.

I get the sense we are not on the Solana happy-path,
wanted to do our own signature verification on-chain,
but our application architecture of first verifying
signatures of our own transaction bundles off-chain,
then passively syncing them to the chain,
kind of demands it.

I realize that the problem here is probably this architectural
decision,
but we are committed to it for now,
and I think it can still work.

We just have to choose a crypto crate that
doesn't create huge stack frames.

In the meantime I file an [issue against `curve25519-dalek`][dalek-issue] asking
about putting their lookup tables in boxes.

[dalek-issue]: https://github.com/dalek-cryptography/curve25519-dalek/issues/355

Then I switch from ed25519 to ECDSA via the `k256` crate.
It only takes a couple of hours.

And now I no longer have the access violation errors.
I have new errors:

```
[2021-05-28T21:39:49Z INFO  geonft_sync] executing step UploadPlantToSolana for gtp1q05z4wcc9l0rah0ce2jpxhy2eyj2mrwljv7pmg2yced6fhgj9krtsxu8xa7
[2021-05-28T21:39:49Z DEBUG solana_client::rpc_client] -32002 Transaction simulation failed: Error processing Instruction 0: Program failed to complete
[2021-05-28T21:39:49Z DEBUG solana_client::rpc_client]   1: Program SMTdcH2EM33tSkbjW1oNxKRwfCCSaVoqTraoVYJAYsZ invoke [1]
[2021-05-28T21:39:49Z DEBUG solana_client::rpc_client]   2: Program log: Geonft_solana entrypoint.
[2021-05-28T21:39:49Z DEBUG solana_client::rpc_client]   3: Program log: plant info: PlantRequestHash { account_public_key: "gap1qt2ylqnejfe9znuvgn24lw240pcf
g50jeg7l5cpmd6rdms6fdmqa6enham7", treasure_public_key: "gtp1q05z4wcc9l0rah0ce2jpxhy2eyj2mrwljv7pmg2yced6fhgj9krtsxu8xa7", treasure_hash: "0d6f3d0ad56a849d017
8b6d2dcad56e4b2dcf95663349044d650d3cafd912c8e", account_signature: "rP/Vk5Q+/ofX+EpDSuTW5/nNWYvGL9u0YITIayA6/b1aD9ZRTW1LXvP55Zl1q6NDk7+5Im43bVLLVcwfZjgN2g=="
, treasure_signature: "H90jsobBourHNdIBLKlQj5zM5T9JHxHPMNG7aZ+D/v4PO7uoMS4HrvDhG1qTZWbsUXl8e+2VykJPntv1HSTUPQ==" }
[2021-05-28T21:39:49Z DEBUG solana_client::rpc_client]   4: Program SMTdcH2EM33tSkbjW1oNxKRwfCCSaVoqTraoVYJAYsZ consumed 200000 of 200000 compute units
[2021-05-28T21:39:49Z DEBUG solana_client::rpc_client]   5: Program failed to complete: exceeded maximum number of instructions allowed (200000) at instructi
on #29900
[2021-05-28T21:39:49Z DEBUG solana_client::rpc_client]   6: Program SMTdcH2EM33tSkbjW1oNxKRwfCCSaVoqTraoVYJAYsZ failed: Program failed to complete
[2021-05-28T21:39:49Z DEBUG solana_client::rpc_client]
[2021-05-28T21:39:49Z ERROR geonft_sync] RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: Program failed to complete
 [6 log messages]
```

It seems our program is just running too much code now.
That's a problem for tomorrow.


## Getting beneath the instruction limit

It's tomorrow.

I ask in `#developer-support`:

> My program fails to run with "Program failed to complete: exceeded maximum
  number of instructions allowed (200000) at instruction #29900". Is it possible
  to raise the instruction limit for a program?

(I think I recall that somebody answered this question, and the cap is fixed).

For the sake of getting this proof-of-concept done
it would be easiest to just raise the limit,
but there are definitely things we are doing in the program
that we don't need to do,
like deserializing keys and signatures from strings.

I also know our data model is not following the Solana way &mdash;
we are storing account records in hash tables when what we really should
do is derive new Solana account keys from one master key,
and create entirely new Solana accounts to hold their data.

We don't have time to understand how to derive new accounts
and store new data in time to make a demo for the hackathon
so I hope we don't have to change that.

After another day of hacking it is clear that two of our
assumptions about what we could do on chain are wrong:

1) We can't just store in our contract a large map
   of account keys to account information.
2) We can't do our own signature verification on-chain.
   The instruction budget is just too low.

Instead of storing a map of account data,
I think we are expected to derive Solana account keys from
a base key, and create new Solana accounts to store their own
finitely-sized records.

Instead of doing our own signing on chain,
I think we are expected to leverage Solana's runtime
by doing any necessary signing with Solana accounts,
passing those into transations with their instructions,
as `AccountMeta` values,
and have the runtime verify the accounts have signed
the instruction.

Both these things are forcing us into some tough,
but doable, rework.

Our program basically does five things:

- Deserialize the instruction
- Verify two signatures
- Deserialize the program state
- Add a record to a `HashMap` associated with an application (not Solana) account
- Serialize the program state

We find that:

- Just deserializing our instruction puts us over the CPU budget
- Verifying one signature puts us over the CPU budget
- Serializing our program state triggers an access violation

On the positive side,
if we comment out our entire program,
the client _can_ successfully execute the transactions
it needs to.

We think we know how to get around the dual problems
of spending too much CPU deserializing our instructions,
and too much CPU verifying signatures:

We are passing two signatures in with our instructions,
so we need to refactor our signing so that it is using Solana's
built in accounts and signatures,
let the runtime verify those signatures,
not pass them in to the program.

Moving the signature validation out of the program
eventually gets us under the instruction limit,
but it takes a while to work through some remaining problems.


## A few more obstacles

We are stumped about the remaining access violation though.
It looks like this:

```
[2021-05-30T17:25:02Z INFO  geonft_sync] executing step UploadPlantToSolana for gtp1q282wzch5t6zltr2f9t8vp3uetdcyl5yucnfq08rmwvwavaszkcuq2u9xwq
[2021-05-30T17:25:02Z DEBUG solana_client::rpc_client] -32002 Transaction simulation failed: Error processing Instruction 0: Program failed to complete
[2021-05-30T17:25:02Z DEBUG solana_client::rpc_client]   1: Program HAmCuzqtJws96qkGctkeHrZcu21PFKNzKGKbHy2wWMxa invoke [1]
[2021-05-30T17:25:02Z DEBUG solana_client::rpc_client]   2: Program log: Geonft_solana entrypoint.
[2021-05-30T17:25:02Z DEBUG solana_client::rpc_client]   4: Program log: plant_treasure
[2021-05-30T17:25:02Z DEBUG solana_client::rpc_client]   5: Program HAmCuzqtJws96qkGctkeHrZcu21PFKNzKGKbHy2wWMxa consumed 38554 of 200000 compute units
[2021-05-30T17:25:02Z DEBUG solana_client::rpc_client]   6: Program failed to complete: Access violation in program section at address 0x100026a00 of size 8 by instruction #12466
[2021-05-30T17:25:02Z DEBUG solana_client::rpc_client]   7: Program HAmCuzqtJws96qkGctkeHrZcu21PFKNzKGKbHy2wWMxa failed: Program failed to complete
[2021-05-30T17:25:02Z DEBUG solana_client::rpc_client] 
[2021-05-30T17:25:02Z ERROR geonft_sync] RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: Program failed to complete [7 log messages]
[2021-05-30T17:25:02Z INFO  geonft_sync] executing step UploadClaimToSolana for gtp1q282wzch5t6zltr2f9t8vp3uetdcyl5yucnfq08rmwvwavaszkcuq2u9xwq
```

I ask in `#developer-support`:

> Are there any smart techniques for debugging an access violation in a solana
  program? Right now I'm just commenting out code and msg! debugging to try to
  identify what code is causing it.

I don't think anybody answers this question,
but a lot of messages scrolled by,
and I find it difficult to identify messages mentioning
or replying to me in Discord.

After minimizing our program code to pinpoint the access violation,
we have a revelation.
Here's what we have:

```rust
pub fn plant_treasure(
    account: &AccountInfo,
    plant_info: PlantRequestSolana,
) -> Result<(), GeonftError> {
    msg!("plant_treasure");
    let mut treasure_data = Treasure::try_from_slice(&account.data.borrow())?;
    
    treasure_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}
```

Here, `Treasure` is our program state,
and the access violation happens when calling `Treasure::try_from_slice`.
I realize that at no time have we executed any code
to initalize our `account.data` to a valid `Treasure`.
So I would expect this call to fail,
but _not_ to trigger an access violation.

What's happening?
Is borsch running off the rails and doing an invalid memory access?

We verify that our `account.data` buffer is filled with zeros initially.

After some experimenting we realize that `HashMap` simply doesn't work
within a Solana program,
and this manifests as an access violation.
I would guess that `HashMap` operations panic,
and we didn't get to see the panic message,
but I think we've seen panic messages before.
I am unclear on what actually happened with `HashMap`.
Maybe it is doing a runtime abort.

We switch to a `BTreeMap` and things start proceeding more smoothly.

We still need to check if our program state is initialized.
Instead of understanding the proper pattern for this
by reading the SPL examples,
I do something obvious to me:

I reserve byte 1 of the program data for an "initialized" flag,
and serialize to the remaining slice of the program data.

Finally, we run into an error deserializing our program state
with the borsch [`try_from_slice`] method.

When we created our program state on chain,
we overallocated the amount of space we would need,
and the amount of state we need for the program currently grows dynamically.

`try_from_slice` though expects to be given a slice with exactly
the number of bytes to deserialize,
and returns an `InvalidData` / "not all bytes read" error
if given an overlarge buffer.

In our case it manifests with these logs in the client:

```
[2021-06-01T03:12:20Z INFO  geonft_sync] executing step UploadPlantToSolana for gtp1q0av2y3acvsmeq6nepdw9hnts5sptmpp97rt4h5xltajxx9d7kqnxkecltt
[2021-06-01T03:12:20Z DEBUG solana_client::rpc_client] -32002 Transaction simulation failed: Error processing Instruction 0: Program failed to complete
...
[2021-06-01T03:12:20Z DEBUG solana_client::rpc_client]   8: Program log: panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: "Not all bytes read" }', src/geonft_solana/src/lib.rs:56:68
...
```

The solution here,
which we found by reading code in the SPL,
is to use the [`try_from_slice_unchecked`] function,
which is not in borsch itself,
but in the Solana SDK.

[`try_from_slice_unchecked`]: https://docs.rs/solana-sdk/1.6.9/solana_sdk/borsh/fn.try_from_slice_unchecked.html
[`try_from_slice`]: https://docs.rs/borsh/0.8.2/borsh/de/trait.BorshDeserialize.html#method.try_from_slice

When we found this function we also
discovered that we were using version 0.9 of borsch
while Solana was using 0.8.
This results in uncharacteristically bed error messages
from the Rust compiler that are hard for newbies to understand and debug,
just complaining about mismatched types.


## Wrapping it up

At this point we were a few days from the end of the hackathon.
We didn't achieve everything we set out to,
and had more problems to solve,
but we were at a good stopping point,
with a vaguely-working client successfully executing
the yet-incomplete on-chain program.

So we spent the last few days cleaning up the UI,
deploying it to [treasuretree.org],
recording [a demo],
and submitting it to the hackathon judges.

[treasuretree.org]: https://treasuretree.org
[a demo]: https://youtu.be/uc2MUdDo4xs
