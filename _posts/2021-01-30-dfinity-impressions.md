---
layout: post
title: First impressions of programming on DFINITY
tags: [rust, blockchain]
---


Continuing our adventure exploring programmable Rust blockchains,
this time [Aimee] and I are going to dive into [DFINITY].

[Aimee]: https://github.com/Aimeedeer/
[DFINITY]: https://github.com/dfinity

I have been aware of Dfinity for a while,
but have not looked at it closely.
I know that they are building a programmable blockchain,
and that they have their own smart contract programming language.

These experience reports tend to be so detailed that they are
mostly useful to the product developers,
and not really worth reading for anybody else.
For other audences,
our [closing thoughts][clt] may be worthwhile.

[clt]: #closing-thoughts

- [What we're going to do](#what-were-going-to-do)
- [Starting by doing some research](#starting-by-doing-some-research)
- [Installing the tools](#installing-the-tools)
- [Aimee upgrades her `dfx`](#aimee-upgrades-her-dfx)
- [Creating my own project](#creating-my-own-project)
- [Running a local test node](#running-a-local-test-node)
- [Deploying and running the contract](#deploying-and-running-the-contract)
- [Aimee resumes her tutorial](#aimee-resumes-her-tutorial)
- [Deploying to the live network](#deploying-to-the-live-network)
- [Writing our own contract](#writing-our-own-contract)
- [Writing a contract in Rust?](#writing-a-contract-in-rust)
- [Successfully completing a tutorial](#successfully-completing-a-tutorial)
- [Closing thoughts](#closing-thoughts)


## What we're going to do

For this first look we're going to install the tools,
walk through the tutorials,
and then try to implement a contract
we've implemented before in solidity, near, and ink &mdash; [the big announcement][tba].
It is a simple program that lets the caller bid to set a singleton string message.

[tba]: https://github.com/Aimeedeer/bigannouncement/


## Starting by doing some research

Myself,
when I start a project,
I usually just jump in,
without informing myself of anything at all really.
Aimee though likes to read,
and watch,
and this time we have watched [an entire YouTube playlist][ytpl]
about "building on the Internet Computer".

[ytpl]: https://www.youtube.com/playlist?list=PLuhDt1vhGcrejCmYeB1uqgl9Y3f6MCyFp

It's a basic introductory series,
but I did have some takeaways:
mostly,
that programming for Dfinity appears to look a lot like traditional programming,
and _not_ like Solidity-descendant smart contract programming.
Gas was not mentioned at all,
and I am curious whether that means the programmer doesn't neeed to worry about gas,
or whether they chose to gloss over that subject.
They are using a custom smart contract language, [Motoko].
It looks like a mashup of a number of languages,
with some clear Rust influence,
though not one single obvious inspiration.
It is modeling its contract interactions as actors,
using the async/await model,
which so far appears to be a good fit.
The code I've seen in the video reads pretty easily.

[Motoko]: https://sdk.dfinity.org/docs/language-guide/motoko.html

I read a blog post by Dfinity's Johan Granström:
[A Closer Look at Software Canisters, an Evolution of Smart Contracts][canblog].

[canblog]: https://medium.com/dfinity/software-canisters-an-evolution-of-smart-contracts-internet-computer-f1f92f1bfffb

Many of the capabilities described here sound similar to other smart contract platforms.
A few that stand out to me though:

- _The memory space of a wasm canister is saved and restored every execution_!
  This should makes it behave as if it were a long-running process,
  even though each invocation may be years apart,
  and on different nodes.
  There is apparently no explicit storage.
  This is _cool_,
  and I am surprised I haven't seen this done before in the smart contract space.
  It implies though that memory leaks live forever,
  and so does memory fragmentation.

- Full nodes are run by data centers.
  This probably allows it to be fast and store a lot of data,
  but reduces the decentralization.
  This is probably the fate of all smart contract blockchains though &mdash;
  Ethereum is already too big for most people to run on their own.
  It's not clear if full nodes need permission to join the network.

- Still no mention of gas.

I begin to read another post by Dfinity's Dominic Williams,
[Announcing the Internet Computer "Mainnet" and a 20-Year Roadmap][twenty].
This is a good "vision statement" about what Dfinity is aiming for,
but it's verbose,
and without technical details.
Definitely worth a read for anybody interested in Dfinity.
I did not finish it.

[twenty]: https://medium.com/dfinity/announcing-internet-computer-mainnet-and-a-20-year-roadmap-790e56cbe04a







## Installing the tools

Following the [quick start docs][qsd].

[qsd]: https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html

I'm doing [local development][ldev],
not connecting to any testnet.

[ldev]: https://sdk.dfinity.org/docs/quickstart/local-quickstart.html

The tools are installed via `ssh`,
which is common and I am fine with.

I download [the script][script] and give it a quick review,
to make sure it's not doing anything obviously wrong.

[script]: https://sdk.dfinity.org/install.sh

I am tickled to see this comment in the headers:

```
# Borrowed from rustup (https://sh.rustup.rs)
```

as _I wrote_ most of rustup's install script.
I also know smart people have carefully improved it over time &mdash;
it is a very battle-tested piece of tricky shell script.
That gives me some immediate confidence in this script.
There is though a lot of Dfinity-specific code here,
and it is not as lovingly-maintained as rustup's,
with bizarre spacing and stripped comments
(it looks like it has been machine-processed).
Important functionality like
architecture detection,
platform-specific quirk handling,
and the security-sensitive curl configuration
are borrowed straight from rustup though.

I don't see anything obviously incorrect in the script,
though its poor readability gives me pause &mdash;
this is a script some hackers will definitely read before running,
and it needs to give them confidence.

I run the installation and see:

```
$ sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
Executing DFINITY SDK install script, commit: 55f1bbedee393411e1ae3a6eaeb449a6dd047c00
The DFINITY Canister SDK
Copyright 2021 DFINITY Stiftung. All Rights Reserved.
The DFINITY Canister SDK (the "Software") is licensed under the Alpha DFINITY
Canister SDK License Agreement (the "License"). You may not use the Software
except in compliance with the License. You may obtain a copy of the License at
    https://sdk.dfinity.org/sdk-license-agreement.txt
The Software is provided to you AS IS and WITHOUT WARRANTY.
Do you agree and wish to install the DFINITY Canister SDK [y/N]?
```

Huh.
They have a non-standard license.
I read it.

It doesn't load as text in my browser,
so it might not in others.
Here's a gist:

> [https://gist.github.com/brson/7abc2be6f9d8e2daf488512af2a866b7](https://gist.github.com/brson/7abc2be6f9d8e2daf488512af2a866b7)

It's not a free-software license.

It is called the "Alpha DFINITY ..." license,
so I am assuming this is a temporary license,
and it will change to open source in the future.
The terms in it are ominous enough that,
if I weren't here to blog about it,
I would stop immediately.
I scan it to determine whether there are any restrictions
on what I am intending to publish in this blog,
and I think the answer is "no".

There's another issue here &mdash;
the installer's text claims

> "The DFINITY Canister SDK is licensed under the Alpha dfinity
  Canister SDK License Agreement"

But also, the source for at least _part_ of the Canister SDK
lives [on GitHub][cdk-rs],
and claims to be Apache-2.0 licensed.

[cdk-rs]: https://github.com/dfinity/cdk-rs

Just to make sure it _is_ possible to opt out at this
stage of the script,
I hit _enter_,
which should default to not accepting the license
(that is what the "y/N" convention means &mdash;
"N" is the default).

It says the following and exits:

```
Please accept the license to continue.
```

Great.

Now I run it again and enter "y".

The install script shows the following and exits:

```
Version found: 0.6.20
Creating uninstall script in ~/.cache/dfinity
uninstall path=/home/ubuntu/.cache/dfinity/uninstall.sh
Checking for latest release...
Will install in: /home/ubuntu/bin
Installed /home/ubuntu/bin/dfx
```

This looks pretty standard.

It's curious that it first said the version it found,
then later said it was checking for the latest release.
Didn't it already do that in order to find the version?

I read the uninstall script and I don't see any
obviously catastrophic bugs.
It does mention a `DFX_INSTALLATION_ROOT` environment
variable that I don't yet know anything about.

I run the uninstall script.
It works,
though provides no feedback.

I install again.

`dfx` is not immediatly in my `PATH`.
The install script or the docs could do a bit better
in guiding the user here.
I know from experience that this first
step of getting `PATH` set correctly is a very common
blocker for newbies.
Unfortunately there are no perfect technical solutions
to setting up `PATH` across all platforms,
so informing the user about it has to be part of the install
experience.

I am on Linux.
On Aimee's Mac though `dfx` is immediately on the path.
This is because the installer installed directly to
`/usr/local/bin`.
This creeps me out a bit
as I am accustomed to needing to `sudo` to write to that location.
Maybe it's common on Macs to install directly to `/usr/local/bin`
without permission.
On Linux, my `dfx` is in `~/bin`.

While experimenting with the install script I hit a particularly
strange bug:

When prompted to accept the license,
if I hit CTRL-C to quit,
I am _not_ dropped back to my own shell;
instead I am dropped into _the shell
the install instructions asked me to run
as part of downloading and running the script_.

In other words,
in the command

```
$ sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

That initial `sh` command is still running after I kill the script.

This confuses me a lot for a bit.

I poke at the script for a bit to see if I can fix the problem,
and one way to do it is with the `trap` shell command:

```sh
trap exit 2
```

Though it should probably do more cleanup than just `exit`.
I note that rustup does not do this though,
and I am not sure offhand why rustup's handling of CTRL-C
works correctly where Dfinity's does not.

The next step is to install a VSCode plugin.
I don't use VSCode,
so I skip it.



## Aimee upgrades her `dfx`

Aimee has previously installed `dfx`.
Today when she ran `dfx new firsttest` to create a Dfinity project,
she had a confusing experience.

This is what she saw:

```
$ dfx new firsttest

The Dfinity Canister SDK sends anonymous usage data to Dfinity Stiftung by
default. If you wish to disable this behavior, then please set the environment
variable DFX_TELEMETRY_DISABLED=1. Learn more at https://sdk.dfinity.org.

Fetching manifest https://sdk.dfinity.org/manifest.json
⠋ Checking for latest dfx version...
You seem to be running an outdated version of dfx.

You are strongly encouraged to upgrade by running 'dfx upgrade'!
  Version v0.6.20 installed successfully.
Creating new project "firsttest"...
CREATE       firsttest/src/firsttest_assets/assets/sample-asset.txt (24B)...
CREATE       firsttest/src/firsttest/main.mo (107B)...
CREATE       firsttest/dfx.json (484B)...
CREATE       firsttest/.gitignore (165B)...
⠐ Checking for latest dfx version...
CREATE       firsttest/src/firsttest_assets/public/index.js (149B)...
CREATE       firsttest/package.json (288B)...
CREATE       firsttest/webpack.config.js (2.15KB)...
⠉ Checking for latest dfx version...
⠒ Checking for latest dfx version...
⠠ Installing node dependencies...
⠤ Checking for latest dfx version...
⠖ Installing node dependencies...

> fsevents@1.2.13 install /Users/aimeez/github/dfinity-project/firsttest/node_modules/watchpack-chokidar2/node_modules/fsevents
> node install.js
⠤ Checking for latest dfx version...
⠖ Checking for latest dfx version...
⠒ Checking for latest dfx version...
⠁ Checking for latest dfx version...
npm WARN firsttest_assets@0.1.0 No repository field.
npm WARN firsttest_assets@0.1.0 No license field.

⠙ Checking for latest dfx version...

13 packages are looking for funding
  run `npm fund` for details

found 1 high severity vulnerability
⠒ Checking for latest dfx version...
⠂ Checking for latest dfx version...

===============================================================================
        Welcome to the internet computer developer community!
                        You're using dfx 0.6.20

            ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄                ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄       
          ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄          ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄    
        ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄      ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  
       ▄▄▄▄▄▄▄▄▄▄▀▀▀▀▀▄▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄▄▀▀▀▀▀▀▄▄▄▄▄▄▄▄▄▄ 
      ▄▄▄▄▄▄▄▄▀         ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀         ▀▄▄▄▄▄▄▄▄▄
     ▄▄▄▄▄▄▄▄▀            ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀             ▄▄▄▄▄▄▄▄
     ▄▄▄▄▄▄▄▄               ▀▄▄▄▄▄▄▄▄▄▄▄▄▀                ▄▄▄▄▄▄▄
     ▄▄▄▄▄▄▄▄                ▄▄▄▄▄▄▄▄▄▄▄▄                 ▄▄▄▄▄▄▄
     ▄▄▄▄▄▄▄▄               ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄              ▄▄▄▄▄▄▄▄
      ▄▄▄▄▄▄▄▄           ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄          ▄▄▄▄▄▄▄▄▀
      ▀▄▄▄▄▄▄▄▄▄▄     ▄▄▄▄▄▄▄▄▄▄▄▄▀ ▀▄▄▄▄▄▄▄▄▄▄▄▄    ▄▄▄▄▄▄▄▄▄▄▄ 
       ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀     ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀  
         ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀         ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄    
           ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀▀             ▀▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀      
              ▀▀▀▀▀▀▀▀▀▀▀                    ▀▀▀▀▀▀▀▀▀▀▀         
     


To learn more before you start coding, see the documentation available online:

- Quick Start: https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html
- SDK Developer Tools: https://sdk.dfinity.org/docs/developers-guide/sdk-guide.html
- Motoko Language Guide: https://sdk.dfinity.org/docs/language-guide/motoko.html
- Motoko Quick Reference: https://sdk.dfinity.org/docs/language-guide/language-manual.html

If you want to work on programs right away, try the following commands to get started:

    cd firsttest
    dfx help
    dfx new --help

===============================================================================
```

This is a mess.

Several things appear to be going wrong here.

- The command has printed "you are strongly encouraged to upgrade by running 'dfx upgrade',
  then it goes on to just do the upgrade on its own, saying
  "Version v0.6.20 installed successfully".

- Ever after checking for and installing a `dfx` upgrade,
  some process continues "Checking for latest dfx version".

- The `dfx` version check is happening _in parallel_
  to the creation and build of the "firsttest" project,
  causing a confusing interleaving of messages.

- Even when not interrupted by other messages,
  the "Checking for latest dfx version" message,
  which is accompanied by a unicode spinner,
  and should only appear once while the spinner changes,
  is printed repeatedly.

- The "found 1 high severity vulnerability" message
  gives the impression that this code is not maintained.

After some careful reading we think we understand everything that happened.

We note that Aimee _already had_ version `dfx` 0.6.20 installed,
but that `dfx` said it updated to 0.6.20.
We don't know if anything changed about `dfx` or if it just got
confused and reinstalled itself.

We try the suggested `dfx upgrade`,
and as we expected it does nothing.
So `dfx new firsttest` suggested we run a command,
then ran that command for us,
and when we ran the suggested command,
it did nothing.

Now we are curious about how the output of `dfx new firsttest`
will change if we run it again,
without requiring an update,
so we delete the new "firsttest" directory and try again:

```
$ dfx new firsttest
Fetching manifest https://sdk.dfinity.org/manifest.json
Creating new project "firsttest"...
CREATE       firsttest/src/firsttest_assets/assets/sample-asset.txt (24B)...
CREATE       firsttest/src/firsttest/main.mo (107B)...
CREATE       firsttest/dfx.json (484B)...
CREATE       firsttest/.gitignore (165B)...
CREATE       firsttest/README.md (1.16KB)...
CREATE       firsttest/src/firsttest_assets/public/index.js (149B)...
CREATE       firsttest/package.json (288B)...
CREATE       firsttest/webpack.config.js (2.15KB)...
⠒ Installing node dependencies...
⠁ Installing node dependencies...
npm WARN deprecated resolve-url@0.2.1: https://github.com/lydell/resolve-url#deprecated
⠁ Installing node dependencies...
⠋ Installing node dependencies...

> fsevents@1.2.13 install /Users/aimeez/github/dfinity-project/firsttest/node_modules/watchpack-chokidar2/node_modules/fsevents
> node install.js
⠒ Installing node dependencies...
⠲ Installing node dependencies...
⠉ Installing node dependencies...
⠲ Installing node dependencies...
npm WARN firsttest_assets@0.1.0 No repository field.
npm WARN firsttest_assets@0.1.0 No license field.

⠄ Installing node dependencies...

13 packages are looking for funding
  run `npm fund` for details

found 1 high severity vulnerability
⠄ Installing node dependencies...


   ╭─────────────────────────────────────────────────────────────────╮
   │                                                                 │
   │      New patch version of npm available! 6.14.8 → 6.14.11       │
   │   Changelog: https://github.com/npm/cli/releases/tag/v6.14.11   │
   │                Run npm install -g npm to update!                │
   │                                                                 │
   ╰─────────────────────────────────────────────────────────────────╯
  Done.
Creating git repository...

===============================================================================
        Welcome to the internet computer developer community!
                        You're using dfx 0.6.20

            ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄                ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄       
          ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄          ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄    
        ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄      ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  
       ▄▄▄▄▄▄▄▄▄▄▀▀▀▀▀▄▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄▄▀▀▀▀▀▀▄▄▄▄▄▄▄▄▄▄ 
      ▄▄▄▄▄▄▄▄▀         ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀         ▀▄▄▄▄▄▄▄▄▄
     ▄▄▄▄▄▄▄▄▀            ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀             ▄▄▄▄▄▄▄▄
     ▄▄▄▄▄▄▄▄               ▀▄▄▄▄▄▄▄▄▄▄▄▄▀                ▄▄▄▄▄▄▄
     ▄▄▄▄▄▄▄▄                ▄▄▄▄▄▄▄▄▄▄▄▄                 ▄▄▄▄▄▄▄
     ▄▄▄▄▄▄▄▄               ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄              ▄▄▄▄▄▄▄▄
      ▄▄▄▄▄▄▄▄           ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄          ▄▄▄▄▄▄▄▄▀
      ▀▄▄▄▄▄▄▄▄▄▄     ▄▄▄▄▄▄▄▄▄▄▄▄▀ ▀▄▄▄▄▄▄▄▄▄▄▄▄    ▄▄▄▄▄▄▄▄▄▄▄ 
       ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀     ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀  
         ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀         ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄    
           ▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀▀             ▀▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀      
              ▀▀▀▀▀▀▀▀▀▀▀                    ▀▀▀▀▀▀▀▀▀▀▀         
     


To learn more before you start coding, see the documentation available online:

- Quick Start: https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html
- SDK Developer Tools: https://sdk.dfinity.org/docs/developers-guide/sdk-guide.html
- Motoko Language Guide: https://sdk.dfinity.org/docs/language-guide/motoko.html
- Motoko Quick Reference: https://sdk.dfinity.org/docs/language-guide/language-manual.html

If you want to work on programs right away, try the following commands to get started:

    cd firsttest
    dfx help
    dfx new --help

===============================================================================
```

So nothing about checking for updates this time.
Good.
It still has a problem with its spinner+status message output,
where "Installing node dependencies..." is printed repeatedly.

This is on a Mac,
so perhaps it's a platform-specific bug.

The final thing that is striking about this command
is that it prints a massive welcome greeting:
huge logo, links to docs, basic commands.

This is perfectly fine for something on first run,
but I don't want to see this again.
It's obvious though that this is not something that happens only on first run,
since we've seen this message multiple times.

We notice one other problem with the banner:
it is not colored correctly on this terminal.
Here's a screenshot:

<img class="blog-photo" src="/images/dfx-new-sth.png"/>

I've seen this banner in a 256 color terminal
and it is properly colored with a gradient:

<img class="blog-photo" src="/images/dfx-correct-black.png"/>

That's actually really cool,
but I still don't want to see it more than once.
Show this after installing `dfx` and be done with it.

I wonder if `dfx` is one of the tools with code available
under the GitHub org,
whether I can look at the code and try to fix some of these issues.

The only repo I see that might contain `dfx` is [cdk-rs].
I clone it and ripgrep for "dfx".

`dfx` is used by some test scripts here,
but I don't think the code for it is here.

[cdk-rs]: https://github.com/dfinity/cdk-rs





## Creating my own project

Back to my own installation,
the next quick start step is to [create a project][cap].

[cap]: https://sdk.dfinity.org/docs/quickstart/local-quickstart.html#create-a-new-project

I run `dfx new hello` and see similar output to Aimee's.
I'm on Linux and I too see the duplicate status messages with spinners:

```
⠴ Installing node dependencies...
⠠ Installing node dependencies...
npm WARN deprecated resolve-url@0.2.1: https://github.com/lydell/resolve-url#deprecated
⠦ Installing node dependencies...
⠐ Installing node dependencies...
npm WARN optional SKIPPING OPTIONAL DEPENDENCY: fsevents@~2.3.1 (node_modules/chokidar/node_modules/fsevents):
```

Maybe node is printing empty lines here and that's causing the spinner to be interrupted,
or maybe it's somehow just from node printing a line.

Let's look at what `dfx` has created on disk...

```
$ tree hello -I node_modules
hello
├── README.md
├── dfx.json
├── package-lock.json
├── package.json
├── src
│   ├── hello
│   │   └── main.mo
│   └── hello_assets
│       ├── assets
│       │   └── sample-asset.txt
│       └── public
│           └── index.js
└── webpack.config.js

5 directories, 8 files
```

It's an npm / webpack project.
`hello` is the contract,
containing a Motoko script,
and `hello_assets` is the web app.

`main.mo` contains:

```
actor {
    public func greet(name : Text) : async Text {
        return "Hello, " # name # "!";
    };
};
```

Easy enough.

There's a special `dfx.json` configuration file here,
used by `dfx`:

```json
{
  "canisters": {
    "hello": {
      "main": "src/hello/main.mo",
      "type": "motoko"
    },
    "hello_assets": {
      "dependencies": [
        "hello"
      ],
      "frontend": {
        "entrypoint": "src/hello_assets/public/index.js"
      },
      "source": [
        "src/hello_assets/assets",
        "dist/hello_assets/"
      ],
      "type": "assets"
    }
  },
  "defaults": {
    "build": {
      "packtool": ""
    }
  },
  "dfx": "0.6.20",
  "networks": {
    "local": {
      "bind": "127.0.0.1:8000",
      "type": "ephemeral"
    }
  },
  "version": 1
}
```

Pretty simple.




## Running a local test node

The next step in the "local quickstart"
is to [start the local network][stln].

[stln]: https://sdk.dfinity.org/docs/quickstart/local-quickstart.html#start-the-local-network

The instructions here are oddly rudimentary:

> "For example, you can do either of the following if running Terminal on macOS:

> Click Shell, then select New Tab to open a new terminal in your current
  working directory.

> Click Shell and select New Window, then run cd ~/ic-projects/hello in the new
  terminal if your hello project is in the ic-projects working folder."

This is how to open a new terminal tab or window,
but only on macOS.
And previously the instructions explained how to open a terminal,
but only on macOS:

> "For example, open Applications, Utilities, then double-click Terminal or press
  ⌘+spacebar to open Search, then type terminal."

I do appreciate accounting for all questions a newbie might have,
but this is assuming that the target audience is somebody who wants to program
for the blockchain,
but doesn't know how to open a terminal.
I think most development tutorials will just list "know how to open a termal"
as a prerequisit,
and maybe link to outside documentation.
It's not really the role of the docs here to explain how to open applications on a mac.

There may be some assumption here that Linux users know more about how to operate a terminal
than macOS users,
which could lead to the instructions glossing over how to configure `PATH`
(which just works on macOS),
while also including very detailed instructions for operating the terminal application.
Or maybe the assumption is just that more developers are doing their local hacking
on macOS.

Or maybe the writer was just using a Mac.

Anyway, in this text:

> "Click Shell, then select New Tab to open a new terminal in your current
  working directory.

> Click Shell and select New Window, then run cd ~/ic-projects/hello in the new
  terminal if your hello project is in the ic-projects working folder."

This is presenting two alternatives,
but only one includes instructions for changing directories to the project folder;
and this is the first mention of `~/ic-projects` &mdash;
the `dfx new hello` instructions didn't say anything about it.

I don't notice the instruction that `dfx start` needs to be run inside
the project folder,
probably because that instruction is buried inside the instruction for how
operate the macOS terminal.

So I run `dfx start` in the wrong folder:

```
dfx start
Cannot find dfx configuration file in the current working directory. Did you forget to create one?
```

`dfx` speculates that I forgot to create a "dfx configuration file",
which I have already guessed is `dfx.json`.
Its speculation is incorrect.
I created it, indirectly;
I'm just not in the right folder.

Is forgetting to create a `dfx.json` file the most common explanation for this error?

Suggestions from developer tools can be awesome when they are 100% right.
They can be infuriating when they are wrong.

(I enter a momentary reverie about the times `rustc` or `clippy` have made wrong suggestions).

Here's the comparable `cargo` error:

```
error: could not find `Cargo.toml` in `/home/ubuntu/dfinity/hello` or any parent directory
```

I am given the filename it's looking for,
and no attempt to guess why it's missing.
I also get the directory it's looking in,
which on consideration I think is pretty thoughtful &mdash;
I have on multiple occassions seen less experienced developers
take a surprisingly long time after these types of errors to check what directory
they are actually in,
and this puts that information right in front of them.

I run `dfx start` in the correct folder:

```
$ dfx start
Jan 24 18:15:14.129 INFO ic-starter. Configuration: ValidatedConfig { replica_path: Some("/home/ubuntu/.cache/dfinity/versions/0.6.20/replica"), replica_version: "0.1.0", log_level: Warning, subnet_id: fscpm-uiaaa-aaaaa-aaaap-yai, cargo_bin: "cargo", cargo_opts: "", state_dir: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state", http_listen_addr: V4(127.0.0.1:0), http_port_file: Some("/home/ubuntu/dfinity/hello/.dfx/replica-configuration/replica-1.port"), metrics_addr: None, hypervisor_create_funds_whitelist: "*", artifact_pool_dir: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/ic_consensus_pool", crypto_root: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/crypto", state_manager_root: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/state", registry_file: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/registry.proto", bootstrap_registry: None, state_dir_holder: None }, Application: starter
Jan 24 18:15:14.129 INFO Initialize replica configuration "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/ic.json5", Application: starter
Jan 24 18:15:14.536 ERRO s:fscpm-uiaaa-aaaaa-aaaap-yai/n:eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe/ic_messaging/xnet_endpoint No XNet configuration for node eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe. This is an error in production, but may be ignored in single-subnet test deployments.
Jan 24 18:15:15.537 WARN s:fscpm-uiaaa-aaaaa-aaaap-yai/n:eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe/ic_http_handler/ic_http_handler NNS subnet not found in network topology. Skipping fetching the delegation.
Starting webserver on port 42067 for replica at "http://localhost:42067"
binding to: V4(127.0.0.1:8000)
replica(s): http://localhost:42067/
Jan 24 18:17:52.946 WARN s:fscpm-uiaaa-aaaaa-aaaap-yai/n:eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe/ic_state_layout/utils StateManager runs on a filesystem not supporting reflinks (attempted to reflink /home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/state/tip/subnet_queues.pbuf => /home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/state/fs_tmp/scratchpad_0000000000000064/subnet_queues.pbuf), running big canisters can be very slow
```

Here's what it looks like wrapped in my terminal:

```
$ dfx start
Jan 24 18:15:14.129 INFO ic-starter. Configuration: ValidatedConfig { replica_path: Some("/home/ubuntu/.cache/dfinity/versions/0.6.20/
replica"), replica_version: "0.1.0", log_level: Warning, subnet_id: fscpm-uiaaa-aaaaa-aaaap-yai, cargo_bin: "cargo", cargo_opts: "", s
tate_dir: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state", http_listen_addr: V4(127.0.0.1:0), http_port_file: Some("/home/ubu
ntu/dfinity/hello/.dfx/replica-configuration/replica-1.port"), metrics_addr: None, hypervisor_create_funds_whitelist: "*", artifact_po
ol_dir: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/ic_consensus_pool", crypto_root: "/home/ubuntu/dfinity/hello/
.dfx/state/replicated_state/node-100/crypto", state_manager_root: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/sta
te", registry_file: "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/registry.proto", bootstrap_registry: None, state_dir_holde
r: None }, Application: starter
Jan 24 18:15:14.129 INFO Initialize replica configuration "/home/ubuntu/dfinity/hello/.dfx/state/replicated_state/ic.json5", Applicati
on: starter
Jan 24 18:15:14.536 ERRO s:fscpm-uiaaa-aaaaa-aaaap-yai/n:eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe/ic_messaging/
xnet_endpoint No XNet configuration for node eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe. This is an error in prod
uction, but may be ignored in single-subnet test deployments.
Jan 24 18:15:15.537 WARN s:fscpm-uiaaa-aaaaa-aaaap-yai/n:eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe/ic_http_handl
er/ic_http_handler NNS subnet not found in network topology. Skipping fetching the delegation.
Starting webserver on port 42067 for replica at "http://localhost:42067"
binding to: V4(127.0.0.1:8000)
replica(s): http://localhost:42067/
Jan 24 18:17:52.946 WARN s:fscpm-uiaaa-aaaaa-aaaap-yai/n:eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe/ic_state_layo
ut/utils StateManager runs on a filesystem not supporting reflinks (attempted to reflink /home/ubuntu/dfinity/hello/.dfx/state/replica
ted_state/node-100/state/tip/subnet_queues.pbuf => /home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/state/fs_tmp/scratc
hpad_0000000000000064/subnet_queues.pbuf), running big canisters can be very slow
```

This is a mouthful.

Frankly, it looks like junk:
debug printing,
huge wrapped lines,
extremely long identifiers,
inconsistent formatting,
inconsistent language.
It's unscannable spew.

Some of the lines here are not log lines,
but are just output straight to the console,
and have a different format.

There's an error here that explains that it is actually not an error
("... may be ignored in single-subnet test deployments").

Here's what a substrate node looks like when it launches:

```
2020-11-25 00:05:57  Running in --dev mode, RPC CORS has been disabled.
2020-11-25 00:05:57  Canvas Node
2020-11-25 00:05:57  ✌️  version 0.1.0-e189090-x86_64-linux-gnu
2020-11-25 00:05:57  ❤️  by Canvas, 2020-2020
2020-11-25 00:05:57  📋 Chain specification: Development
2020-11-25 00:05:57  🏷 Node name: somber-thread-7554
2020-11-25 00:05:57  👤 Role: AUTHORITY
2020-11-25 00:05:57  💾 Database: RocksDb at /tmp/substrateBjvYLz/chains/dev/db
2020-11-25 00:05:57  ⛓  Native runtime: canvas-8 (canvas-0.tx1.au1)
2020-11-25 00:05:57  🔨 Initializing Genesis block/state (state: 0x76e4…0f61, header-hash: 0x70f1…6167)
2020-11-25 00:05:57  👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
2020-11-25 00:05:57  ⏱  Loaded block-time = 6000 milliseconds from genesis on first-launch
2020-11-25 00:05:57  Using default protocol ID "sup" because none is configured in the chain specs
2020-11-25 00:05:57  🏷 Local node identity is: 12D3KooWDdvLqPW8gzaPBWgYjd6Q2yC2abk6713QykMfVAGHVtfr
2020-11-25 00:05:57  📦 Highest known block at #0
2020-11-25 00:05:57  〽️ Prometheus server started at 127.0.0.1:9615
2020-11-25 00:05:57  Listening for new connections on 127.0.0.1:9944.
2020-11-25 00:06:00  🙌 Starting consensus session on top of parent 0x70f1a0488a744075c07ca30d890d981697ffff0c2ef024e9753b9152afd46167
2020-11-25 00:06:00  🎁 Prepared block for proposing at 1 [hash: 0x50ff56ca14d680e03c3c1a2a231f27a1c4ffee2c52bba5a8459112f5a375c2ff; p
arent_hash: 0x70f1…6167; extrinsics (1): [0x115d…2969]]
2020-11-25 00:06:00  🔖 Pre-sealed block for proposal at 1. Hash now 0x0aee39eb04a2283232d41ca12ea1418f3215378455e2ea4e0e9312ec9455307
2, previously 0x50ff56ca14d680e03c3c1a2a231f27a1c4ffee2c52bba5a8459112f5a375c2ff.
2020-11-25 00:06:00  ✨ Imported #1 (0x0aee…3072)
```

This is pretty easy to scan for useful information.
On the console the timestamp is colored in such a way that,
even with some lines wrapping,
each log entry is easy to identify.

When a server starts logging,
as a newcomer the two things I usually want to know are:

- Is it operating correctly?
- What ports is it listening on?

`dfx start` ends like this

```
$ dfx start
...
Jan 24 18:15:15.537 WARN s:fscpm-uiaaa-aaaaa-aaaap-yai/n:eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe/ic_http_handl
er/ic_http_handler NNS subnet not found in network topology. Skipping fetching the delegation.
Starting webserver on port 42067 for replica at "http://localhost:42067"
binding to: V4(127.0.0.1:8000)
replica(s): http://localhost:42067/
Jan 24 18:17:52.946 WARN s:fscpm-uiaaa-aaaaa-aaaap-yai/n:eq5dc-p6wlh-rvwea-wfxrq-fqewv-n4q4v-k22qr-awprk-mcqa5-vjyrh-aqe/ic_state_layo
ut/utils StateManager runs on a filesystem not supporting reflinks (attempted to reflink /home/ubuntu/dfinity/hello/.dfx/state/replica
ted_state/node-100/state/tip/subnet_queues.pbuf => /home/ubuntu/dfinity/hello/.dfx/state/replicated_state/node-100/state/fs_tmp/scratc
hpad_0000000000000064/subnet_queues.pbuf), running big canisters can be very slow
...
```

And it's not all that clear.
There are two different output formats (log lines and regular printlns).
The three printlns,
which are probably the information the devs want me to see most,
are visually obscured in the line-wrapped log lines.
In the important printlns,
there are two addresses involved,
printed in three different ways.
Port 8000 is bound for something, but what?
We're told that a webserver for "replicas" is listening on port 42067,
twice,
with the port mentioned three times.
The first time we're told what's about to happen ("starting");
the last time it's implied that the port has successfully opened and is listening.
The port 8000 message again says what's about to happen ("binding to"),
but not that that port has successfully opened and is listening.




## Deploying and running the contract

The next step of the quick start
is to [register, build, and deploy the application][rbdta].

[rbdta]: https://sdk.dfinity.org/docs/quickstart/local-quickstart.html#register-ids

`dfx deploy` builds and deploys the two canisters in the project.
One of those canisters is `hello_assets`,
which is the frontend for the web application.

So that's a notable difference from Ethereum-like platforms,
where the frontends that use a contract just live on the web somehow.
Here the frontend code lives in the network's own storage,
and a network node can serve the entire web app.

I notice that there is no Rust in the generated `hello` applicaiton &mdash;
just an a simple npm package configuration that delegates entirely to webpack,
the `hello_assets` web application, and a `hello.mo` Motoko script.

I am getting a picture of how the developer experience with this system could end up quite fun,
with several of the typical rough edges of Ethereum-style application development out of the picture.

The rest of the local quick-start proceeds smoothly.

I run the `greet` method and get the output as expected:

```
$ dfx canister call hello greet everyone
("Hello, everyone!")
```

I load the app in the web browser and it runs.





## Aimee resumes her tutorial


Aimee, a day after deploying "hello",
comes back to the tutorial,
restarts here node with `dfx start`.
She realizes she no longer knows the ID
of the canister she deployed yesterday.
This is information shee needs to open it in the web UI
at

```
http://127.0.0.1:8000/?canisterId=<...>
```

She finds that information in her terminal backscroll.
Probably we could have figured out how to query that information
otherwise.

Aimee finds this command hard to read:

```
dfx canister call hello greet everyone
```

It makes sense to me,
knowing how CLI subcommands work,
and realizing "hello" is a contract,
"greet" is a method,
and "everyone" is an argument to that method.

Although I think this unfurling of sub-commands within sub-commands,
from the CLI commands into the contract methods,
is elegant,
I do recognize that the lack of argument names means that the command itself contains
no meta-information about what "hello", "greet", and "everyone" are.

She finds it not clear where the CLI sub-commands end and their arguments begin,
expecting _some_ `--foo`-style arguments somewhere.

I like this command though :)

She takes the opportunity to explore the feedback she gets from the CLI as these
arguments are progresively added from `dfx canister call`:

```
$ dfx canister call 
error: The following required arguments were not provided:
    <canister-name>
    <method-name>

USAGE:
    dfx canister call [FLAGS] [OPTIONS] <canister-name> <method-name> [argument]

For more information try --help

$ dfx canister call hello
error: The following required arguments were not provided:
    <method-name>

USAGE:
    dfx canister call [FLAGS] [OPTIONS] <canister-name> <method-name> [argument]

For more information try --help

$ dfx canister call hello memememe
The Replica returned an error: code 3, message: "Canister rwlgt-iiaaa-aaaaa-aaaaa-cai has no update method 'memememe'"

$ dfx canister call hello greet
Invalid data: Unable to serialize Candid values: wrong number of argument values

$ dfx canister call hello greet memeememememme
("Hello, memeememememme!")
```

This is pretty helpful.
It is most helpful while the CLI is parsing known arguments.
When she accidentally forgets the "greet" method name,
the error does provide enough info to figure out what went wrong.
The errors that don't come from the command-line parser could probably be more consistent and helpful,
but I don't have specific insights to share.





## Deploying to the live network

The quick start documentation also includes a page on [network deployment][nwdm].

[nwdm]: https://sdk.dfinity.org/docs/quickstart/network-quickstart.html

Aimee tries to follow these instructions.
Many of the instructions are the same as for local deployment,
except they need to specify `--network=ic`,
or something similar.

Steps 4 and 5 of [register, build, and deploy the application][rbdan]
confuse her.

[rbdan]: https://sdk.dfinity.org/docs/quickstart/network-quickstart.html#net-deploy

In these steps we are told to check the cycle balance of the canister,
and to call the `greet` method of `hello`,
with these two commands:

```
dfx canister --network=ic call <WALLET-CANISTER-ID> cycle_balance
dfx canister call hello greet everyone
```

The number of problems we run into attempting to accomplish these two steps
becomes overwhelming,
and my notes are filled with digressions.

Firstly,
we haven't been told explicitly what a "wallet canister ID" is,
but we guess it is in this output
printed by `dfx deploy --network=ic`:

```
"hello" canister created on network "ic" with canister id: "xibue-hiaaa-aaaaa-qabza-cai"
```

Unfortunately,
there just doesn't seem to be a method called `cycle_balance`
on the canister.
We try running this command in a bunch of variations,
local network vs live network,
cannister id vs cannister name:

```
dfx canister --network=ic call xibue-hiaaa-aaaaa-qabza-cai cycle_balance
dfx canister --network=ic call hello cycle_balance
dfx canister call rwlgt-iiaaa-aaaaa-aaaaa-cai cycle_balance
dfx canister call hello cycle_balance
```

All return the same error:

```
The Replica returned an error: code 3, message: "Canister xibue-hiaaa-aaaaa-qabza-cai has no update method 'cycle_balance'"
```

riprepping the docs repo yields 3 mentions of "wallet",
and leads us to think maybe there is another cannister
that we don't know about that is a special "wallet cannister".

We're lost and keep going.

The second command we are told to execute,

```
dfx canister call hello greet everyone
```

This pretty clearly is missing the `--network=ic` argument.
We check this by proving that it doesn't work while we aren't running `dfx start`

Running it with `--network=ic` produces the same expected output
as on the local network.

We [submit a pull request to fix what we can in docs][pr].

[pr]: https://github.com/dfinity/docs/pull/289

While we are working through these steps in the instructions,
we hit this error:


```
$ dfx deploy --network=ic
Deploying all canisters.
Creating canisters...
Creating canister "hello"...
The replica returned an HTTP Error: Http Error: status 504 Gateway Timeout,
content type "text/html", content: [60, 104, 116, 109, 108, 62, 13, 10, 60, 104,
101, 97, 100, 62, 60, 116, 105, 116, 108, 101, 62, 53, 48, 52, 32, 71, 97, 116,
101, 119, 97, 121, 32, 84, 105, 109, 101, 45, 111, 117, 116, 60, 47, 116, 105,
116, 108, 101, 62, 60, 47, 104, 101, 97, 100, 62, 13, 10, 60, 98, 111, 100, 121,
62, 13, 10, 60, 99, 101, 110, 116, 101, 114, 62, 60, 104, 49, 62, 53, 48, 52,
32, 71, 97, 116, 101, 119, 97, 121, 32, 84, 105, 109, 101, 45, 111, 117, 116,
60, 47, 104, 49, 62, 60, 47, 99, 101, 110, 116, 101, 114, 62, 13, 10, 60, 104,
114, 62, 60, 99, 101, 110, 116, 101, 114, 62, 110, 103, 105, 110, 120, 47, 49,
46, 49, 56, 46, 48, 32, 40, 85, 98, 117, 110, 116, 117, 41, 60, 47, 99, 101,
110, 116, 101, 114, 62, 13, 10, 60, 47, 98, 111, 100, 121, 62, 13, 10, 60, 47,
104, 116, 109, 108, 62, 13, 10]
```

Here,
the server we're connecting to returned an error,
with accompanying HTML explaining it,
and `dfx` has unhelpfully presented the HTML as bytes.

At some point Aimee attempted to call `hello greet`
by "hello"s canister ID, instead of by name,
and got this error:

```
$ dfx canister --network=ic call xibue-hiaaa-aaaaa-qabza-cai greet everyone
error: parser error
  ┌─ Candid argument:1:1
  │
1 │ everyone
  │ ^^^^^^^^ Unexpected token
  │
  = Expects "("

Invalid argument: Invalid Candid values: Candid parser error: Unrecognized token `Id("everyone")` found at 0:8
Expected one of "("
```

This is confusing and meaningless to us.

Continuing the network deployment,
the next step is to [test the application front-end][ttafe].

[ttafe]: https://sdk.dfinity.org/docs/quickstart/network-quickstart.html#quickstart-frontend

Following these instructions,
we should be able to open our "hello" app at
this URL:

> [https://ur4ki-qiaaa-aaaab-aacga-cai.ic0.app/](https://ur4ki-qiaaa-aaaab-aacga-cai.ic0.app/)

Even though the docs said to use the ID of the _hello_assets_
cannister,
we mistakenly use the ID of our _hello_ contract cannister,
and get this error:

> 

In Firefox:

```
An error happened:

v/<@https://xtnc2-uaaaa-aaaab-qadaq-cai.ic0.app/bootstrap-0e838d0d6297858d1bfe.js:2:43879
```

In Brave:

```
An error happened:
Error: Query failed:
  Status: rejected
  Message: IC0302: Canister xtnc2-uaaaa-aaaab-qadaq-cai has no query method 'retrieve'

    at r.retrieve (https://xtnc2-uaaaa-aaaab-qadaq-cai.ic0.app/bootstrap-0e838d0d6297858d1bfe.js:2:43879)
    at async _loadJs (https://xtnc2-uaaaa-aaaab-qadaq-cai.ic0.app/bootstrap-0e838d0d6297858d1bfe.js:2:245656)
    at async _main (https://xtnc2-uaaaa-aaaab-qadaq-cai.ic0.app/bootstrap-0e838d0d6297858d1bfe.js:2:246619)
```

We are confused over this a long time until Aimee realizes she used the wrong canister ID.
Nagivating to the correct _assets_ URL of course works.


## Writing our own contract

To create our own contract we first try to start by copying the [echo example][echo].

[echo]: https://github.com/dfinity/examples/tree/master/motoko/echo

The example though is out of date and running `dfx deploy` yields:

```
Warning: The version of DFX used (0.6.12) is different than the version being run (0.6.20).
This might happen because your dfx.json specifies an older version, or DFX_VERSION is set in your environment.
We are forwarding the command line to the old version. To disable this warning, set the DFX_WARNING=-version_check environment variable.

Error when trying to forward to project dfx:
Unknown version '0.6.12'.
Installed executable: 0.6.20
```

Editing `dfx.json` to update the `dfx` version fixes the problem.

Regardless, we realize that `echo` isn't a better base to build off of
than `hello`,
so we go back to using `dfx new` to create our project,
in this case `dfx new tba`.

When running `dfx start` we again are
"strongly encouraged to upgrade by running 'dfx upgrade'",
though this time `dfx` doesn't attempt to do the upgrade itself.

We run `dfx upgrade` and it does nothing
(later we see the same suggestion,
we run `dfx upgrade`,
and it does upgrade to 0.6.21).

We edit our Motoko script to be

```
actor {
    stable var tba_msg = "The Big Announcement";

    public func set_message(msg : Text) {
        tba_msg := msg;
    };

    public func get_message() : async Text {
        return tba_msg;
    };
};
```

We deploy it with `dfx deploy`
and test it:

```
$ dfx canister call tba set_message "this is a test"
()
$ dfx canister call tba get_message
("this is a test")
```

At some point we accidentally replace the type `Text`
with the typo `Test` and this is the compiler error:

```
$ dfx deploy
Deploying all canisters.
All canisters have already been created.
Building canisters...
Building frontend...
The build step failed for canister 'rwlgt-iiaaa-aaaaa-aaaaa-cai' with an
embedded error: The command '"/home/ubuntu/.cache/dfinity/versions/0.6.21/moc"
"/home/ubuntu/dfinity/tba/src/tba/main.mo" "-o"
"/home/ubuntu/dfinity/tba/.dfx/local/canisters/tba/tba.did" "--idl"
"--actor-idl" "/home/ubuntu/dfinity/tba/.dfx/local/canisters/idl/"
"--actor-alias" "tba" "rwlgt-iiaaa-aaaaa-aaaaa-cai" "--actor-alias" "tba_assets"
"rrkah-fqaaa-aaaaa-aaaaq-cai" "--package" "base"
"/home/ubuntu/.cache/dfinity/versions/0.6.21/base"' failed with exit status
'exit code: 1'.
Stdout:

Stderr:
/home/ubuntu/dfinity/tba/src/tba/main.mo:8.39-8.43: type error [M0029], unbound type Test
```

The final error, "unbound type Test",
is not too hard to understand,
and we get a line and column number.
I think it's reasonable.
There is a lot of spew preceding it,
but I'm undecided on if it is out of place or not &mdash;
I do like to see the underlying commands being run
when my compiles fail.


## Writing a contract in Rust?

Dfinity runs a wasm VM,
and it is theoretically possible to write contracts
in any language.

The docs do have [a section on writing contracts in Rust][cir].

[cir]: https://sdk.dfinity.org/docs/developers-guide/work-with-languages.html#_using_rust

Aimee spends some time trying to follow these docs,
but doesn't succeed.

A cursory glance at them reveals they are inconsistent and incomplete.

The two problems I see:

- Different steps explain about putting the application code in the `src` directory,
  but seem to be using several different names and conventions.
  In one step `Cargo.toml` is placed directly under `src`,
  in other steps there is a `src/my_rust_program` directory
  to contain the app.

- The build is for the `wasm32-unknown-unknown` target,
  but there's no explaination or example of how to structure the code
  to export the correct symbols or otherwise interact
  with the Dfinity runtime.

Somebody knowledgable could probably figure it out by
cribbing off of the [C examples][cex].

[cex]: https://github.com/dfinity/examples/tree/master/c/


## Successfully completing a tutorial

Ok, by this time I was personally done,
and wanted to stop,
but Aimee just kept going.

She next tried the ["add a stylesheet" tutorial][stylet].

[stylet]: https://sdk.dfinity.org/docs/developers-guide/tutorials/my-contacts.html

The great news is that Aimee worked all the way through
this tutorial and got her frontend working,
which is live here:

> [https://5jw7w-wiaaa-aaaab-qacza-cai.ic0.app/](https://5jw7w-wiaaa-aaaab-qacza-cai.ic0.app/)

She had some opinions along the way.

This tutorial is about creating a frontend in React.
We're not super excited about using React &mdash;
we just want to test-drive the interaction between a webpage
and a contract,
and React is overkill.
Frankly,
we don't _know_ React,
and are resisting learning it just for this purpose.
Still,
we keep running into this situation across multiple projects,
where all the frontend examples are written in React.

We are probably just going to have to learn React.

There were some steps in this tutorial that seemed
either redundant or could have been automated:

- Installing these things via npm:

  ```
  npm install --save react react-dom
  npm install --save typescript ts-loader
  npm install --save style-loader css-loader
  ```

  Can this be done automatically by `package.json` scripts?

- Renaming `main.mo` to `contacts.mo` and `index.js` to index.jsx`.

  Several steps in this tutorial focused on these file renamings.
  The `main.mo` to `contacts.mo` renaming seems superflous &mdash;
  if renaming the main source file is important,
  maybe `dfx new` should be doing that renaming itself.
  The `index.js` to `index.jsx` renaming is probably necessary but unfortunate.

In total,
it _felt_ like we weren't getting a lot of mileage out of the `dfx new`
template,
since so much had to be changed.
It would have though been much harder to do without the template as a starting point.

There was no pre-written example source code for this tutorial
to cheat off of,
and Aimee said she wished there were.

Though if the example did exist,
would she actually have done the steps manually herself?




## Closing thoughts

As usual for me,
I got way bogged down in the details
of the onboarding experience,
and didn't get far into the actual work of hacking.

Looking back,
I can see that a lot of the mistakes we made were our own,
that frustration is common when learning all new types of development,
and I also know we've been frustrated with the experience of
programming on almost every single blockchain we've tried.

And I see that there were several moments where I was
intrigued by the design of Dfinity,
and could imagine it being a fun experience.
In particular,
the programming model here appears to be more traditional
than I expect from smart contracts,
giving me hope that I could think less about blockchain mechanics
and more about the logic of the services I want to write.

The application logic being bundled with the contract
logic has potential &mdash;
other blockchains don't have such a wholistic view
on application programming,
providing a javascript library to RPC with a node,
and having no opinion about the rest of the application development experience.
It also though limits how developers can work with the system &mdash;
the tutorial we worked through explained that
"currently, you can only use Javascript to implement the front-end for your canister".

By the time we finished the quick start,
I felt like I was done for now,
that Dfinity needs some more time to polish the developer experience
before I would want to try it more extensively.

Aimee made it further than I did &mdash;
there are [many tutorials][tuts] beyond the quickstart
that I didn't even see.
And from experience,
I know that once one has pushed past the
initial onboarding phase,
that one can be productive with just about any platform,
so there still may be a lot worth diving into in Dfinity
right now.

[tuts]: https://sdk.dfinity.org/docs/developers-guide/tutorials-intro.html

Even though there were a lot of challenges
to overcome in the onboarding process,
unlike e.g. Substrate,
at no point here did I think the challenges were _because of Rust_.
Writing Dfinity contracts doesn't require Rust knowledge.
Working with the tools doesn't require Rust knowledge.

I didn't dwell on it much in the body of this post,
but that Dfinity's source code is mostly still closed
was offputting to me.
Looking at the GitHub before starting this experiment,
I did not have the impression Dfinity was even ready for
casual testing &mdash;
it looks like a project that is still stealthily incubating.
As an open source developer,
I expect to be able to dig into my tooling and investigate
why things are behaving the way they are,
and to fix the bugs I run into and participate in the upstream project.

Dfinity does not have a particularly open-source posture right now.
If attracting developers is an immediate goal,
then that's a disadvantage compared to the many blockchains
that are developed completely openly.

