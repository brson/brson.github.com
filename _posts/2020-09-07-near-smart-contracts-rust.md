---
layout: post
title: First impressions of NEAR smart contract development in Rust
tags: [rust, blockchain]
---

Rust is increasingly used as a programming language for smart contracts.
Whereas the first generation blockchains used specialized VMs,
newer blockchains are running general purpose VMs,
especially [WASM].
And of course,
Rust is a modern language that runs just about everywhere,
and is especially good at targetting WASM.
Somewhat serendipitously,
this has made Rust one of the best candidate languages for writing
smart contracts on the new generation of blockchains.

Lately I've been learning more about the smart contract programming landscape,
and this post is about my first dive into smart contract programming in
Rust on the [NEAR] platform.

[NEAR]: https://github.com/nearprotocol/nearcore
[WASM]: https://webassembly.org/

NEAR is a sharded, proof-of-stake blockchain that runs WASM.
It not only supports smart contracts written in Rust
(in addition to [AssemblyScript]),
but is also itself written in Rust.
While one the most prominent Rust blockchain projects,
it is perhaps not yet widely known in the broader blockchain development community.

[AssemblyScript]: https://www.assemblyscript.org/

Based on my experiences here though,
I'm quite enthusiastic about the NEAR developer experience,
and I intend to spend more time hacking on NEAR.

These are my first impressions of NEAR,
as someone with moderate knowledge of blockchains,
but limited previous blockchain programing experience.
I have previously written similarly about my first experiences
with other blockchains: [Nervos][nervos], [Ethereum][ethereum].

[nervos]: https://talk.nervos.org/t/experience-report-first-time-building-and-running-ckb/4518/
[ethereum]: https://github.com/Aimeedeer/bigannouncement/blob/master/doc/hacklog.md


## Starting at the start of the docs

The [NEAR documentation][docs.near.org] seems refreshingly thorough,
if a bit confusing to navigate.
There are different docs to follow depending on your role in the network,
whether a validator,
smart contract programmer,
etc.
and I find myself jumping between multiple sections,
without quite understanding how the docs are organized overall,
perhaps because there isn't a single table of contents that covers the entire site.

Anyway, after browsing the docs to get a general idea of their organization,
I go back to the front page and I click "Basics".
This seems to be leading me down the path of being a smart contract developer,
not running my own local node,
not building NEAR itself.

[docs.near.org]: https://docs.near.org

For sake of understanding,
I generally want to build the entire stack I'm working on,
but for now I'm going to follow the docs exactly and see where that gets me,
and not build NEAR on my own.


## Account creation

Setting up an account is done through a web interface at:

> <https://wallet.testnet.near.org/>

I create a testnet account,
`floopy.testnet`,
and a recovery phrase.

The NEAR explorer page for my account is

> <https://explorer.testnet.near.org/accounts/floopy.testnet>

Instead of a hash for an ID,
as in most blockchains,
you get an actual string name.
And the system can optionally do account recovery over email or phone.
While this is convenient, it seems suspiciously centralized for a public blockchain,
and I wonder if it is possible to create accounts without any central authority.
I imagine it is, and this website is mostly a convenience, but don't know yet.


## Installation

Following [the docs][clidocs], I install `near-cli`, a Node app.

[clidocs]: https://docs.near.org/docs/development/near-cli

```
$ npm install -g near-cli
/home/ubuntu/.nvm/versions/node/v12.16.2/bin/near -> /home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/bin/near

> node-hid@1.3.0 install /home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/node_modules/node-hid
> prebuild-install || node-gyp rebuild


> usb@1.6.3 install /home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/node_modules/usb
> prebuild-install --verbose || node-gyp rebuild

prebuild-install info begin Prebuild-install version 5.3.5
prebuild-install info looking for cached prebuild @ /home/ubuntu/.npm/_prebuilds/f5a2d7-usb-v1.6.3-node-v72-linux-x64.tar.gz
prebuild-install http request GET https://github.com/tessel/node-usb/releases/download/v1.6.3/usb-v1.6.3-node-v72-linux-x64.tar.gz
prebuild-install http 200 https://github.com/tessel/node-usb/releases/download/v1.6.3/usb-v1.6.3-node-v72-linux-x64.tar.gz
prebuild-install info downloading to @ /home/ubuntu/.npm/_prebuilds/f5a2d7-usb-v1.6.3-node-v72-linux-x64.tar.gz.26621-920b88608b4cf.tmp
prebuild-install info renaming to @ /home/ubuntu/.npm/_prebuilds/f5a2d7-usb-v1.6.3-node-v72-linux-x64.tar.gz
prebuild-install info unpacking @ /home/ubuntu/.npm/_prebuilds/f5a2d7-usb-v1.6.3-node-v72-linux-x64.tar.gz
prebuild-install info unpack resolved to /home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/node_modules/usb/build/Release/usb_bindings.node
prebuild-install info unpack required /home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/node_modules/usb/build/Release/usb_bindings.node successfully
prebuild-install info install Successfully installed prebuilt binary!
```

Semes to work fine.

From previous doc reading I know that running a full node typically involves running [nearup].
It appears though that I don't need that now,
so probably for the purposes of developing on near,
`near-cli` will just talk to existing public nodes.

[nearup]: https://github.com/near/nearup


At this point I seem to hit the end of the "basics" documentation flow,
without really accomplishing anything,
but I remember [from the beginning of these docs][docbegin] that "choose a starter project"
is step 2.

[docbegin]: https://docs.near.org/docs/quick-start/new-to-near#how-do-i-get-started

I back up and choose the [`rust-status-message`] starter project.

[`rust-status-message`]: https://examples.near.org/rust-status-message


## Running a contract

The examples say to install `near-shell`,
but that project has been renamed to `near-cli`.
I see that there are already PRs submitted to fix this,
but they are about a month old.
There are a number of relatively old PRs in NEAR repos that haven't been reviewed.

I clone the example:

```
$ git clone https://github.com/near-examples/rust-status-message
$ cd rust-status-message
```

And build per the instructions:

```
$ npm run build
```

This is an npm build that wraps a cargo build.
I'm curious if there's any JavaScript involved,
but I suspect they are just using npm as the primary interface
for the sake of familiarity and for consistency with
the build process for AssemblyScript smart contracts.

Looking at `package.json` it's true that npm isn't doing much here,
but it _is_ adding a post-build step that cargo is incapable of:

```json
  "scripts": {
    "build": "cargo build --target wasm32-unknown-unknown --release",
    "postbuild": "cp target/wasm32-unknown-unknown/release/status_message.wasm ./res/"
  },
```

There is an error in the build:

```
error[E0463]: can't find crate for `core`
  |
  = note: the `wasm32-unknown-unknown` target may not be installed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `byte-tools`.
warning: build failed, waiting for other jobs to finish...
error: build failed
npm ERR! code ELIFECYCLE
npm ERR! errno 101
npm ERR! rust-status-message-builder@1.0.0 build: `cargo build --target wasm32-unknown-unknown --release`
npm ERR! Exit status 101
npm ERR!
npm ERR! Failed at the rust-status-message-builder@1.0.0 build script.
npm ERR! This is probably not a problem with npm. There is likely additional logging output above.
npm WARN Local package.json exists, but node_modules missing, did you mean to install?

npm ERR! A complete log of this run can be found in:
npm ERR!     /home/ubuntu/.npm/_logs/2020-09-06T22_44_16_397Z-debug.log
```

This is an easy error to understand to somebody familiar with embedded Rust development,
but probably not to other newbies.
The toolchain just doesn't have the `wasm32-unknown-unknown` target installed.
I don't know if I missed the documentation that explained this,
but the user experience here could be better.

I add the wasm target:

```
$ rustup target add wasm32-unknown-unknown
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
```

After that the build works:

```
$ npm run build

> rust-status-message-builder@1.0.0 build /home/ubuntu/near/rust-status-message
> cargo build --target wasm32-unknown-unknown --release

   Compiling near-sdk v0.11.0
   Compiling status-message v0.1.0 (/home/ubuntu/near/rust-status-message)
    Finished release [optimized] target(s) in 3.99s

> rust-status-message-builder@1.0.0 postbuild /home/ubuntu/near/rust-status-message
> cp target/wasm32-unknown-unknown/release/status_message.wasm ./res/
```

I try to deploy to the testnet using a temporary developer account:

```
$ near dev-deploy --wasmFile res/status_message.wasm --helperUrl https://near-contract-helper.onrender.com
We would like to collect data on near-cli usage to improve developer experience. We will never send private information. We only collect which commands are run via an anonymous identifier. Would you like to opt in (y/n)?
```

NEAR wants me to opt in to telemetry.
I'm sympathetic,
but since this command presumably will have access to private keys,
I'm not confident that the developers have been suitably careful
about avoiding collection of private data.
For now I say "n", until I can review that code.

The command continues:

```
$ near dev-deploy --wasmFile res/status_message.wasm --helperUrl https://near-contract-helper.onrender.com
We would like to collect data on near-cli usage to improve developer experience. We will never send private information. We only collect which commands are run via an anonymous identifier. Would you like to opt in (y/n)? n
Starting deployment. Account id: dev-1599433413131-7008906, node: https://rpc.testnet.near.org, helper: https://near-contract-helper.onrender.com, file: res/status_message.wasm
Transaction Id 6cT3Su1BTo52i1EwSgaD6Wm9mTvA8238PWQmUFYPkS11
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/6cT3Su1BTo52i1EwSgaD6Wm9mTvA8238PWQmUFYPkS11
Done deploying to dev-1599433413131-7008906
```

`near dev-deploy` has apparently created a script `neardev/dev-account.env`
that will set the `CONTRACT_NAME` environment variable to my
temporary account id, `dev-1599433413131-7008906`.
I call it with `source neardev/dev-account.env`.

I call `near call` to set the status message in the contract:

```
$ near call $CONTRACT_NAME set_status '{"message": "aloha!"}' --accountId $CONTRACT_NAME
Scheduling a call: dev-1599433413131-7008906.set_status({"message": "aloha!"})
Receipt: GcJaodbrMSMAhXCq1qhbsgtkdBUvtPGHa5DL1if3biY5
        Log [dev-1599433413131-7008906]: A
Transaction Id AXsm1VkUPkEFsAHLkhtTpMLi6gmCm9Zq7Xt46N6u1V1u
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/AXsm1VkUPkEFsAHLkhtTpMLi6gmCm9Zq7Xt46N6u1V1u
''
```

I browse to the provided testnet explorer address to see the transaction:

> <https://explorer.testnet.near.org/transactions/AXsm1VkUPkEFsAHLkhtTpMLi6gmCm9Zq7Xt46N6u1V1u>

The transaction is already confirmed by the time I open the block explorer.
That's a nice change [from Ethereum development][tbalog].
I wonder if the mainnet is so fast.

[tbalog]: https://github.com/Aimeedeer/bigannouncement/blob/master/doc/hacklog.md

I note that the explorer is able to show me the contract arguments,

```json
{
  "message": "aloha!"
}
```

which is nice.

And the final step,
I run `near view` to query the contract's message:

```
$ near view $CONTRACT_NAME get_status '{"account_id": "'$CONTRACT_NAME'"}'
View call: dev-1599433413131-7008906.get_status({"account_id": "dev-1599433413131-7008906"})
Log [dev-1599433413131-7008906]: A
'aloha!'
```

Seems everything worked pretty well.

Honestly,
this has been a pretty refreshing experience compared to Ethereum onboarding.
Besides the docs being a bit out of date,
everything worked,
and worked quickly.


## Using a real account

The previous deployment used a temporary dev account.
Per the docs,
next I should use my real account that I created earlier,
`floopy.testnet`.

I login to near cli with `near login`:

```
near login

Please authorize NEAR CLI on at least one of your accounts.

If your browser doesn't automatically open, please visit this URL
https://wallet.testnet.near.org/login/?title=NEAR+CLI&public_key=ed25519%3A8qohFi4YY3uNNbagonWgx7yiHDLod3onmN4NsMt4edXj&success_url=http%3A%2F%2F127.0.0.1%3A5000
Please authorize at least one account at the URL above.

Which account did you authorize for use with NEAR CLI?
Enter it here (if not redirected automatically):
```

So I have to visit a web page to authorize `near-cli` to use my account,
then come back here and enter the account name.

I open the web page and it's a bit confusing. It says

> Unknown App
>
> is requesting full access
> to your account.
>
> This provides access to all of your tokens.
>
> Proceed with caution!
>
> [more information]

Why is it "Unknown App"?

If I click "more information" then I see that the web site does
know that this is for authorizing "NEAR CLI":

> Warning
> 
> This allows access to your entire balance. Please proceed with caution.
>
> This allows NEAR CLI to:
>
> - Create new accounts
> - Transfer tokens from your account to other accounts
> - Deploy smart contracts
> - Call functions on any smart contract
> - Stake and unstake NEAR tokens
> - Create and delete access keys

The web page automatically fills in the name of the testnet account
I made earlier,
`@floopy.testnet`.
I assume it is stored in a cookie.

I am not asked to enter any passwords,
and come to think of it,
I don't have a password or private key that I know of,
just a recovery seed,
so I am curious about the security model.

I run into a problem:
as part of the authentication flow,
the testnet wallet website redirects me to a localhost HTTP address,
presumably hosted by `near login`:

> `http://127.0.0.1:5000/?account_id=floopy.testnet&public_key=ed25519%3A8qohFi4YY3uNNbagonWgx7yiHDLod3onmN4NsMt4edXj&all_keys=ed25519%3A7NCW4ELSHsJE1z4tfAG2d1VPGHRrNPcoLjsugkUDGUyQ`

I am running on an EC2 instance though,
so this address is not accessible to me.

I kill `near login` and look at its command line arguments to see if
I can have it listen on a different address.

```
$ near login --help
near login

logging in through NEAR protocol wallet

Options:
  --help                     Show help  [boolean]
  --version                  Show version number  [boolean]
  --nodeUrl, --node_url      NEAR node URL  [string] [default: "https://rpc.testnet.near.org"]
  --networkId, --network_id  NEAR network ID, allows using different keys based on network  [string] [default: "default"]
  --helperUrl                NEAR contract helper URL  [string]
  --keyPath                  Path to master account key  [string]
  --accountId, --account_id  Unique identifier for the account  [string]
  --useLedgerKey             Use Ledger for signing with given HD key path  [string] [default: "44'/397'/0'/0'/1'"]
  --walletUrl                URL of wallet to use  [string]
  --contractName             Account name of contract  [string]
  --masterAccount            Master account used when creating new accounts  [string]
  --helperAccount            Expected top-level account for a network  [string]
  --verbose, -v              Prints out verbose output  [boolean] [default: false]
```

It's not obvious.
`--walletUrl` looks promising, though.

I try it, and that is not it.
`--walletUrl` is (obviously in hindsight)
the address of `wallet.testnet.near.org`.

I am stumped.

For now I give up on seeing what's at that localhost address,
and hope I can complete authentication without it.
I run `near login` again,
go back through the website authentication flow,
then finally type in `floopy.testnet` at the CLI prompt.

`near login` says

```
Logged in as [ floopy.testnet ] with public key [ ed25519:HjHPUz... ] successfully
```

I guess it worked.

I deploy `status_message.wasm` using my `floopy.testnet` account:

```
$ near deploy --wasmFile res/status_message.wasm --accountId floopy.testnet
Starting deployment. Account id: floopy.testnet, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: res/status_message.wasm
Transaction Id FVyNdAyqj4Tb9EyeCoJyjSDFC4Ebs2T86No6Hm6ZDpg1
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/FVyNdAyqj4Tb9EyeCoJyjSDFC4Ebs2T86No6Hm6ZDpg1
Done deploying to floopy.testnet
```

The transaction is on the block explorer here:

> <https://explorer.testnet.near.org/transactions/FVyNdAyqj4Tb9EyeCoJyjSDFC4Ebs2T86No6Hm6ZDpg1>

And again I call the `set_status` and `get_status` methods:

```
$ near call floopy.testnet set_status '{"message": "aloha friend"}' --accountId floopy.testnet
Scheduling a call: floopy.testnet.set_status({"message": "aloha friend"})
Receipt: BrSeVGXUtgZAj1fAgzBQHBXhZPLq73jV8U5G3Focsq8Q
        Log [floopy.testnet]: A
Transaction Id G9x3T3pkR6fdyi2UHn97E8EU4XQd4qtfZNZtnAvtJgMo
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/G9x3T3pkR6fdyi2UHn97E8EU4XQd4qtfZNZtnAvtJgMo
''

$ near view floopy.testnet get_status '{"account_id": "floopy.testnet"}'
View call: floopy.testnet.get_status({"account_id": "floopy.testnet"})
Log [floopy.testnet]: A
'aloha friend'
```

Everything works as expected.
I feel a momentary lightness,
as if I am having fun.
I dare to think that maybe dapp development can be enjoyable.


## Interacting with the contract from a second account

Per the `rust-status-message` docs
I am going to proceed to set a status message for a second account.
Seems like good NEAR practice to get used to juggling multiple accounts.

I create another account,
this time from the command line.
The `rust-status-message` docs indicate the command for creating a new account is

```
near create_account NEW_ACCOUNT_NAME --masterAccount YOUR_ACCOUNT_NAME
```

It isn't stated explicitly,
but from the `--masterAccount`
switch I take it that this command is for creating _sub-accounts_.
The format of `NEW_ACCOUNT_NAME` is not obvious,
but I will assume it is `subaccount.floopy.testnet`,
fully-qualified,
not just `subaccount`.

I try it:

```
$ near create_account account2.floopy.testnet --masterAccount floopy.testnet
near create_account is deprecated and will be removed in version 0.26.0. Please use near create-account.
Saving key to '/home/ubuntu/.near-credentials/default/account2.floopy.testnet.json'
Account account2.floopy.testnet for network "default" was created.
```

The docs are out of date here too:
the `create_account` command is deprecated,
and should be `create-account`.

I am confused by the text that says the account is for network "default".
What is the "default" network?
I thought I was on the "testnet" network.

Anyway, the command seems to have worked.
I haven't been presented with any secret keys or recovery seed.
I guess that's all derived from the main account.

I try the `set_status` / `get_status` calls with the subaccount:

```
$ near call floopy.testnet set_status '{"message": "bonjour"}' --accountId account2.floopy.testnet
Scheduling a call: floopy.testnet.set_status({"message": "bonjour"})
Receipt: CohthzYQG22mVaLXNTA7s8crt3BQZXFFGNDXnoh8Q1dm
        Log [floopy.testnet]: A
Transaction Id HrVCroubnnX4a9CNbZAYexEGuzWLDWTWinHk3gBQBJ6z
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/HrVCroubnnX4a9CNbZAYexEGuzWLDWTWinHk3gBQBJ6z
''

$ near view floopy.testnet get_status '{"account_id": "account2.floopy.testnet"}'
View call: floopy.testnet.get_status({"account_id": "account2.floopy.testnet"})
Log [floopy.testnet]: A
'bonjour'
```

This time I am calling the contract associated with the main account,
`floopy.testnet`,
as the subaccount,
`account2.floopy.testnet`.
Now the contract contains a status message for both accounts.

I can still retrieve the status of the original:

```
$ near view floopy.testnet get_status '{"account_id": "floopy.testnet"}'
View call: floopy.testnet.get_status({"account_id": "floopy.testnet"})
Log [floopy.testnet]: A
'aloha friend'
```

I am curious what the explorer says about the state
of the contract,
whether it understands it enough to show me
the two stored status messages.

The explorer page for `@floopy.testnet` is

> <https://explorer.testnet.near.org/accounts/floopy.testnet>

While that page shows reasonably-rich information about
transactions against its contract,
it doesn't seem to show anything about the
current state of the contract.
I might expect the data associated with a contract to
be self-describing enough to be displayed in the explorer.


## Running unit tests

The final instructions for the example are for running the contract's unit tests:

```
   Compiling status-message v0.1.0 (/home/ubuntu/near/rust-status-message)
    Finished test [unoptimized + debuginfo] target(s) in 53.40s
     Running target/debug/deps/status_message-95eddd21d3352f72

running 2 tests
test tests::get_nonexistent_message ... ok
test tests::set_get_message ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests status-message

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The `--nocapture` flag lets the Rust test runner print to the console,
which in this case ... does nothing.
I guess the docs are again a bit misleading here.
I wonder if there are any cases in which the NEAR SDK displays
useful information from unit tests.


## A peek at the code

The code for this contract is contained in a single Rust source file:

> [https://github.com/near-examples/rust-status-message/blob/master/src/lib.rs][exsrc]

[exsrc]: https://github.com/near-examples/rust-status-message/blob/master/src/lib.rs

It is surprisingly short.
Small enough to reproduce the whole thing,
sans unit tests,
right here:

```rust
use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    records: UnorderedMap<String, String>,
}

#[near_bindgen]
impl StatusMessage {
    pub fn set_status(&mut self, message: String) {
        env::log(b"A");
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        env::log(b"A");
        return self.records.get(&account_id);
    }
}
```

Not a lot of boilerplate,
despite Rust being a general purpose language,
not a purpose-built smart contract language like Solidity.

Some of the expected boilerplate is hidden behind `#[near_bindgen]` macros.

There's no `main` function.
Whatever mechanism this program uses to launch from NEAR's WASM VM
is hidden behind the `#[near_bindgen]` macros.

Curiously,
there's no indication this is a 'no-std' project,
though it does use its own allocator,
and,
as far as I know,
the standard library does not "just work" on the `wasm32-unknown-unknown` target.

I would expect to see a `#![no_std]` attribute on the crate.
I wonder if the standard library has been automatically pruned by virtue of
not using any of its features.

Oh, `wasm32-unknown-unknown` is probably simply a _no-std target_:
the standard library is never available.
I wonder if this code can possibly compile for any targets other that `wasm32-unknown-unknown`,
not that it needs to.

Wow this code is really succinct.
Except for the custom allocator,
it's pretty much all contract code,
contract state in `struct StatusMessage`,
functions in `impl StatusMessage`,
no boilerplate.

The `borsh` library used here is NEAR's custom serialization library.

I notice the calls to `env::log("A")`.
The logs output here show up in the blockchain explorer,
as seen here:

> <https://explorer.testnet.near.org/transactions/HrVCroubnnX4a9CNbZAYexEGuzWLDWTWinHk3gBQBJ6z>

Pretty neat.

I notice that the contract state is stored in
a custom `UnorderedMap` that is imported from `near_sdk`,
while the docs for this example say that the state is
stored in a `HashMap`.
The docs probably do this for the simplicity of
not explaining the SDK's custom data types,
but it's a bit misleading.
It's probably not too confusing to say something more like,
"status messages are stored in an `UnorderedMap`,
which is similar to the standard Rust `HashMap`".

I note that the value returned from
`env::signer_account_id()` appears to be a `String`,
while I might expect it to be its own type.
Might prevent some errors,
though might make the code less succinct.

NEAR's Rust contracts are unit-testable,
and the SDK comes with a `MockBlockchain`.
That's nice,
since testing on a full devnet can be complex.
I wonder if the unit tests are sufficient that
automated testing on a devnet is unneeded,
or if NEAR provides any help for automated testing on a live devnet.

The unit tests look like

```rust
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }
```

Again this is nice and clean,
but there's something curious here:
the `context` is created and passed to a mysterious `testing_env!` macro,
but then never used again!
What is it doing?
Creating a thread local VM context?
Seems potentially error-prone,
though perhaps understandable considering that in a WASM environment,
the runtime itself is a global resource.


## Changing the contract

I've reached the end of the `rust-status-message` example docs,
but feel I shouldn't stop without doing some programming of my own.
I'm going to make a change to the contract and deploy and test it,
make sure I can write my own NEAR code and run it,
end-to-end.

I add the obvious `remove_status(&mut self)` method.
To do so I need the appropriate method on `UnorderedMap`,
so I look in the [near_sdk API docs][napi].

[napi]: https://docs.rs/near-sdk

The NEAR SDK is reasonably-well documented,
though it could be better.
There are no crate-level docs,
and there are plenty of APIs without any docs.

The method on `UnorderedMap` I need is [`remove`].

[`remove`]: https://docs.rs/near-sdk/2.0.0/near_sdk/collections/struct.UnorderedMap.html#method.remove

So I add this method to `StatusMessage`:

```rust
    pub fn remove_status(&mut self) {
        env::log(b"B");
        let account_id = env::signer_account_id();
        self.records.remove(&account_id);
    }
```

Before I go through the build and deploy steps again,
I wonder if I will be able to deploy a new contract to
`floopy.testnet`,
overwriting the existing contract.

The sequence of commands I issue is:

```
$ npm run build

> rust-status-message-builder@1.0.0 build /home/ubuntu/near/rust-status-message
> cargo build --target wasm32-unknown-unknown --release

   Compiling status-message v0.1.0 (/home/ubuntu/near/rust-status-message)
    Finished release [optimized] target(s) in 1.17s

> rust-status-message-builder@1.0.0 postbuild /home/ubuntu/near/rust-status-message
> cp target/wasm32-unknown-unknown/release/status_message.wasm ./res/

$ near deploy --wasmFile res/status_message.wasm --accountId floopy.testnet
Starting deployment. Account id: floopy.testnet, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: res/status_message.wasm
Transaction Id 4nQUUzF5XqLb7qqim912yUQ5EepJXAA9NcyinS9eq6LT
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/4nQUUzF5XqLb7qqim912yUQ5EepJXAA9NcyinS9eq6LT
Done deploying to floopy.testnet
```

Seems to deploy successfully.
I learn that an account's contract can be overwritten.

With a new contract,
I expect the state will be cleared,
and that if I call `get_status` I will get an empty result:

```
$ near view floopy.testnet get_status '{"account_id": "floopy.testnet"}'
View call: floopy.testnet.get_status({"account_id": "floopy.testnet"})
Log [floopy.testnet]: A
'aloha friend'
```

OK, that's interesting.

I (think) I have updated the contract for `floopy.testnet`,
but the previous contract's state is still there,
and is still compatible with the new contract.

Can I call my new `remove_status` method?

```
$ near call floopy.testnet remove_status --accountId floopy.testnet
Scheduling a call: floopy.testnet.remove_status()
Receipt: BheNnAiuvX69YKpCGvML36FRCHKJCLXZcLvkRQJLCpyM
        Failure [floopy.testnet]: Error: Contract method is not found
An error occured
MethodNotFound [Error]: Contract method is not found
    at Object.parseRpcError (/home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/node_modules/near-api-js/lib/utils/rpc_errors.js:38:19)
    at Account.signAndSendTransaction (/home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/node_modules/near-api-js/lib/account.js:139:36)
    at processTicksAndRejections (internal/process/task_queues.js:97:5)
    at async scheduleFunctionCall (/home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/commands/call.js:38:34)
    at async Object.handler (/home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/utils/exit-on-error.js:39:9) {
  [stack]: 'Error: Contract method is not found\n' +
    '    at Object.parseRpcError (/home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/node_modules/near-api-js/lib/utils/rpc_errors.js:38:19)\n' +
    '    at Account.signAndSendTransaction (/home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/node_modules/near-api-js/lib/account.js:139:36)\n' +
    '    at processTicksAndRejections (internal/process/task_queues.js:97:5)\n' +
    '    at async scheduleFunctionCall (/home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/commands/call.js:38:34)\n' +
    '    at async Object.handler (/home/ubuntu/.nvm/versions/node/v12.16.2/lib/node_modules/near-cli/utils/exit-on-error.js:39:9)',
  [message]: 'Contract method is not found',
  type: 'MethodNotFound',
  context: undefined,
  index: 0
}
```

No.

It _appears_ that I uploaded a new contract with a `remove_status` method,
but that I actually did not,
and that the old contract is still in place.

Something is wrong.

To double-check,
I go through the build-deploy process again,
and this time I get a different result.

This time,
after delpoying the new contract,
I see that I no longer have a status message:

```
$ near view floopy.testnet get_status '{"account_id": "floopy.testnet"}'
View call: floopy.testnet.get_status({"account_id": "floopy.testnet"})
Log [floopy.testnet]: A
null
```

And I can set a new status message:

```
$ near call floopy.testnet set_status '{"message": "woah"}' --accountId floopy.testnet
Scheduling a call: floopy.testnet.set_status({"message": "woah"})
Receipt: E6YTPZuH4zPMCxduvQMEmTMgDyCCaznyb48PEZZWNnNe
        Log [floopy.testnet]: A
Transaction Id BD8ZKCXjSsjYNfwLFKmYWquSPpGqYtSUJYVpjGzk25me
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/BD8ZKCXjSsjYNfwLFKmYWquSPpGqYtSUJYVpjGzk25me
''

$ near view floopy.testnet get_status '{"account_id": "floopy.testnet"}'
View call: floopy.testnet.get_status({"account_id": "floopy.testnet"})
Log [floopy.testnet]: A
'woah'
```

And finally call my new `remove_status` to remove it again:

```
$ near call floopy.testnet remove_status --accountId floopy.testnet
Scheduling a call: floopy.testnet.remove_status()
Receipt: 8x1s7C3gJAanboJofsx6QDR2YUXx3eNxXa6w6AokTzwf
        Log [floopy.testnet]: B
Transaction Id 2ysXxvC3WnRu9zbxJUw4TeJtowa8Q8YJyZJqwXfzY89K
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/2ysXxvC3WnRu9zbxJUw4TeJtowa8Q8YJyZJqwXfzY89K
''

$ near view floopy.testnet get_status '{"account_id": "floopy.testnet"}'
View call: floopy.testnet.get_status({"account_id": "floopy.testnet"})
Log [floopy.testnet]: A
null
```

I'm confused about the inconsistent behavior.

I experiment some more,
go through the build deploy cycle,
with manual testing,
numerous times.
I can't reproduce my original results,
and have to assume it's a problem with me and not NEAR.

Subsequent tests exhibit the following behavior:

- The contract can be overwritten.
- Overwriting the contract preserves the previous contract state.

Well, this is a confusing end to an otherwise painless intro to NEAR programming.
Presumably things will become clearer with more experience.



## Concluding thoughts

Although there are quite a few blockchains that support Rust smart contracts,
I originally picked NEAR for this experiment after noticing
that their documentation was surprisingly clear,
in contrast to my experiences with other blockchains.

They've put a strong emphasis on the developer experience,
and in my limited time here it shows.

When creating a platform for others to build off of,
as programmable smart-contract blockchains are,
it's pretty important to make the developer experience as inviting as possible,
and I think that is probably especially true in the blockchain space,
where there is so much competition that it's impossible to keep track of the options.

A good developer experience is important because
the developers are your first adopters,
the "thought leaders" who will advocate the product to the non-technical.
If not a guarantee of success,
at least a good developer experience can be the feature to set a product apart.

And so far I think that NEAR has a surprisingly good developer experience.


## Appendix: bugs encountered

These are the issues I ran into during this experience.
Team members might want to address these.
Yeah, I should probably fix or file these,
but hopefully at least listing them here is a useful gesture.

- Doc references to `near-shell`. Should be `near-cli`.
  - existing PRs:
    - https://github.com/near-examples/rust-status-message/pull/40
    - https://github.com/near-examples/rust-status-message/pull/41
- Missing instructions for `rustup target install wasm32-unknown-unknown`.
- Incorrect `near create_account` command. Should be `create-account`.
- Unclear docs about "default" network. What is it?
- Incorrect instructions to use `--nocapture`. It does nothing here.
  Does it ever do anything with NEAR unit tests?
- Docs say state uses a `HashMap` but they use a custom `UnorderedMap`.
- `near login` leads to a webpage that is authorizing an "Unknown App".
  Should be "NEAR CLI".