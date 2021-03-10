---
layout: post
title: Further adventures with Substrate and Ink
tags: [rust, blockchain]
---

During my [last series of bloggings][subblog] about [Substrate],
I intended to explore [`ink!`],
Parity's DSL library for writing smart contracts in Rust.
Those posts though mostly ended up being about setting up
the appropriate development environment,
and my failures along the way.

I remain though interested in learning Substrate,
and in particular smart contract programming in Rust on Substrate.
So in the time since that last exploration,
[Aimee] and I have envisioned a small project
to hack on that will guide us to learn more about
Substrate/Ink development.

[Aimee]: https://github.com/aimeedeer

This report will be about our experiences beginning to implement that project.
It is broadly divided into three sections:
contract implementation with Ink,
client implementation with [polkadot.js],
and some concluding thoughts.

A lot of this post is tedious details
that will probably only be interesting to the product developers,
but I've tried to summarize some of the useful bits in
the ["learnings and tips"][lat] section near the end,
and the [final section][somethoughts] is a bit of a rant
about my experience learning smart contract programming
that is hopefully interesting or entertaining.

[somethoughts]: #some-thoughts

This project began on January 1, 2021,
and continued in small bursts through March 1, 2021.
Along the way we changed the exact revisions of the software
we were using several times.
Some observations may be slightly out of date,
and I try to note such changes in the text where I am aware of them.

[Substrate]: https://substrate.dev
[subblog]: https://brson.github.io/2020/12/03/substrate-and-ink-part-1
[`ink!`]: https://github.com/paritytech/ink
[polkadot.js]: https://github.com/polkadot-js
[lat]: #learnings-and-tips

- [Our project](#our-project)
- [Terminology](#terminology)
- [Implementing the contract](#implementing-the-contract)
  - [Debugging, etc.](#debugging-etc)
  - [Debugging cross-contract calls](#debugging-cross-contract-calls)
  <!-- - [But first, updating our tools](#but-first-updating-our-tools)-->
  <!-- - [And then debugging cross-contract calls](#and-then-debugging-cross-contract-calls)-->
  - [Debug-printing the environment doesn't work](#debug-printing-the-environment-doesnt-work)
  - [Wait what's this? Some weird new issue!](#wait-whats-this-some-weird-new-issue)
  - [Errors in ink, and a `CallBuilder` miracle](#errors-in-ink-and-a-callbuilder-miracle)
  - [Completing the level progression logic](#completing-the-level-progression-logic)
- [The many-step, error-prone, build-deploy-test cycle](#the-many-step-error-prone-build-deploy-test-cycle)
- [Testing ink contracts](#testing-ink-contracts)
- [Fixing a build failure in `cargo-contract`](#fixing-a-build-failure-in-cargo-contract)
- [Connecting to our contract with polkadot-js](#connecting-to-our-contract-with-polkadot-js)
  - [Next attempt to get a simple UI working](#next-attempt-to-get-a-simple-ui-working)
- [We fail to complete the prototype](#we-fail-to-complete-the-prototype)
- [Learnings and tips](#learnings-and-tips)
- [Some thoughts](#some-thoughts)
  - [First, some hopefulness](#first-some-hopefulness)
  - [Next, some venting](#next-some-venting)
  - [At least I'm learning](#at-least-im-learning)


## Our project

Our project is a game for teaching ink smart contract programming.
It runs on a substrate blockchain ([canvas] for now),
where players submit their own contracts in order to
complete individual programming challenges.
These challenges are judged by running them on the blockchain,
and comparing their behavior to the expected behavior,
similar to a unit test.

[canvas]: https://github.com/paritytech/canvas-node

We like this project because it accomplishes multiple goals for us:

- It lets _us_ learn smart contract programming
- It lets us _write_ about what we learn
- It creates a _product_ that others may find useful
- It may be worthy of a _grant_ to continue the work

Our goal for this initial sprint has been to create
a prototype that demonstrates the basic functionality
of submitting and running levels,
and progressing through levels;
both the contracts and the UI.

Also to write about how we struggled and what we learned.

The source for this project lives at

> [https://github.com/brson/contract-game](https://github.com/brson/contract-game)


## Terminology

The start of every project is filled with an awkward period
of figuring out what to call things.
Here are some of the terms used in our game,
and used in this blog post.

### Substrate terms

- _Account ID_ - A substrate account.
  These can represent things, but for our purposes so far
  they represent the accounts of people and the accounts of contracts.
  Sometimes referred to abstractly as just "account".
- _Method selector_ - Under the hook, ink methods are called
  via RPC, and methods don't really have names, but instead magic
  numbers. We'll need to use these explicitly to call other contracts
  dynamically.

### Game terms

- _Game contract_ - The Substrate / Ink contract that runs the game.
- _Player account_ - An account within the game belonging to a single player,
  associated with a Substrate account and the player's game data.
- _Player level contract_ - The contract submitted by a player
  to complete a single level in the game. This is sometimes
  abbreviated as just "level contract" in context.
- _Player level contract account ID_ - The Substrate account associated with a player level contract.
  Some of these words may be abbreviated out in context.

Just re-reading this I am struck at the immediate complexity
of disambiguating these concepts:
"player level contract account ID"?
Yeek.
Maybe I'm overthinking this.



## Implementing the contract

We use `cargo contract new` to create our new contract,
then immediately delete most of the contents,
leaving just the contract module and its decorating
`ink::contract` attribute:

```rust
use ink_lang as ink;

#[ink::contract]
mod game {
}
```

Most everything we write will go inside this module.

The first thing we think about when writing our contract is the
state represented in the account.
This state is called "storage" in substrate.
Or it lives in "storage".
It lives on the blockchain.

We begin by defining our storage requirements as simply
a map from the substrate account IDs owned by individual players
to the game-specific state for that player.

We write it

```rust
#[ink(storage)]
pub struct Game {
    player_accounts: HashMap<AccountId, PlayerAccount>,
}
```

The substrate `AccountId` represents multiple important pieces for us:
users have accounts, and contracts on the blockchains have accounts.

In ink, `AccountId`, along with some other types, is just magically in scope,
probably thanks to the `#[ink:contract]` macro.

And that `HashMap` is not the standard `HashMap`:
it is a special ink `HashMap` that stores items on the blockchain.
It is imported like:

```rust
#[cfg(not(feature = "ink-as-dependency"))]
use ink_storage::collections::HashMap;
```

We don't yet understand what the `ink-as-dependency` feature means;
we are just copying this attribute from examples.

`PlayerAccount` is our own type that holds our game-specific
player account state.
We try to put an `ink_storage::HashMap` into the `PlayerAccount` struct,
inside another `ink_storage::HashMap`, inside our `Game` storage type.

Something like

```rust
#[ink(storage)]
pub struct Game {
    player_accounts: ink_storage::HashMap<AccountId, PlayerAccount>,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode,
         ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
pub struct PlayerAccount {
    level: u32,
    level_programs: ink_storage::HashMap<u32, AccountId>,
}
```

It doesn't work, with a bunch of errors about traits like `WrapperTypeEncode`
not being implemented for types like `ink_storage::HashMap`.

We look at the ink examples and don't see any using nested collection
types in their storage types.
Instead they all use a "flat" data model.
I don't really want to do that because it will be harder to maintain
the invariants I want.
Reading the API docs for `scale::Encode` we see that the standard
`BTreeMap` type implements it,
so instead of nesting `ink_storage::HashMap`s inside each other,
we use a `BTreeMap` instead, like

```rust
#[ink(storage)]
pub struct Game {
    player_accounts: ink_storage::HashMap<AccountId, PlayerAccount>,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode,
         ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
pub struct PlayerAccount {
    level: u32,
    level_programs: BTreeMap<u32, AccountId>,
}
```

I imagine this will be inefficient,
but for now we want our code to be readable,
not efficient.

We can easily imagine the purpose of the `scale::Encode` and `scale::Decode`
derives,
but don't yet understand the `PackedLayout`
and `SpreadLayout` derives.

For each level of our game,
our game contract needs to call a player's "level contract".
So each of our levels defines a contract interface,
and each player implements that interface in their own
contract for the level.

That's what we store in the `level_programs` map:
a mapping from a level number to the account ID of the contract,
owned by the player,
that implements a solution to that level.

In the UI,
we imagine the initial game flow will begin
with tasks like connecting the wallet,
checking whether the player has an account,
creating player accounts,
and registering level contracts.

We write a few initial methods to accomplish
these tasks without many problems,
and test them in [canvas-ui].

[canvas-ui]: https://github.com/paritytech/canvas-ui

Here's `create_player_account`:

```rust
#[ink(message)]
pub fn create_player_account(&mut self) -> Result<PlayerAccount, Error> {
    let caller = self.env().caller();

    if self.have_player_account(caller) {
        Err(Error::AccountExists)
    } else {
        self.create_player(caller)
    }
}
```

`have_player_account` and `get_player_account` are similarly obvious.

After a bit of success,
we decide that the next problem for us to solve
is to call our players' level contracts.

These are contracts that conform to an interface defined by our game,
but have implementations defined by our players,
so we know we're going to have to call these contracts dynamically
by their interface.

When we start trying to figure this out,
we run into a lack of examples and documentation.




### Debugging, etc.

While incrementally adding features,
experimenting with ink APIs,
and attempting to debug,
we find that we don't know how to do "println debugging":
ink defines [`ink_env::debug_println`],
but when we use it we don't see any output anywhere.

[`ink_env::debug_println`]: https://paritytech.github.io/ink/ink_env/fn.debug_println.html

I ask in the `#ink:matrix.parity.io` channel where to see the output,
and Alexander Theißen replies:

> "They are printed to the console. You need to raise the log level for `runtime`
  module in order to see them. I also recommend to silence all the block
  production noise: `-lerror,runtime=debug`"

So those are presumably flags to `canvas-node`.

Now I am running my canvas node, from source, with the command

```
cargo run -- --dev --tmp -lerror,runtime=debug
```

We have Alice construct our game contract,
and want to test having that contract
call another contract (a "level contract").
For testing purposes that level contract is the
"flipper" example contract.
We upload that contract and have Bob construct it.

We are confused about:

- Are contracts identified by users' account IDs,
  or do they have their own account IDs?
- How can we find the account ID of a contract we've constructed?

We're confused every step of the way.

While trying to figure out Bob's account ID we make two discoveries:

1) The `subkey inspect` command:

    ```
    subkey inspect //Bob
    Secret Key URI `//Bob` is account:
      Secret seed:      0x398f0c28f98885e046333d4a41c19cee4c37368a9832c6502f6cfd182e2aef89
      Public key (hex): 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
      Account ID:       0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
      SS58 Address:     5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
    ```

    I don't fully understand how to specify account names, but I know
    that `//Alice` and `//Bob` are known test accounts. This `subkey inspect`
    command gives the account id and secret key.

    The `subkey` command is part of the substrate repo and [the command
    for installing it is a bit awkward][subkeycommand], but easy enough
    to copy-paste.

2) The [polkadot explorer][pex] can be configured to connect to my
   local devnet!

   This seems pretty huge. I can see account ID's here, and hopefully
   even more useful info about what is going on inside my devnet.

[subkeycommand]: https://substrate.dev/docs/en/knowledgebase/integrate/subkey#build-from-source
[pex]: https://polkadot.js.org/apps/#/explorer



Both canvas-ui and the polkadot.js explorer cache information about
code hashes (and probably other things) they've seen, but
which don't actually exist on the chain they are currently connected to.
This is maddening every time I restart my devnet and see a bunch of
hashes of old contracts that I can't distinguish from the new contracts.

I wish these apps would either verify these code hashes actually exist on
chain, or at least give me a big red "reset" button to purge all
this incorrect info.

At some point I had canvas-ui showing me two "flipper" contracts,
and I didn't know which one was real,
so I told it to "forget" both of them.
Then I tried to redeploy flipper,
but of course flipper was already deployed so I got an error,
and now I don't know the correct address of flipper,
and can't add it back to the UI and I'm stuck,
have to shut down my devnet and restart it.

I now have a habit of going through all the canvas-ui tabs
and "forgetting" everything every time I restart canvas-node.

At least the polkadot explorer says "not on-chain" for code
hashes that don't actually exist.

After some experimenting I learn that
constructing a contract creates a new account;
that is, a contract is not associated with the account
of the user that creates it; a contract has its own account.

We execute transactions in the canvas-ui, but no
events seem to register in the explorer ui.

Sometimes canvas-ui doesn't know how to interpret the results
of a method call and gives no feedback.

We're going to have to add logging _everywhere_
to have any clue what the hell is going on.



### Debugging cross-contract calls

We ran into our first big blocker:
we can't figure out how to call another contract via interface.

The ink repo contains examples of defining traits,
via `#[trait_definition]`
and calling contracts through them,
but those examples all still have direct access to the _implementations_
of those traits.
That is too limiting for our needs.

By browsing the API docs we came across [CallBuilder],
which appears to call an arbitrary method in another contract,
given that contract's account id, a known _method selector_,
which is a unique ID used to identify a method,
and the configuration for the args, return value,
gas, and other things.

[CallBuilder]: https://paritytech.github.io/ink/ink_env/call/struct.CallBuilder.html

We tried to use this and failed.
And failed.

And failed.

For days.

We took a long break.

We were discouraged,
and it was hard to come back to this problem,
but we can't make any progress without solving it.

So that's today's mission.


<!--
### But first, updating our tools

Since the canvas tools are immature,
and it has been over a week since I last used them,
I first pull and rebuild `canvas-node`,
`cargo-contract`, and `canvas-ui`.

I recall that last time I built `canvas-ui` it
would not successfully connect to my `canvas-node` instance,
and I hope things have changed.

After running `git pull` on `canvas-node`,
I see no new commits, which is surprising &mdash;
no changes since December 2!

Development must not happen on the master branch.
Wait, there's no obvious development branch either.
It seems that canvas-node really hasn't changed in over a month.

With consideration,
this is pretty reasonable,
since canvas-node is just an instantiation of the gigantic
substrate toolkit &mdash;
it is only 1200 lines of code &mdash;
so most development goes directly into substrate.

This though makes me wonder if I can update canvas's substrate revision
(also if I should).
It is something I am curious to attempt,
but doesn't seem prudent right now,
since presumably most everybody else using canvas-node is using the substrate revisions in Cargo.lock.

*Note that since this was written, `canvas-node` has received an update to a more recent version of substrate*.

`cargo-canvas` has updates and I install them from the source directory with

```
git pull
cargo install --path .
```

I notice that while I usually run `canvas-node` directly from the source directory via `cargo`,
I have been installing `cargo-canvas`.
I think this is because the two have different usage models:
`canvas-node` is a server and I mostly just leave it running in a tmux window;
`cargo-canvas` though is a tool I might need to run in any of multiple windows
for different purposes.

`canvas-ui` also has updates and I build them with ... (I have to look this up in `package.json`):

```
git pull
yarn install
```

and then I try the `yarn build` command listed in `package.json`,
thinking that it is probably one of the prerequisites to `yarn start`,
but it doesn't seem so,
as it doesn't work, printing

```
command not found: node_modules/@polkadot/dev/scripts/polkadot-dev-build-ts.js
```

and exiting with an error code.
That's fine: I know how to run `canvas-ui` with `yarn start`.
It takes a long time to start up though,
and I was hoping I could do some of that webpacking or whatever ahead of time.

While I'm tinkering with building my tooling,
I wonder if I can install an updated [`subkey`][subkeycommand] tool from source.

I `cd` into my substrate source directory and run

```
git pull
cargo install --path bin/utils/subkey/
```

It works. I have updated tools.
Enough procrastinating; time to solve problems.
-->



<!--### And then debugging cross-contract calls-->





What happens when we try to call a method via `CallBuilder` is
that `canvas-node` logs (when `-lruntime=debug` is passed on the command line):

```
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:DispatchError
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:8
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:17
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:ContractTrapped
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:PostInfo:
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:actual_weight=
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:7172485790
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:pays_fee=
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:Yes
```

As the only clue to what's happening, it's some pretty horrible feedback.
By ripgrepping the substrate code [for `DispatchError`][fde] we gather that

- "8" is a "module index", here the contracts module, though it isn't explicitly stated
- "17" is a module-specific error number, here it corresponds to [`ContractTrapped`].
  This _is_ indicated in the output, though the connection between "17" and `ContractTrapped`
  is not explicit.
- "actual_weight" is part of [`PostDispatchInfo`] and indicates the
  "actual weight consumed by a call or `None` which stands for the worst case static weight".

[fde]: https://github.com/paritytech/substrate/blob/7a79f54a5d92cecba1d9c1e4da71df1e8a6ed91b/primitives/runtime/src/lib.rs#L404
[`ContractTrapped`]: https://github.com/paritytech/substrate/blob/7a79f54a5d92cecba1d9c1e4da71df1e8a6ed91b/frame/contracts/src/lib.rs#L399
[`PostDispatchInfo`]: https://github.com/paritytech/substrate/blob/7a79f54a5d92cecba1d9c1e4da71df1e8a6ed91b/frame/support/src/weights.rs#L329

With our "actual weight" looking like a pretty large number,
our best guess right now is that we just didn't spend enough gas
to make the call.

For our experiment today
we create a repo just for testing `CallBuilder`:

> https://github.com/Aimeedeer/game-test

Our `CallBuilder` right now looks like:

```rust
let return_value: bool = build_call::<DefaultEnvironment>()
    .callee(program_id) 
    .gas_limit(50)
    .transferred_value(10)
    .exec_input(
        ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF]))
    )
    .returns::<ReturnType<bool>>()
    .fire()
    .unwrap();
```

The `program_id` is the `AccountId` of the flipper contract,
which we are using as a test game level contract,
provided as an argument to the caller.
The selector is one that we've given to the flipper's `get`
method with the `selector` attribute.

When we run our contract we see 

```
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:DispatchError
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:8
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:6
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:OutOfGas
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:PostInfo:
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:actual_weight=
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:50000000
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:pays_fee=
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:Yes
```

Hey!

This is a different error: now we're `OutOfGas`, not `ContractTrapped`.

And `actual_weight` looks suspicious:
it is 50 million;
and our call builder set the gas limit to "50".
Is `gas_limit` specified in millions?

After some experimentation the answer seems to be "no".
and these two numbers are not directly connected,
and we were putting some number beginning with "5" as
the gas limit in canvas-ui.

We have become completely baffled by how units are specified
in various places in the code and UI.
Parts of the code say the units are "Unit",
like when specifying the amount paid to a contract;
parts say "M",
like when specifying the gas paid to run a contract,
which we assume is a million units;
but units don't seem to be the actual underlying integer number of units,
since units are divisible to some much smaller number.

We think we are not providing enough gas to the call builder
and increase that number;
we trying increasing the gas we provide to the callee
when invoking it in the canvas-ui.
No matter what we do it's `OutOfGas` or `ContractTrapped`.

We think maybe our calling account doesn't have enough
"units" to execute the contract,
so we transfer all of the money in the ALICE, ALICE_STASH,
and BOB_STASH accounts to BOB, and BOB still can't seem to
get the contract to execute successfully.

I ask some questions in the ink matrix channel,
and I feel extremely frustrated.

> "Every combination of attempts I make results in either an "OutOfGas" or "ContractTrapped" error"

> "We're very confused about units and gas and weight. How many underlying integer units are in a "unit" (how divisible is a unit)? Does "Max Gas Allowed (M)" mean the amount of millions of "unit"s payed from the caller to run the contract?"

> "When our default devnet accounts say alice has "1,152" "units" does that man she doesn't have the millions of gas necessary to execute a contract?"

Robin responds:

> "To clarify: cross-contract calls are possible using the CallBuilder API that should ideally not be used since it is a rather low-level abraction API. Instead users should try using the interfaces allowed for by the ink_lang macros."

> "Users generally won't need to use the CallBuilder API for their everyday smart contracts; it is only really required for niche use cases that you can inspect in our multi_sig example contract."

> "The cross-chain calling is something completely different and requires support in lower-levels of abstraction than ink! itself."

> "I am sorry to hear about your confusing user experiences with the CallBuilder API. Answering the question "how much gas is enough?" isn't easy since it depends on the chain, its configuration and also on which version of Substrate it is based (defaults). E.g. for the Canvas network we resolved that approx 5 CANs should be sufficient to operate smart contracts for a week on our chain. The Alice test account normally has plenty of funds and will likely not require any more if the chain configuration for gas prices is sane."

This doesn't quite resolve my confusion,
but now I know the default accounts should have enough "units" to pay for gas.
I still don't have a clue how divisible units are or whether gas is paid in millions.
And if I'm not supposed to use `CallBuilder` to make cross-contract calls,
then I don't know what I am supposed to use instead.

I ask:

> "@Robin I don't see in ink_lang how to call a cross-contract method for contracts that I don't a-priori have the implementation for. I know the signature of the method I want to call, and the account id of the contract i want to call, but don't know the concrete interface. Is there a way to make that call without callbuilder?"

During this time we do figure out one thing we were doing wrong:
we were calling `unwrap` on the results of executing our cross-contract call,
and _that_ was what was triggering the `ContractTrapped` error.
When we stopped unwrapping and printed the result we could see
that the call was returning a `Err(Decode(Error))`.



### Debug-printing the environment doesn't work

In the meantime I decide to debug-log _everything_ in the environment I can to try to understand
what units are what,
and how the numbers I input in canvas-ui translate to the numbers my code sees.

I try to do this in the caller's method:

```rust
ink_env::debug_println(&format!("env: {:?}", self.env()));
ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
```

Just dump the environment.
When I try to upload this to the chain the UI reports an error, and the log says

```
2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:DispatchError                                                                                                             2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:Invalid module for imported function
2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:PostInfo:                                                                                                                 2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:actual_weight=
2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:max-weight
2021-01-16 22:29:48.039  DEBUG tokio-runtime-worker runtime:pays_fee=
2021-01-16 22:29:48.039  DEBUG tokio-runtime-worker runtime:Yes
```

Further experiments with logging the environment don't exhibit this error,
so I do some work reducing this error.
I create a contract that consists only of this:

```rust
#[ink(storage)]
pub struct Game { }

impl Game {
   #[ink(constructor)]
   pub fn default() -> Self {
       Game { }
   }

   #[ink(message)]
   pub fn run_game_test(&mut self) {
       ink_env::debug_println(&format!("env: {:?}", self.env()));
       ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
   }
}
```

It can't be uploaded. Doing so results in the above `DispatchError`.

If I remove the _second_ `debug_println` I can upload and execute this contract.
So this method now is doing nothing but 

```rust
ink_env::debug_println(&format!("env: {:?}", self.env()));
```

It logs

```
DEBUG tokio-runtime-worker runtime:env: EnvAccess
```

So not useful.

Changing the method to do only

```rust
ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
```

Results in a contract that again can't be uploaded.

Anyway, I'm going to move on and continue trying to log whatever bits of the environment I can,
in our original contract method.

I try to log just about every accessor on `self.env()`,
and the only one that results in a contract that successfully uploads is `self.env().caller()`.
Once again I am completely baffled.
Clearly I am doing something wrong because this is just broken.

During all this I note repeatedly that today the deployed canvas-ui's icons are all missing,
replaced by those replacement unicode number filler characters.
It's getting a bit annoying.

So much brokenness.
It's super frustrating.

I make a note that some of these things are probably things _I_ can help fix,
at some point,
once I feel like I have an understanding of how things are supposed to work.
Right now though I don't know what I don't know,
and am just flailing around,
feeling like nothing works.

Anyway today I went backwards,
and am fed up.

So that's it.
Time for a break.





### Wait what's this? Some weird new issue!

When I resume hacking I run into another issue.

I try to run `cargo +nightly contract build` on our game contract,
and it fails like this:

```
ERROR: error parsing LLVM's version: too many version components

Caused by:
    too many version components
```

It seems to be coming from `cargo-contract`,
as just running `cargo +nightly build`
doesn't trigger it.

For reference I am running these versions of these tools:

```
$ rustc +nightly --version
rustc 1.51.0-nightly (4253153db 2021-01-17)
$ cargo contract --version
cargo-contract 0.8.0
```

I notice that `cargo-contract` only reports a version number,
and I _definitely_ installed it from source,
so knowing the exact commit would help.
Something `cargo-contract` could add to its version output,
at least when building from source.
Though I don't know how common it is for programs that aren't
`rustc` to include their commit hash;
and once `cargo-contract` is mature I wouldn't expect many people
to be installing from the master branch.

Ah, now I notice that I have updated my nightly toolchain
since the last time I hacked on this project.
So something seems to have changed in how Rust's
LLVM version is reported.

While annoying,
this kind of thing is not terribly uncommon to run into
when working with Rust,
especially nightly Rust.
I'm wondering though if whatever is happening here
to break `cargo-contract` is a regression in Rust
that needs to be reported.

I don't want to debug this!
But I think I should!

I _could_ try to just upgrade cargo-contract
and see if somebody else has fixed the problem,
but I _really_ think the regression should be
reported to upstream Rust.

I search the Rust issue tracker and don't see
any reports that sound related.

Looking at `cargo-contract`'s git log from my current HEAD to origin/master
I see commits that look like they probably fix this issue:

```
$ git shortlog HEAD..origin/master
Andrew Jones (1):
      2021 file headers (#138)

dependabot[bot] (3):
      Bump async-std from 1.8.0 to 1.9.0 (#156)
      Bump rustc_version from 0.3.2 to 0.3.3 (#155)
      Bump cargo_metadata from 0.12.2 to 0.12.3 (#154)
```

Ok, there's the [fixed issue in `rustc_version`][rcv].

[rcv]: https://github.com/Kimundi/rustc-version-rs/issues/37

This is a nightly-only problem,
as only Rust nightly reports the LLVM version,
so there's no upstream bug to report.

```
$ rustc -vV
rustc 1.49.0 (e1884a8e3 2020-12-29)
binary: rustc
commit-hash: e1884a8e3c3e813aada8254edfa120e85bf5ffca
commit-date: 2020-12-29
host: x86_64-unknown-linux-gnu
release: 1.49.0

$ rustc +nightly -vV
rustc 1.51.0-nightly (4253153db 2021-01-17)
binary: rustc
commit-hash: 4253153db205251f72ea4493687a31e04a2a8ca0
commit-date: 2021-01-17
host: x86_64-unknown-linux-gnu
release: 1.51.0-nightly
LLVM version: 11.0.1
```

I install the latest master version of `cargo-contract`
and go on with my life:

```
cargo install --path .
```


### Errors in ink, and a `CallBuilder` miracle

This will be our 4th attempt to successfully call another contract with `CallBuilder`.

In the meantime Robin responded to more of my questions in chat.

From that I learned that I should definitely be able to do what I'm attempting,
but more importantly that I can set the gas limit to 0 in a cross-contract call,
and that will just make the gas limit unlimited.
That will remove one of the hurdles we've had during development &mdash;
figuring out how much gas to provide in the canvas-ui.

Again,
we are working [off of a repo][gtr] that just
contains this reduced problem of calling
a method in another contract,
while only knowing the
callee's account id and
method selector.

[gtr]: https://github.com/Aimeedeer/game-test

Since last time we tried this,
I was given some more advice by Robin
about how to call other contracts.
This feature of calling another contract by interface
doesn't exist yet in the ink DSL,
but should [eventually be possible][dyntrait] through
`#[ink::trait_definition]`.
For now what we are trying to do
by manually manipulating method selectors
and using the low-level `CallBuilder`
should work &mdash; we just haven't understood how yet.

[dyntrait]: https://github.com/paritytech/ink/issues/631

We've learned two important things recently

- We can just set the `gas_limit` to "0",
  which means it is unlimited.
- Our `CallBuilder`'s `fire` method was returning an
  `Err(Decode(Error))`,
  that we were unwrapping instead of handling.

So we've fixed both those problems:
having unlimited gas will let us not worry about
choosing the correct gas price during development;
and now our task is to figure out why our call
is returning a decoding error.

I want to do a few things:

- Look up the `CallBuilder::fire` method and see what
  errors it returns.
- Ripgrep the codebase for this `Decode` error we are hitting,
  try to understand why it happens.
- Ripgrep the ink codebase for more examples
  of using `CallBuilder` and see if there are things
  those examples are doing that we are not.

Here's how we are using `CallBuilder` as of now:

```rust
let return_value = build_call::<DefaultEnvironment>()
    .callee(program_id) 
    .gas_limit(0)
    .transferred_value(0)
    .exec_input(
        ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF]))
    )
    .returns::<ReturnType<bool>>()
    .fire();

ink_env::debug_println(&format!("return value {:?}", return_value));
```

First up, let's investigate the errors we're getting from the [`fire`] method.

Compared to the rest of `CallBuilder`,
which is very generic,
this has a simple signature:

```
pub fn fire(self) -> Result<(), Error>
```

This method is returning the error I am currently stuck on.

`Error` here is the [`ink_env::Error`] enum,
which, as we would hope,
has the variant we've seen called `Decode`.
It contains an inner `Error` type that is
curiously not linked to anything useful in the API docs.

[`fire`]: https://paritytech.github.io/ink/ink_env/call/struct.CallBuilder.html#method.fire
[`ink_env::Error`]: https://paritytech.github.io/ink/ink_env/enum.Error.html

That probably means it originates from
somewhere outside of the source tree the docs are being generated from.
I would guess it comes from a SCALE crate,
which is responsible for serialization in Substrate.
But let's not guess,
let's go look at the ink code.

[Yeah, it's a `scale::Error`][itsascaleerror].
And from reading the `Cargo.toml` file I see that the `scale` crate
is [actually called `parity-scale-codec`][apsc].

[itsascaleerror]: https://github.com/paritytech/ink/blob/01f987d7f70b8b1bbc05fe016021d2d77e3ded54/crates/env/src/error.rs#L24
[apsc]: https://github.com/paritytech/ink/blob/01f987d7f70b8b1bbc05fe016021d2d77e3ded54/crates/env/Cargo.toml#L23

We're going to want to log this error,
but first let's just look up those docs
to see what we're in for.

It suddenly occurs to me that I don't know where
to find the Substrate API docs.
I know the Ink API docs are self hosted,
and not on docs.rs.

Rust API docs that aren't on docs.rs are a minor annoyance.

I go to substrate.dev,
and click the prominent "API Reference" link.
It leads to yet another URL:

> https://substrate.dev/rustdocs/

Navigating to that URL then redirects to ...

> https://substrate.dev/rustdocs/v2.0.1/sc_service/index.html

Okay... is `sc_service` really the first page
a user should read about Substrate API docs?
I don't know enough to say "no",
but it's definitely not what I'm looking for.

Anyway, I should be able to search for `scale`
or `parity-scale-codec` in the top search bar.

I search for `scale` and see the top hits:


|name|description|
|--|--|
|sp_runtime::traits::Scale|Multiply and divide by a number that isn't necessarily … |
|alga::general::ComplexField::scale|Multiplies this complex number by factor.|
|nalgebra::ComplexField::scale|Multiplies this complex number by factor.|
|nalgebra::base::Matrix::scale|Multiplies each component of the complex matrix self by …|
|num_complex::Complex::scale|Multiplies self by the scalar t.|
|statrs::distribution::Cauchy::scale|Returns the scale of the cauchy distribution|
|statrs::distribution::Pareto::scale|Returns the scale of the Pareto distribution|
|statrs::distribution::StudentsT::scale|Returns the scale of the student's t-distribution|
|statrs::distribution::Weibull::scale|Returns the scale of the weibull distribution|

Junk.

Searching for `parity-scale-codec` yields no results.

_Searching for `parity_scale_codec` (with underscores) finds the crate._

This search failure seems to be rustdoc's fault &mdash;
crates can have dashes in their names,
but in code they must be valid Rust identifiers,
so the dashes are converted to underscores;
and the code is where the docs originate.

Finally,
after quite a lot of digging we find
the [`scale::Error` docs][sed].

[sed]: https://substrate.dev/rustdocs/v2.0.1/parity_scale_codec/struct.Error.html

This error doesn't give much useful info:
it contains one method:

```rust
pub fn what(&self) -> &'static str
```

and this method is documented that

> "This function returns an actual error str when running in std environment, but "" on no_std."

As our contract is a no-std environment,
we can expect to get no further useful info out of this error.

We're going to try to log it anyway and see what it says.

We modify our post-call code to log the error in two ways:

```rust
match return_value {
    Err(ink_env::Error::Decode(e)) => {
        ink_env::debug_println(&format!("e: {:?}", e));
        ink_env::debug_println(&format!("estr: {}", e.what()));
    }
    _ => { }
}
```

We expect the `what()` call to return an empty string,
but _maybe_ debug-printing the error itself will give some
useful info.

Probably not. Yeah, after reading the implementation of `scale::Error`,
we're not going to get any useful info out of it.
Oh well, let's prove it by doing the test.

I run the contract.

&nbsp;

_**`· A MIRACLE HAPPENS ·`**_

&nbsp;

The `CallBuilder` call worked. No error. Our logs say so:

```
DEBUG tokio-runtime-worker runtime:return value Ok(false)
```

This is a huge suprise,
as we made no changes to the `CallBuilder` invocation
that we are aware of.

What did we do differently this time!?

I stash the changes I've made to the contract today,
and try again,
using a setup that failed for us many,
many times.
The only changes should be that I have a different
Rust nightly compiler,
and that should be irrelevant.

The old contract succeeds.
Very unexpectedly.

Yay?

I mean, "nooooo".

This is the worst kind of success.
Literal days of work,
spanning weeks,
and we don't know how we fixed our bug.

I'm tempted to say we learned _nothing_,
but that's not true.
We did learn quite a few things,
_but not what we did wrong_.

This is a tableflip moment.

I have Aimee reproduce the experiment,
with our _old_ code,
that has also _failed for her_,
many times.

After like 10 minutes of setup,
in which canvas-ui fails us in at least 2 new ways (††),
she executes our test game contract.

I cross my fingers,
but I don't even know what I'm hoping for:
for the contract to succeed,
or for the contract to fail again
so we can continue to debug what we did wrong.

The old contract succeeds again,
whether I run it,
whether she runs it.

We step away and have a drink.

> (††) "canvas-ui fails us in at least two new ways":
>
> 1) During contract upload,
     the status spinner spun forever.
     Re-navigating to the "upload" tab
     revealed that the contract had been successfully uploaded.
     After this, the "deploy" tab showed the same contract uploaded
     twice, both with the same code hash.
>
> 2) We told canvas-ui to "forget" a deployed contract,
     with the intent of re-deploying it.
     Upon attempted redeployment we received an error,
     and the canvas-node logs said something along the lines
     of "live contract already exists".
     Unfortunately we did not have that contract's account id,
     the error message didn't tell us,
     the polkadot.js/app explorer couldn't tell us that contract's
     account id (presumably because it wasn't running at the time
     the event was dispatched);
     so we didn't know what else to do but restart our devnet.



### Completing the level progression logic

Ok, after that last experience we took a multi-week break,
and [tried out Dfinity][dft].

[dft]: https://brson.github.io/2021/01/30/dfinity-impressions

Now, we are determined to make a final push to complete this project's prototype.

With our game contract able to call player level contracts via their account ID's and method selectors,
today we are going to try to add level progression,
where completing one level opens up the next level.

Since it has been a while,
I am again going to update my tools.
Since the last time I built `cargo-contract` something has changed,
and it no longer works without `wasm-opt` installed.
I dig into and and find out I need to build it with `--features=binaryen-as-dependency`
so that it will bundle the `wasm-opt` tool.

While I've been goofing off and hacking on other projects,
Aimee has continued trying to complete our proof of concept contract,
adding level progression,
so that when a player succeeds at level 0,
they get access to level 1,
etc.

I'll not show the logic inline here,
but [it's on GitHub][run_level].

[run_level]: https://github.com/brson/contract-game/blob/c73b3378206bb41d93c9cc1786a63f94240e7753/src/game/lib.rs#L158

Since we managed to get `CallBuilder` working,
we continue to have no problems with it,
and still don't know what we were ever doing wrong.

`canvas-ui` when executing a transaction doesn't show anything about the return value,
and in particular doesn't show anything when our methods return `Err`.
To compensate for this we find it necessary to add logging to every exit condition
so that we can figure out what happened.
This won't be as crucial once we are exercising the contract via our own UI,
and our UI can interpret our own error return values.

At this point we have completed the essential contract
features of our prototype.





## The many-step, error-prone, build-deploy-test cycle

We have found that the testing process we are stuck in is extremely onerous,
so much so that after a few rounds we get discouraged
and walk away for a while.
And when we think about coming back to the project,
the knowledge that the test cycle is going to be so tedius
often makes us procrastinate and do something else instead.

Since we were unable to discover what we were doing wrong
when trying to use `CallBuilder` (our code ultimately
just started working on its own)
I am inclined to attribute our extended confusion to human
errors in this build-deploy-test cycle.

Here are more-or-less the steps we need to take for every change:

- Build the game contract
  - `cargo +nightly contract build --manifest-path/src/game/Cargo.toml`
- Build the player level contract(s):
  - `cargo +nightly contract build --manifest-path/src/example-levels/flipper/Cargo.toml`
  - For now every level is just "flipper", but later each level will be different
- Run a clean node:
  - `canvas-node --dev --tmp -lerror,runtime=debug`
- Clean the `canvas-ui` caches
  - "Forget" any existing code bundles
  - "Forget" any deployed contracts
- Upload game contract with Alice account
  - via `canvas-ui`
- Deploy game contract with Alice account
  - via `canvas-ui`
- Upload each player level program with Bob account
  - via `canvas-ui`
  - Again, at the moment its just one flipper program
- Deploy each player level contract with Bob account
  - via `canvas-ui`
- Execute game contract via Bob's account
  - via `canvas-ui`
  - While correctly setting the arguments every time,
  - Call `create_account` transaction
    - Call then sign
    - Check logging output
  - Call `have_player_account` RPC
    - Call
  - Call `submit_level` transaction
    - Call then sign
    - Check logging output
  - Call 'run_level' transaction
    - Call
    - Check logging output
  - etc.

All of these steps are manual,
and involve flipping between the command line and the web browser,
and a lot of mouse-clicking.

Obviously there's some automation we could add,
scripting up some test cases.
The big obstacle to that is that we don't know how to deploy
and instantiate contracts from the command line yet,
nor how to call methods on a contract;
the features in `cargo-contract` that enable some of those things
are under an experimental `extrinsics` feature flag,
and compilation of those features is currently broken.



## Testing ink contracts

So far we have not written any unit tests for our contracts.
In our experience, ink unit tests are limited,
require some fiddly low-level glue code,
and I am skeptical that they reasonably capture the behavior of
the system running live.
I am sure unit tests will be useful the more software I write,
and as ink matures.

There are a number of `cfg` attributes and features
that an ink contract seemingly needs to maintain in order
to establish the difference between the normal on-chain
runtime environment, where there is no standard library;
and the unit-testing environment,
which does have access to the standard library.
So far these attributes,
`std`, and `ink-as-dependency`,
are just magic to us and we copy-and-paste
them from examples without understanding.

I have an inclination to just delete them all
and always assume the code is running on-chain,
forget about unit testing.

I am more interested in integration testing on a dev chain,
and intend to instead write scripts that run a chain,
deploy contracts, call methods on those contracts, etc.

We have not yet attempted to write such tests,
but it is high on our list of priorities,
particularly due to the extreme annoyance of all the manual steps
that we are doing to test our changes today.



## Fixing a build failure in `cargo-contract`

*Note: this entire section is just an exercise in updating
out-of-date dependencies in cargo.*

As mentioned above,
the testing process for ink contracts requires
too much manual work.

One of the first things we need to be able to do to
script some test cases is to deploy contracts from the command line.

To do that, we need the experimental `extrinsics`
feature of `cargo-contract`.
I know the first time I looked at this feature I couldn't understand
how to use it,
and the last time I looked it failed to build,
but I'm curious if I can improve my workflow by doing a little
yak shaving, get this feature working.

Start ing at `cargo-contract` commit 79dbcb655dd77701c79b2f1b459767ac3108cc58
I try to build with the following command line:

```
cargo build --features=binaryen-as-dependency,extrinsics
```

And get the error:

```
error[E0603]: module `export` is private
   --> /home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/parity-multiaddr-0.9.6/src/onion_addr.rs:3:12
    |
3   | use serde::export::Formatter;
    |            ^^^^^^ private module
    |
note: the module `export` is defined here
   --> /home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.123/src/lib.rs:275:5
    |
275 | use self::__private as export;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^

   Compiling contract-metadata v0.2.0 (/home/ubuntu/substrate/cargo-contract/metadata)
error: aborting due to previous error
```

This error is in `parity-multiaddr`,
which seems to only be part of the build when the `extrinsics` feature
is activated.
From the error it seems like `parity-multiaddr` is just doing
something bogus,
so my first idea to fix it is to just see if the crate can be upgraded.

I run `cargo update`:

```
$ cargo update -p parity-multiaddr
    Updating crates.io index
    Updating parity-multiaddr v0.9.6 -> v0.9.7
```

That does indeed update the lockfile to a new revision of parity-multiaddr,
and running the build again succeeds in building that crate,
but runs into a new error:

```
error[E0433]: failed to resolve: could not find `export` in `syn`
  --> /home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/frame-support-procedural-2.0.0/src/storage/parse.rs:50:37
   |
50 | impl<P: syn::export::ToTokens> syn::export::ToTokens for Opt<P> {
   |                                     ^^^^^^ could not find `export` in `syn`

error[E0433]: failed to resolve: could not find `export` in `syn`
  --> /home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/frame-support-procedural-2.0.0/src/storage/parse.rs:50:14
   |
50 | impl<P: syn::export::ToTokens> syn::export::ToTokens for Opt<P> {
   |              ^^^^^^ could not find `export` in `syn`

error: aborting due to 2 previous errors
```

Another peculiar build error.
Both the previous and this one look to be the kind of errors that would
arrive from semver-incompatible changes.
If that _is_ the case, it's disappointing,
and especially in these crates &mdash; `serde` and `syn`.
Or maybe substrate was using these crates in a way
that _used_ to work with old Rust revisions,
but bugs in Rust's name resolution broke them.
I don't know. Just speculating.
This kind of breakage in the Rust ecosystem is annoying though.
Rust isn't nearly as version-stable as it wishes it were.

I take the same approach as to the last problem and try
to upgrade `frame-support-procedural`:

```
$ cargo update -p frame-support-procedural
    Updating crates.io index
    Updating frame-support-procedural v2.0.0 -> v2.0.1
```

Another encouraging version bump.

And again that fixes the build of that crate.

But, and &mdash; *sigh* &mdash; _not_ the full cargo-contract build.
I wonder what happened since the last time this build configuration
built successfully and now.

Let's turn some CI on for this configuration.

The new error:

```
error[E0282]: type annotations needed
    --> /home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/sp-arithmetic-2.0.0/src/fixed_point.rs:541:9
     |
541  |                   let accuracy = P::ACCURACY.saturated_into();
     |                       ^^^^^^^^ consider giving `accuracy` a type
...
1595 | / implement_fixed!(
1596 | |     FixedI64,
1597 | |     test_fixed_i64,
1598 | |     i64,
...    |
1601 | |     "_Fixed Point 64 bits signed, range = [-9223372036.854775808, 9223372036.854775807]_",
1602 | | );
     | |__- in this macro invocation
     |
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
```

This one looks like the result of a type inference change in Rust.

I take the same approach to fixing it:

```
$ cargo update -p sp-arithmetic
    Updating crates.io index
    Updating sp-arithmetic v2.0.0 -> v2.0.1
```

Clearly somebody in the substrate world has identified and fixed all these problems,
and `cargo-contract` has just not been updated.
I am doing that update.

Again build, again progress.

Again, a similar build error:

```
   Compiling sc-rpc-api v0.8.0
error[E0282]: type annotations needed
   --> /home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/frame-system-2.0.0/src/offchain.rs:187:22
    |
186 |                 let public = generic_public.into();
    |                     ------ consider giving `public` a type
187 |                 let account_id = public.clone().into_account();
    |                                  ^^^^^^ cannot infer type
    |
    = note: type must be known at this point
```

This time though `cargo update -p frame-system` does not give me
the expected version bump from version 2.0.0.
I check crates.io and see that there _is_ a newer version, 2.0.1.
Why couldn't cargo automatically update it?

I read the `cargo update --help` and add some more command line flags:

```
$ cargo update -p frame-system --precise 2.0.1
    Updating crates.io index
    Updating frame-support v2.0.0 -> v2.0.1
    Updating frame-system v2.0.0 -> v2.0.1
```

So probably it didn't work before because `frame-support` needed to be bumped
prior to bumping `frame-system`.

Again I make build progress.

Again the build fails. Again I fix it similarly.
And again. And again.

This time I have to do a version-incompatible bump
of `substrate-subxt` from 0.13.0 to 0.14.0,
by modifying the `cargo-contract` manifest,
not the lockfile.

I finally complete the build.
Since some of this breakage looks like language-level breakage,
I wonder about my own compiler.
I see I have 1.49, from December,
and run `rustup update` to install 1.50.

I run the build again.
It succeeds.

I [submit a PR with my fixes][expr].

[expr]: https://github.com/paritytech/cargo-contract/pull/175

I do not get around to attempting to script up
parts of the testing process.

Someday.




## Connecting to our contract with polkadot-js

While Aimee was mostly working on the contract code,
I started figuring out how to interact with substrate
via JavaScript,
in the browser,
so we could prototype the UI.

The library for interacting with substrate nodes is
[polkadot.js].

It's strange that the JS compononts are "polkadot"-branded,
where so many other things in this ecosystem are "substrate"-branded,
but I imagine that's a historical artifact of the way polkadot evolved.

I am hoping I can write the frontend as a simple static web site.
I try not to use npm unless I have to,
and prefer to create a simple frontend using plain HTML and JavaScript.
All the documentation for the polkadot JS API assumes the use of npm/yarn,
which I would rather not.
I am trying to figure out how to use webpack to package up `@polkadot/api` so I can use it outside npm,
but don't know how.

I have previously succeeded in this with web3.js and ipfs.js,
but don't really remember how.

I ask in #ink:matrix.parity.io

> I'm not very familiar with npm programming, and I want to use the
  @polkadot/api from a non-npm script in the browser. Is it possible to use
  webpack to package @polkadot/api into a standalone javascript file that I can
  use outside of npm. Any hints how to do that?

In the meantime I give up trying to package polkadot-js for use
outside of npm,
and try to set up a yarn app that will let me import the library
in the expected way.

As someone mostly unfamiliar with npm I immediately encounter more problems.
I know how to add `@polkadot/api` to a yarn project,
but I don't know how to set yarn up to run a server on `yarn start`.
From Googling, as with most things in the npm ecosystem,
there seem to be many options.

Similar to the Ethereum docs,
I'm finding that the polkadot-js docs completely gloss over topics
about setting up npm/parn projects,
and I am completely lost.

I know that I can't expect Polkadot to teach me npm,
just like I can't expect them to teach me Rust,
but this has been a huge problem for me every time I try to use modern JavaScript.

On https://polkadot.js.org/docs/api/examples/promise/ it says

> "From each folder, run yarn to install the required dependencies and then run
  yarn start to execute the example against the running node."

But there are no "folders" in this documentation.
Is there a link to actual, complete, ready-to-run source code I'm missing?

I additionally ask in the "#ink" channel if there's a basic
yarn template for using the polkadot JS API's.

Dan Shields comes through with a link to

> https://github.com/substrate-developer-hub/substrate-front-end-template

I recall seeing this before and must have forgotten about it.
I'll try to reboot my web efforts from this template.

Well, maybe I'll just use it for hints.
It's a react app, and I really don't want to learn react right now.
So complex.

I am going to try instead using `webpack-dev-server` for my `yarn start` command.

I eventually follow the [webpack "Getting Started" guide][wpgs].
I'm real yak shaving now.

[wpgs]: https://webpack.js.org/guides/getting-started/

While running weback with my script that imports "@polkadot/api" I run into this error:

```
ERROR in ./node_modules/scryptsy/lib/utils.js 1:15-32
Module not found: Error: Can't resolve 'crypto' in '/home/ubuntu/contract-game/www2/node_modules/scryptsy/lib'

BREAKING CHANGE: webpack < 5 used to include polyfills for node.js core modules by default.
This is no longer the case. Verify if you need this module and configure a polyfill for it.

If you want to include a polyfill, you need to:
        - add a fallback 'resolve.fallback: { "crypto": require.resolve("crypto-browserify") }'
        - install 'crypto-browserify'
If you don't want to include a polyfill, you can use an empty module like this:
        resolve.fallback: { "crypto": false }
 ```

To fix this it seems I have to add a `webpack.config.js` and configure it per the
[webpack "resolve" instructions][wri].
After creating `webpack.config.js` I can more-or-less copy-paste the suggestion
straight from the command line.
My new `webpack.config.js` looks like

```js
const path = require("path");

module.exports = {
    entry: './src/index.js',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'dist'),
    },
    resolve: {
        fallback: {
            "crypto": require.resolve("crypto-browserify")
        }
    }
};
```

I also have to `yarn add crypto-browserify`.
Once I do I see more similar errors about polyfills,
and when I finally have webpack working I have
three polyfills in my `webpack.config.js`:

```js
        fallback: {
            "buffer": require.resolve("buffer"),
            "crypto": require.resolve("crypto-browserify"),
            "stream": require.resolve("stream-browserify")
        }
```

[wri]: https://webpack.js.org/configuration/resolve/

... Hours and hours go by as I Google how to do basic webpack stuff ...

I'm using webpack 5,
which doesn't do a bunch of node polyfills when it compiles
for the browser.
I think that's a big part of my pain.

After tons of Googling and hacking I finally manage
to load the polkadot JS API in my mostly vanilla JS
web page.

I have to do a lot of hacks.

At the end my `webpack.config.js` looks like

```js
const path = require("path");
const webpack = require("webpack");

module.exports = {
    mode: "development",
    entry: './src/index.js',
    output: {
        filename: 'js/bundle.js',
        path: path.resolve(__dirname, 'dist'),
    },
    resolve: {
        fallback: {
            "buffer": require.resolve("buffer"),
            "crypto": require.resolve("crypto-browserify"),
            "stream": require.resolve("stream-browserify")
        }
    },
    plugins: [
        new webpack.ProvidePlugin({
            Buffer: ['buffer', 'Buffer'],
        }),
    ]
};
```

That `plugins` section is new and mysterious,
just copy pasted from [some commit to some other
software project][webhack].

[webhack]: https://github.com/duplotech/create-react-app/commit/d0be703d40cd4bc32cd91128ba407a138c608243#diff-8e25c4f6f592c1fcfc38f0d43d62cbd68399f44f494c1b60f0cf9ccd7344d697

I also have this lovely garbage in my HTML header
before loading my webpack bundle:

```html
<script>
  let global = window;
  let process = {
    "versions": null
  };
</script>
```

Yup.

Obviously I don't know what I'm doing here.



### Next attempt to get a simple UI working

Ok, this is at least a month after the last attempt at hacking on the UI.
In the meantime, we spent a lot of time debugging our contract,
and also did [that experiment with Dfinity][dft].

Today my goal is to execute transactions in our contract using
the test accounts.

My node setup is wonky, so I better describe it:

I am just using webpack to give me a bundle of the JS libraries I need,
and everything else is just a static website, served by `yarn start`.
I am doing this because I like and understand static websites:
I want to be in the web platform world, not the npm/yarn/node world.
If this were to progress beyond a prototype I would put more effort
into learning proper node development.

[Here is the source][jssrc] as of the current commit.

[jssrc]: https://github.com/brson/contract-game/tree/45788de48f86b39baa8ba721bfbce8d32d109d75/www

My webpacked JS file, under `src/index.js` just contains this:

```javascript
"use strict";

import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { CodePromise, BlueprintPromise, ContractPromise, Abi } from "@polkadot/api-contract";

document.polkadotApiBundle = {
    ApiPromise,
    WsProvider,
    Keyring,
    CodePromise,
    BlueprintPromise,
    ContractPromise,
    Abi
};
```

So this is collecting all the node-built APIs I want and stuffing them into `document.polkadotApiBUndle`.

All my other code is in the `dist` directory:
that webpacked bundle becomes `dist/js/bundle.js`,
and everything else in `dist` is static hand-written code.

This is a single-page app that lives in `index.html`,
and it is a single-page app because,
for now at least,
I don't want to worry about maintaining account information across pages,
and reconnecting to the substrate node between pages.

The script loading for this page looks like this:

```html
<script src="js/polyfill.js"></script>
<script src="js/bundle.js" async></script>
<script src="js/script.js" type="module" async></script>
```

That first synchronously-loaded `polyfill.js` is the hack I made last time:

```javascript
"use strict";

let global = window;
let process = {
    "versions": null
};
```

Other node polyfills are specified in my `webpack.config.js`,
and I figured out how to do them by tedious googling of each;
but these two tiny fills I couldn't find a real solution for,
so just wrote these definitions myself as the running script threw errors.

My application code is in `js/script.js`.
I made it a module so that I can freely use globals inside of it.
Otherwise I don't know much about JavaScript modules.

Using mostly just the [polkadot.js docs][pjsdocs] I quickly
figure out how to

* connect to a node,
* connect to a keyring, at least for the `//Alice` and `//Bob` test accounts,
* call a method on the game contract

[pjsdocs]: https://polkadot.js.org/docs/

I run into problems with calling contracts,
and it turns out that
my "//Bob" address is wrong &mdash; when I print it,
it is not the same address that I see for Bob in the polkadot.js explorer.
It was because I wasn't specifying a "sr25519" keyring.
I thought "sr25519" was the default but I had to specify it as

```javascript
keyring = new polkadot.Keyring({ type: "sr25519" });
```

The [keyring docs][krdcs] say to use `ss58Format: 2`,
but I find if I do this that my key looks different
than the one on the Polkadot block explorer for my devnet.

[krdcs]: https://polkadot.js.org/docs/keyring/start/create

I believe the reason the polkadot.js docs seem to lead me
to write code that is sometimes incompatible with canvas
is that canvas is using a several-months-old revision of substrate.
Unfortunately, simply reading the canvas-ui code looking
for examples is pretty tough since that code is
pretty big and abstract, built on react components.
The few times I looked through it trying to figure out
how to use the polkadot.js APIs I just gave up,
and continued guessing and searching through substrate GitHub
issues until I stumbled on the answers I needed.

Note that since this was written,
canvas-node has been upgraded
to a more recent substrate.



## We fail to complete the prototype

That is as far as we got before we lost our enthusiasm,
got distracted with other projects,
and decided we had to publish this bitrotting blog post before finishing the prototype.

The game contract itself has all the features we aimed to implement,
though we didn't implement any actual game levels;
the UI is not implemented though;
and we have never deployed to the live network.

I am not happy to say it,
but the overriding experience here was one of frustration.

We might come back to this project,
and we'll probably continue hacking with substrate/ink,
but we're taking a break for a while.



## Learnings and Tips

- Substrate is in rapid development,
  and you will need to be committed to keep up with
  its changes. Beyond my own experience, I have
  heard this mentioned by many substrate developers.
- Logs from `debug_println` only appear if you run
  `canvas-node` with `-lruntime=debug`.
- Run `canvas-node --dev --tmp -lerror,runtime=debug`.
  The `-lerror,runtime=debug` reduces spew by setting
  the default log level to "error", and also emits
  logs from `debug_println`.
- The [polkadot explorer][pex] can connect to your devnet.
  Click on the "Polkadot" dropdown menu in the top left,
  then the "development" dropdown;
  select "local node",
  then click "switch".
- In aggregate, the various tools involved in ink development
  seem to break pretty often. It's not clear if its best to
  try to keep up with the master branches of these projects or not.
- There is little debuggabality for ink contracts,
  and `canvas-ui` often won't interpret errors returned during normal
  execution. Put `debug_println` logging in every error branch to understand what is
  going on inside the contract.
- At least some substrate errors (`scale::Error`),
  within the wasm contract sandbox,
  do not carry dynamic metadata that would be useful to help interpret them,
  and that is available when compiled outside of wasm.
- Install the [subkey command][subkeycommand].
- With `CallBuilder`, just set `gas_limit` to 0 during development:
  it will use as much gas as needed,
  and on the canvas chain,
  you should have more than enough gas in the test accounts.


## Some thoughts

The rest of this post is pretty much a rant I wrote down
in a few minutes sometime mid-January,
to capture my thoughts about smart contract programming so far,
with Substrate and other systems.

### First, some hopefulness

As I learn about Substrate and the corners of its ecosystem,
I do feel a bit giddy.
The high level docs are pretty enlightening to read,
and I find myself keeping them open in a browser tab,
and just perusing them sometimes;
I think they provide some good insight into the structure
of modern programmable blockchains generally,
not just Substrate.
And when I discovered that Polkadot explorers,
specifically the one at [polkadot.js.org][pex],
can more-or-less seamlessly work with any substrate chain,
even my own dev chain,
I was kind of blown away.

I can see the possibility of many people building
new and different chains, that can all interoperate,
on one powerful toolkit.

It seems like it could be fun to be a part of.


### Next, some venting

Right now though, it is not fun.
Writing applications on blockchains is not fun,
both generally,
and specifically in the case of Substrate,
for many reasons.

Smart contract programming continues to be challenging.
I know learning new things always is,
but I'm finding myself surprised at how long it is
taking me to turn the corner and understand what I'm doing.

Programming a smart-contract-based application for
Substrate requires at least three kinds of expertise:

- Rust programming, and its idiosyncracies and ecosystem
- Node.js programming, and its complexly layered ecosystem
- Blockchain technology, and its alien and rapidly changing
  form of software construction and interaction

If one doesn't literally need to be an _expert_ in
these things, then at least one needs intermediate experience
in both Rust and Node.js to make any kind of satisfying progress
on the path to learning Substrate.

Each of these three technologies is intimidating,
but combining them is brutal.

And before much longer smart contract programmers are going
to need to understend zero-knowledge cryptography too.

That's not to mention tokenomics,
but yeah, that's anothing thing you've got to understand.

How can this complexity ever be reduced to something an average programmer can
manage? The blockchain ecosystem just seems to be exploding in complexity.

I can hardly imagine the path forward,
but I hope one day there is a contraction and consolidation
of all the complexity,
of the extremely many slightly-different models of permissionless
distributed programming evolving all over the industry.


This is all without even addressing the embarassingly
immature state of developer tooling in blockchain ecosystems.
It will be a decade before the smart contract programming experience,
on any chain,
is as convenient as the traditional programming experience.
There just aren't enough resources dedicated to it,
nor reason to bother dedicating those resources,
with blockchain programming having such a
small developer audience.


### At least I'm learning

My partner is learning to program right now,
and I am patiently watching her go through all the difficulties
every programmer goes through as they discover basic computer
science topics. I try to be empathetic by remembering
my learning experience so long ago, of hacking out simple
programs without understanding.

I am feeling a lot of that pain again now as a smart contract learner.
I have been following the progress of blockchain tech for
quite a long time,
and doing some hands on programming on and off for
nearly a year.
That combined with my long background in writing software
gives me a lot of intuition about how software running
on the blockchain _should_ work.
But I still feel lost.
I am still not seeing the big picture.

I am going to continue trying though,
and hoping for a moment of enlightenment,
where it all feels right,
and where the fun begins.

