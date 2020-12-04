---
layout: post
title: First impressions of Rust smart contracts with Substrate and Ink, part 2
tags: [rust, blockchain]
---

This is part 2 of my 3 part series on getting started with [Substrate] and [Ink].

- [Part 1: Wandering around the Polkadot docs][pt1]
- [Part 2 (this part): Building a blockchain with Substrate][pt2]
- [Part 3: Creating a contract with Ink and running it on the Canvas network][pt3]

[pt1]: substrate-and-ink-part-1
[pt2]: substrate-and-ink-part-2
[pt3]: substrate-and-ink-part-3

[Ink]: https://github.com/paritytech/ink
[Substrate]: https://www.parity.io/substrate/

In the previous post I just explored the documentation,
and never actually got around to writing any smart contract code.
But I ended by coming up with a game plan,
which I intended to follow now:

- create a substrate devnet with the contracts pallet
- write an ink contract
- test that ink contract on our local devnet

Spoiler: we're not actually going to end up writing a contract in this part either,
but we're going to learn useful info about substrate development.
We'll definitely get to Ink in [part 3][pt3].


## In this post

- [Creating a blockchain with substrate](#creating-a-blockchain-with-substrate)
- [Following the "create your first substrate chain" tutorial](#following-the-create-your-first-substrate-chain-tutorial)
- [Interlude: What's in the substrate node template?](#interlude-whats-in-the-substrate-node-template)
- [The build fails](#the-build-fails)
- [The front-end](#the-front-end)
- [Running our new node](#running-our-new-node)


## Creating a blockchain with substrate

Now I'm going to the [Substrate Developer Hub][sdh] and following those docs.

[sdh]: https://substrate.dev/docs/en/

> "Welcome to the wonderful world of blockchain development with Substrate!"

Oooh.

Oh neat:

> "In Substrate, runtime code is compiled to Wasm and becomes part of the
  blockchain's storage state - this enables one of the defining features of a
  Substrate-based blockchain: forkless runtime upgrades."

So you customize a substrate blockchain my writing its logic in WASM,
which is itself stored on-chain.
That's fun.

There are several ways to build a substrate chain,
but the easiest is to use the "Substrate Node" right out of the box,
with a little bit of custom configuration.
For our purposes,
this seems right.

Oh, there's so much documentation here.
It seems impossible to follow it in any linear way.
That's ok - at least there are docs.

For running a Substrate Node,
the docs indicate to use the two tutorials:

- [Create Your First Substrate Chain](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/)
- [Start a Private Network](https://substrate.dev/docs/en/tutorials/start-a-private-network/)

I figure we'll do the first, then the second if we need to.

I've only just started reading the top-level docs on [substrate.dev] though.
Looking at the sidebar I see lots of interesting topics I want to know more about,
but for now we'll go straight to the tutorial.

[substrate.dev]: https://substrate.dev


## Following the "create your first substrate chain" tutorial

Ok, now we're following _these_ docs:

> [https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/)

Hopefully we'll be hacking soon.

There's a one-liner for setting up development tools:

```
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

but reading about what it does,
I should already have everything it installs on my box.
I skip that step...

The next step is to clone a "node template":

```
git clone -b v2.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template
```

I see it is cloning "v2.0.0" and wonder if that is the latest.
Is it?
Yes, there are no newer tags.
The docs are up to date.

Per the docs I update my stable and nightly Rust toolchains,
and add the `wasm32-unknown-unknown` target to the nightly.

```
rustup update nightly
rustup update stable
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Now I can build the template with `cargo build --release`.


## Interlude: What's in the substrate node template?

While we are building...

The [readme for `substrate-node-template`][rsnt] is informative.

[rsnt]: https://github.com/substrate-developer-hub/substrate-node-template

From a very naive guess,
it looks like a simple substrate node is divided into two parts:

- [The "node"][sn] - This is the instance of substrate that is compiled to machine
  code, and should be substantially the same for any instance of substrate.
- [The "runtime"][sr] - This is the thing that makes your substrate _your substrate_.
  It is typically compiled to wasm and is run by the substrate node. It defines
  the crucial state transition function that characterizes your blockchain.

There's also a ["pallets template"][spt] that defines some mysterious macro magic,
and is imported by the runtime. Best to ignore that for now I think.

[sn]: https://github.com/substrate-developer-hub/substrate-node-template/tree/master/node
[sr]: https://github.com/substrate-developer-hub/substrate-node-template/tree/master/runtime
[spt]: https://github.com/substrate-developer-hub/substrate-node-template/blob/master/pallets/template/src/lib.rs

None of this code looks simple - it's all a bit panic-inducing,
really,
but I'm sure with enough time I could more-or-less understand it.

Building a custom substrate runtime is complex enough that [it requires a macro][rtm].

A big macro. Look at this syntax:

```rust
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
		Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
		Aura: pallet_aura::{Module, Config<T>, Inherent},
		Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Module, Storage},
		Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
		// Include the custom logic from the template pallet in the runtime.
		TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
	}
);
```

I kind of can't even guess what this might expand to,
even though the syntax begins with "pub enum Runtime ...".
But even though that's alarming,
it's also what's so great about macros:
you can wrap up some very complex constructions into something
that is at least simple on the surface.

Much of the rest of the runtime file is dedicated to defining
assocated types on `Runtime` for specific pallets, like

```rust
impl pallet_transaction_payment::Trait for Runtime {
	type Currency = Balances;
	type OnTransactionPayment = ();
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ();
}
```

[rtm]: https://github.com/substrate-developer-hub/substrate-node-template/blob/master/runtime/src/lib.rs#L270.

I'm sure there is tons of inherent complexity in building even a simple blockchain,
and substrate is not a simple blockchain,
it's a complex toolkit for building complex blockchains.


## The build fails

I run into a build error:

```
  error[E0282]: type annotations needed
      --> /home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/sp-arithmetic-2.0.0/src/fixed_point.rs:541:9
       |
  541  |                   let accuracy = P::ACCURACY.saturated_into();
       |                       ^^^^^^^^ consider giving `accuracy` a type
```

Because this is a Rust typechecking error,
I'm guessing this is a problem related to nightlies:
the tutorial has told me to use just "nightly",
and probably I need a specific nightly.

I see that the readme for `substrate-node-template` has a different `cargo build` invocation than the tutorial:

```
WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
```

This is telling the substrate build scripts (presumably) to use a specific nightly when building
the WASM parts of the runtime.

I install that nightly and its wasm target:

```
rustup toolchain install nightly-2020-10-05
rustup target add wasm32-unknown-unknown --toolchain=nightly-2020-10-05
```

and run the new build command:

```
$ WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
   Compiling librocksdb-sys v6.11.4
   Compiling futures-timer v3.0.2
   Compiling jsonrpc-ipc-server v15.0.0
   Compiling parity-util-mem v0.7.0
The following warnings were emitted during compilation:

warning: couldn't execute `llvm-config --prefix` (error: No such file or directory (os error 2))
warning: set the LLVM_CONFIG_PATH environment variable to the full path to a valid `llvm-config` executable (including the executable itself)

error: failed to run custom build command for `librocksdb-sys v6.11.4`
```

A different error.
This is surely because I didn't run the `getsubstrate.io` setup script,
thinking I had all the development prerequisites installed.
Fortunately I'm familiar with this error and know I need to install clang/llvm development packages.

I `sudo install libclang-dev` and try again.
It fails.
I `sudo install llvm-dev` and try again:

```
$ WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
   Compiling librocksdb-sys v6.11.4
   Compiling async-std v1.6.2
   Compiling sp-utils v2.0.0
   Compiling cranelift-wasm v0.66.0
error: failed to run custom build command for `librocksdb-sys v6.11.4`

Caused by:
  process didn't exit successfully: `/home/ubuntu/substrate-node-template/target/release/build/librocksdb-sys-871a9c160c12016c/build-script-build` (exit code: 101)
  --- stderr
  rocksdb/include/rocksdb/c.h:65:10: fatal error: 'stdarg.h' file not found
```

This one is because clang's stdlib is not installed.
I `sudo install libc++-dev` and try again.
Nope, not that one.
I stop guessing and look at the script at `https://getsubstrate.io`.
Oh, it's just `clang`.
I `sudo install clang` and try again.

Now my build succeeds.
Big brain right here.

While my build suceeds,
my partner's (who is also following along)
does not.
She continues to get the same "type annotations needed" error.
After investigation we discover that while I have
the "stable" toolchain set to the default,
she has the "nightly" toolchain as the default.

So on her computer we run

```
rustup default stable
```

Now her build succeeds.

There must be a type inference regression on nightlies that is causing the build to break.

I should report this to the Rust bug tracker...
but I'm tired of diversions and want to press on.
Sorry.

The build takes about 15 minutes,
in release mode,
on my fairly puny machine.
Thanks, Rust.

I have my own blockchain now, yay!
Thanks again, Rust.

These would have been difficult problems to overcome for someone not already familiar with nightly Rust development,
and I can tell the Substrate docs are trying to be friendly to non-Rust devs.
On the other hand,
someone new to Rust would probably not have tried to skip steps like I did,
so wouldn't have hit as many of these challenges.

Time to continue the "create your first substrate chain" tutorial...


## The front-end

Next, the tutorial wants us to install the front end template,
which is of course JavaScript.

I don't _think_ I need this right now,
since I'm interested in writing contracts,
but I'll need it eventually,
and I'm curious what the front end is like,
and I want to follow the tutorial as written
(especially as I skipped steps in the last section to my own detriment).

I'm always a bit apprehensive whenever I have to build / run a JavaScript project.
That world feels alien.

Let's feel the pain...

```
$ git clone -b v2.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-front-end-template
$ cd substrate-front-end-template
$ yarn install
yarn install v1.22.5
[1/5] Validating package.json...
[2/5] Resolving packages...
[3/5] Fetching packages...
info fsevents@2.1.2: The platform "linux" is incompatible with this module.
info "fsevents@2.1.2" is an optional dependency and failed compatibility check. Excluding it from installation.
info fsevents@1.2.13: The platform "linux" is incompatible with this module.
info "fsevents@1.2.13" is an optional dependency and failed compatibility check. Excluding it from installation.
info fsevents@2.1.3: The platform "linux" is incompatible with this module.
info "fsevents@2.1.3" is an optional dependency and failed compatibility check. Excluding it from installation.
[4/5] Linking dependencies...
warning "react-scripts > @typescript-eslint/eslint-plugin > tsutils@3.17.1" has unmet peer dependency "typescript@>=2.8.0 || >= 3.2.0-dev || >= 3.3.0-dev || >= 3.4.0-dev || >= 3.5.0-dev || >= 3.6.0-dev || >= 3.6.0-beta || >= 3.7.0-dev || >= 3.7.0-beta".
[5/5] Building fresh packages...
Done in 83.31s.
```

Ok that was painless.


## Running our new node

The next page in the tutorial, "Background Information"

> [https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/background](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/background)

is a bit out of place here,
just kind of trivia.

Following along on the next page, "Interacting with Your Node":

> [https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/interact](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/interact)

The instructions say to run

```
./target/release/node-template --dev --tmp
```

but I personally don't like to simply run binaries out of the `target` directory,
so I try:

```
$ WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run --release -- --dev --tmp
```

The output is exciting:

```
Nov 16 00:13:28.538  INFO 🏷  Node name: slim-turn-8420
Nov 16 00:13:28.538  INFO 👤 Role: AUTHORITY
Nov 16 00:13:28.538  INFO 💾 Database: RocksDb at /tmp/substrateQmdqLp/chains/dev/db
Nov 16 00:13:28.538  INFO ⛓  Native runtime: node-template-1 (node-template-1.tx1.au1)
Nov 16 00:13:28.634  INFO 🔨 Initializing Genesis block/state (state: 0xe656…ffd0, header-hash: 0x3844…fe3c)
Nov 16 00:13:28.635  INFO 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
Nov 16 00:13:28.657  INFO ⏱  Loaded block-time = 6000 milliseconds from genesis on first-launch
Nov 16 00:13:28.658  WARN Using default protocol ID "sup" because none is configured in the chain specs
Nov 16 00:13:28.658  INFO 🏷  Local node identity is: 12D3KooWBmAKFMGa6Dt9oi4Ct1erBgZgYkunYDupASh3L1EaKbpg (legacy representation: 12D3KooWBmAKFMGa6Dt9oi4Ct1erBgZgYkunYDupASh3L1EaKbpg)
Nov 16 00:13:28.669  INFO 📦 Highest known block at #0
Nov 16 00:13:28.669  INFO 〽️ Prometheus server started at 127.0.0.1:9615
Nov 16 00:13:28.670  INFO Listening for new connections on 127.0.0.1:9944.
Nov 16 00:13:30.006  INFO 🙌 Starting consensus session on top of parent 0x3844523584d4b8572c80478e0c69bb79dbdba6416283a1af756e2a8211b8fe3c
Nov 16 00:13:30.012  INFO 🎁 Prepared block for proposing at 1 [hash: 0x0dbc7b803cf6730d0e3b56c48e3d513fbb5298c9087b727bbf7a57d24da66f3d; parent_hash: 0x3844…fe3c; extrinsics (1): [0xa89d…df2b]]
Nov 16 00:13:30.015  INFO 🔖 Pre-sealed block for proposal at 1. Hash now 0x1601051fdd3e7670d2eacce27a59688eb7a1dad690dbce7bdc697810bb64c491, previously 0x0dbc7b803cf6730d0e3b56c48e3d513fbb5298c9087b727bbf7a57d24da66f3d.
Nov 16 00:13:30.016  INFO ✨ Imported #1 (0x1601…c491)
Nov 16 00:13:33.670  INFO 💤 Idle (0 peers), best: #1 (0x1601…c491), finalized #0 (0x3844…fe3c), ⬇ 0 ⬆ 0
```

Those emoji are so silly,
but I love seeing them.
I can imagine they make scanning the log easier once you are familiar with them,
so maybe not as silly as they look.

I am relieved that Substrate makes it easy to run a temporary devnet with a single command,
no generating keys,
creating config files,
creating directories.
In my (admittedly limited) experience NEAR also makes this easy,
Nervos does not,
and the proper development workflow for Ethereum is just not clear at all.

Every blockchain should have a simple command to spin up a local devnet.

I note the output line

```
Nov 16 00:13:28.669  INFO 〽️ Prometheus server started at 127.0.0.1:9615
```

I don't know much about Prometheus,
but do know it's for collecting metrics.
I hope I can open that port in a web browser and see something interesting.

Now I run the frontend with `yarn start`:

```
Compiled successfully!

You can now view substrate-front-end-template in the browser.

  Local:            http://localhost:8000/substrate-front-end-template
  On Your Network:  http://172.30.0.181:8000/substrate-front-end-template

Note that the development build is not optimized.
To create a production build, use yarn build.
```

Cool.

Now I should be able to open a web browser on port 8000 to see the substrate GUI,
and port 9615 to see the metrics.

I'm running remotely on EC2,
so I need to exit my shell and reconnect with the appropriate port forwarding
in order to tunnel those ports to my local computer:

```
ssh -A <my-server> -L localhost:8000:localhost:8000 -L localhost:9615:localhost:9615
```

I try to open `localhost:8000` on my local computer and see an error:

> Error Connecting to Substrate
> [object Event]

Awesome.
I'm guessing that I need to forward another port,
probably the RPC port for the substrate node.

Based on the Substrate logging output,
I establish a new ssh tunnel,
adding port 9944 as well:

```
ssh -A <my-server> -L localhost:8000:localhost:8000 -L localhost:9615:localhost:9615 -L localhost:9944:localhost:9944
```

Now I see the frontend.

It's lovely.


<img class="blog-photo" src="/images/substrate-ui.png"/>

There's so much fun-looking stuff to mess with here,
but for now I'm not going to.

Really,
this is a pretty impressive experience for a newbie -
lots of interesting things to wonder about and play with,
from the docs,
to the node logging,
to the default UI.

I attempt to navigate to `localhost:9516`,
the prometheus address,
but it just returns "not found",
so I guess that's not a prometheus UI,
just a service port of some kind.

Some googling reveals that I can get the server metrics from

> http://localhost:9615/metrics

but it seems if I want a nice metrics frontend I need to do some more work.

Later.

I note that,
compared to NEAR,
it has taken me quite a bit longer to get started actually writing a contract,
since I have first had to learn how to build my own blockchain.
I don't particularly mind this,
as building a substrate blockchain has been interesting,
and I am excited about the possibilities,
but it is a notable point of comparison.

(In the next part,
we'll see there was a path to getting started with
substrate smart contract programming that didn't involve
building a custom substrate node first,
I just didn't find it directly).

Let's build a Substrate smart contract.

Continued in [part 3].

[part 3]: substrate-and-ink-part-3
