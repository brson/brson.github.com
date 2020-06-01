---
layout: post
title: "Intro to Rust on Fuchsia"
tags: [rust]
---

The astute will be aware by now of [Fuchsia], a new operating system being developed by a team at Google. The very astute may even be aware that Rust runs on Fuchsia (largely due to the work of Raph Leviene, who those in the Rust community may be familiar with).

As a systems enthusiast, and a Rust enthusiast, this excites me much.

Fuchsia is a greenfield OS project with a modern microkernel design, and Rust is a modern systems language. There is so much code that needs to go into a production OS; there is so much opportunity for Rust to make an impact in this new ecosystem, whereas it is very hard to make inroads in established systems like Linux.

So I want to start writing Rust software for Fuchsia. Here's what I learned on Day 1.

[Fuchsia]: https://fuchsia.googlesource.com/

## Roadmap

- [Fuchsia resources][fr]
- [Build system setup][bss]
- [Building Fuchsia (hopefully)][bfh]
- [Building Fuchsia (maybe really?)[bfm]
- [Building Fuchsia (on EC2)][bfe]
- [Conclusian?][cq]
- [Building Fuchsia (yeah, right)[bfy]
- [Peeking under the hood][pk]
- [A stupid tangent][st]
- [Ok, what's with that `bootstrap_jiri` script?][bj]
- [Getting a working checkout of Fuchsia via jiri][wc]
- [Does yoga improve jiri's success rate?][jsr]
- [How I got Fuchsia downloadad and built][dlb]
- [Updating Fuchsia][udf]
- [Building Fuchsia!][bdf]
- [No, no, I'm totally serious &mdash; now I'm going to build Fuchsia][nnits]
- [The zircon boot sequence][zbs]
- [Running Fuchsia][rf]
- [Running code on Fuchsia][rcof]

## Fuchsia resources

The documentation for setting up the Fuchsia environment is spread across several pages, and the degree to which any particular step is necessary or correct seems as might be expected from such a young project (i.e. it's a bit messy and unclear). Here are some of the most important resources I used during my initial exploration:

- [Fuchsia getting started][f1] - The primary build instructions. Mostly follow this.
- [Acquiring fuchsia source][f2] - Fuchsia uses a Google in-house tool for coordinating their source repos, called [jiri]. This explains how to use it to get the source.
- [The Fuchsia Book][fb] - Lots of good info about various systems, but clearly incomplete. (Ripe for contribution?!)
- [The Jiri Readme][jiri] - Jiri is the tool Fuchsia employs to manage a multi-repo project. It'll be used to acquire and update the source.

[f1]: https://fuchsia.googlesource.com/docs/+/HEAD/getting_started.md
[f2]: https://fuchsia.googlesource.com/docs/+/HEAD/development/source_code/README.md
[fb]: https://fuchsia.googlesource.com/docs/+/HEAD/the-book/README.md
[jiri]: https://fuchsia.googlesource.com/jiri/

I'll show below what _I_ did, as a synthesis of the above.

I'm working on Ubuntu 16.04 under Windows Subsystem for Linux ([WSL]). (Aside: last time I tried to build Fuchsia on WSL I ran into a mystery error, asked for help, and the response indicated that probably nobody had built or tested under WSL yet - let's see if things have improved).

[WSL]: https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux




## Build system setup

Summer 2018.

I have a prior WSL system running on Windows 10, filled with various development tools and Rust toolchains, so I'm partially set up already. Mostly what I need to do is follow the Fuchsia instructions to get it building.

As with many large projects, the fuchsia build system is its own intimidating beast and requires a bit of learning just to get started.

Install prerequisites:

```sh
sudo apt-get install texinfo libglib2.0-dev liblz4-tool autoconf libtool libsdl-dev build-essential golang git curl unzip
```

Since it seems like this is going to be one of those 'big' multi-repo projects with lots of pieces to it, I'm creating my own directory to hold Fuchsia stuff:

```sh
mkdir fuchsia && cd fuchsia
```

Everything I do from here on will be in this directory unless otherwise stated.




## Building Fuchsia (hopefully)

Ok, now I'm working off the ["Fuchsia Source"][fs] instructions.

[fs]: https://fuchsia.googlesource.com/docs/+/HEAD/development/source_code/README.md

They say that the bootstrap requires Go 1.6 or newer, and I have ...

```sh
brian@whatever:~/fuchsia⟫ go version
go version go1.6.2 linux/amd64
```

Good.

So the instructions immediately ask me to decide the ["layer"] I want to build, recommending "topaz". On these big projects with their own cultures and terminology to learn. That's just the way it is. But it nudges up the learning curve. From that "layer" link I learn that Fuchsia is a "layer cake", Fuchsia the overall project, with the following layers:

- __Zircon__ - The microkernel, the device model and first-party drivers (which I assume run in usermode). Also libc - sad that libc yet must have such a privileged position in this greenfield operating system, but that's life in this reality that Unix made for us. It also contains a library that seems to be given the same deference as libc: [fdio]. I'll read about that later.

- __Garnet__ - Remote device management and administration, updates, package management and deployment. Curiously also contains the network, graphics, and media services. Wonder what the coupling is there.

- __Peridot__ - A layer filled with mystery Fuchsia concepts - "stories", "agents", "entities", "story runners", "ledger", "resolver", "context engine", "suggestion engine". This seems to be the user-facing interface - the thing users will see as the OS. Interested in understing what all these concepts mean, if they add up to an interesting new UI _paradigm_ (hm, wish I didn't just write that word...)

- __Topaz__ - The software built on the Peridot foundations, in four kinds: "modules", "agents", "shells", and "runners". I yet know what none of those things are. Adventure! There are runners for "Web", "Dart", and "Flutter" - I wonder if there's one for Rust, and if not whether I can help make one, or help inspire someone else to make one. Note to self: figure out what runners do.

So that is my paraphrasing of these subsystems's descriptions, plus educated guesses about their function. Read the prior link for how the Fuchsia devs describe them.

These layer names are codenames for the sake of organization and development and not expected to be e.g. named in Fuchsia APIs.

["layer"]: https://fuchsia.googlesource.com/docs/+/HEAD/development/source_code/layers.md
[fdio]: https://fuchsia.googlesource.com/docs/+/HEAD/the-book/life_of_an_open.md

(At this point I come to the opinion that the Fuchsia docs could use some improvement, and I clone them into my 'fuchsia' directory for further [ripgrepping][rg]):

```sh
git clone https://fuchsia.googlesource.com/docs
```

[rg]: https://github.com/BurntSushi/ripgrep

That clone takes a surprisingly long time ... maybe the docs are more thorough than I was thinking. Anyway, whatever. Maybe docs are something I can contribute usefully to.

From ripgrepping the docs repo I discover there is a thing called "the-book" (like Rust!), and oh it was hidden in the README.md as a link titled "System - documentation for how Fuchsia works". Kinda unasuming. If it were me - and this is just a first-pass, whatever's coming to me, I might write:

- ["The Fuchsia Book"][fb] - Everything you want to know about. Read this!

Anyway, I've linked it there for you. And, uh, gotten really lost on this doc-reading tangent. I was supposed to be following instructions for building Fuchsia, so back to that.




## Building Fuchsia (maybe really?)

So again following the ["Fuchsia Source"][fs] instructions.

[fs]: https://fuchsia.googlesource.com/docs/+/HEAD/development/source_code/README.md

On their recommendation I will build `topaz`, because it contains everything. This next step is going to invoke a magic `curl` script that sets up a bunch of stuff needed to build `topaz`. I don't know yet all it's going to do, but I'm going to run it and then poke around:

```sh
curl -s "https://fuchsia.googlesource.com/scripts/+/master/bootstrap?format=TEXT" | base64 --decode | bash -s topaz
```

(Note that last "topaz" - presumably I could have written something else there, like `zircon`; and again note from the official instructions that after this is all done I should be able to run `scripts/fx set-layer <layer>` to change my mind and work on `zircon` or whatever layer of the Fuchsia layer cake I please).

Excitement is rising!

I suspect this is going to take a long time, as it is with big projects. The first thing this script prints is

```sh
Please add /home/brian/fuchsia/fuchsia/.jiri_root/bin to your PATH
WARN: Please opt in or out of analytics collection. You will receive this warning until an option is selected.
To check what data we collect run 'jiri init -show-analytics-data'
To opt-in run 'jiri init -analytics-opt=true "/home/brian/fuchsia/fuchsia"'
To opt-out run 'jiri init -analytics-opt=false "/home/brian/fuchsia/fuchsia"'
```

So while I'm waiting I can maybe do some of that jazz.

I edit my `~/.bashrc file, and on the very last line write:

```sh
export PATH="$HOME/fuchsia/fuchsia/.jiri_root/bin:$PATH"
```

You've probably already got something similar there for your `~/.cargo/bin` directory. This is just like that - more dev-environment specific configuration. This is the point where we're really buying into the Fuchsia system, whatever it is; just like when you decide you are going to do things the Rust way and allow `~/.cargo/bin` into your environment.

I still don't know what jiri is, and that script I curled earlier is still running, but I can run `source ~/.bashrc` to pull `jiri` onto my path and run it:

```sh
brian@whatever:~/fuchsia⟫ jiri
ERROR: jiri: no command specified

Command jiri is a multi-purpose tool for multi-repo development.

Usage:
   jiri [flags] <command>

The jiri commands are:
   branch          Show or delete branches
   diff            Prints diff between two snapshots
   edit            Edit manifest file
   grep            Search across projects.
   import          Adds imports to .jiri_manifest file
   init            Create a new jiri root
   patch           Patch in the existing change
   project         Manage the jiri projects
   project-config  Prints/sets project's local config
   manifest        Reads <import> or <project> information from a manifest file
   run-hooks       Run hooks using local manifest
   runp            Run a command in parallel across jiri projects
   selfupdate      Update jiri tool
   snapshot        Create a new project snapshot
   source-manifest Create a new source-manifest from current checkout
   status          Prints status of all the projects
   update          Update all jiri projects
   upload          Upload a changelist for review
   version         Print the jiri version
   help            Display help for commands or topics
Run "jiri help [command]" for command usage.

The jiri additional help topics are:
   filesystem     Description of jiri file system layout
   manifest-files Description of manifest files
Run "jiri help [topic]" for topic details.

The global flags are:
 -color=auto
   Use color to format output. Values can be always, never and auto
 -j=25
   Number of jobs (commands) to run simultaneously
 -metadata=<just specify -metadata to activate>
   Displays metadata for the program and exits.
 -progress-window=5
   Number of progress messages to show simultaneously. Should be between 1 and 10
 -q=false
   Same as -quiet
 -quiet=false
   Only print user actionable messages.
 -root=
   Jiri root directory
 -show-progress=true
   Show progress.
 -show-root=<just specify -show-root to activate>
   Displays jiri root and exits.
 -time=false
   Dump timing information to stderr before exiting the program.
 -time-log-threshold=10s
   Log time taken by operations if more than the passed value (eg 5s). This only works with -v and -vv.
 -v=false
   Print debug level output.
 -vv=false
   Print trace level output.

Run "jiri help -style=full" to show all flags.
```

Yeah, so it's a thing! Looks like a developer tool, like many other developer tools. Developers love making developer tools (I know - I've done it too). They mostly make our lives better, but y'know, there are so many of them. Here's another one I'm going to have to internalize if I'm going to be the Fuchsiarustic superstar I imagine myself to be. This is probably something Google leans on extensively, and we've got to learn it to participate in their projects. Whatever, I'm not mad or anything, you know? That's just the way these things are. Every big org has their own beloved (or behated) proceses and toolings. Let's learn to love jiri.

So I don't think there's much interesting I can do or want to do with jiri yet. Back in my other window that curl script I ran is now doing this:

```sh
Updating all projects
PROGRESS: Creating project "third_party/vim"
PROGRESS: Creating project "docs"
PROGRESS: Creating project "external/github.com/g-truc/glm"
PROGRESS: Creating project "third_party/icu"
PROGRESS: Creating project "third_party/googleapis"
```

So progress is slow. I wonder if I have the necessary diskspace and idly check it with `df -h` - 80% free, seems ok.

Tap, tap. Tap, tap, tap. /me bored.

(I go off and imbibe the appropriate substances to keep that motivation flowing. I comment recklessly on various issue trackers beloging to other people and projects. Come on, man - get focused.)

Oh, god. Oh my god (Or "higher-power" or whatever - I'm not a believer of anything in particular. And now this is a religious piece too).

I come back to my terminal and see this disaster:

```sh
Updating all projects
ERROR: exit status 1

Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon

Attempt 2/3: running hook(download-prebuilt) for project zircon

ERROR: exit status 1

Wait for 5s before next attempt...: running hook(cipd-update) for project garnet

Attempt 2/3: running hook(cipd-update) for project garnet

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(update) for project buildtools

Attempt 2/3: running hook(update) for project buildtools

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon

Attempt 3/3: running hook(download-prebuilt) for project zircon

ERROR: "running hook(download-prebuilt) for project zircon" failed 3 times in a row, Last error: exit status 1
Error for hook(download-prebuilt) for project "zircon"
######################################################################## 100.0%
Error: resolving package: failed to resolve package version (line 56): prefix "fuchsia_internal/firmware/intel-adsp-sst" doesn't exist or the caller is not allowed to see it.
/home/brian/fuchsia/fuchsia/zircon/scripts/download-prebuilt: /home/brian/fuchsia/fuchsia/zircon/../buildtools/cipd failed.  For direct downloads, remove cipd from PATH.
[P29929 15:31:06.292 fs.go:424 W] fs: failed to rename("/home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/pkgs/4/ae0677829357edb199c21e76b1ea31eb8ef62817", "/home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/tr
ash/sOEyPVvuZieZ") - rename /home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/pkgs/4/ae0677829357edb199c21e76b1ea31eb8ef62817 /home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/trash/sOEyPVvuZieZ: permission de
nied
[P29929 15:31:06.293 fs.go:360 W] fs: failed to rename("/home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/pkgs/4/EcIBHu1UYwws/x", "/home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/pkgs/4/ae0677829357edb199c21
e76b1ea31eb8ef62817") - rename /home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/pkgs/4/EcIBHu1UYwws/x /home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/pkgs/4/ae0677829357edb199c21e76b1ea31eb8ef62817: file ex
ists
[P29929 15:31:15.189 client.go:1416 E] Failed to install fuchsia/sysroot/linux-amd64:ae0677829357edb199c21e76b1ea31eb8ef62817 - rename /home/brian/fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/pkgs/4/EcIBHu1UYwws/x /home/brian/
fuchsia/fuchsia/zircon/prebuilt/downloads/.cipd/pkgs/4/ae0677829357edb199c21e76b1ea31eb8ef62817: file exists
Error: failed to update packages, see the log.
/home/brian/fuchsia/fuchsia/zircon/scripts/download-prebuilt: /home/brian/fuchsia/fuchsia/zircon/../buildtools/cipd failed.  For direct downloads, remove cipd from PATH.

output for hook(download-prebuilt) for project "zircon"

PROGRESS: running hook(update) for project "buildtools"
```

You can't see it here, but on the terminal those "ERROR"s are bright red and threatening.

I am feeling defeated.

OK. This seems disastrous. I Ctrl-C kill this bloodbath.

Let's optimistically chalk this fail up to WSL. I'll boot up an EC2 instance, also I'll start kindling a fire out of dollar bills. 

(_NB: I wrote several more sections trying to figure out what the hell is going wrong with my setup, but I think _probably_ the right solution is to let the setup script [running jiri] finish its thing, then cd into the `fuchsia` directory and run `jiri update` to have it run through the manifest and make sure all the repo's it's downloading are in a consistent state. But IDK - this was not a good first look_).




## Building Fuchsia (on EC2)

OK so I go through the whole EC2 thing. I pick a t2.medium machine running Ubuntu 16.04 LTS and give it 32 GB of disk space, assuming this build environment will be hungry for disk. Configure my preferred screen multiplexer, [byobu], just how I like it.

[byobu]: https://help.ubuntu.com/community/Byobu

Create my `fuchsia` directory again.

```sh
mkdir fuchsia && cd fuchsia
```

Recalling that Go 1.6+ is a requirement, I discover that this time I don't have Go on the stock install:

```sh
ubuntu@whatever:~/fuchsia⟫ go
The program 'go' is currently not installed. You can install it by typing:
sudo apt install golang-go
```

So I install it as suggested:

```sh
ubuntu@whatever:~/fuchsia⟫ sudo apt install golang-go -y
... snipped many lines of spew ...

E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-5/libstdc++-5-dev_5.4.0-6ubuntu1~16.04.9_amd64.deb  404  Not Found [IP: 34.212.136.213 80]

E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-5/g++-5_5.4.0-6ubuntu1~16.04.9_amd64.deb  404  Not Found [IP: 34.212.136.213 80]

E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
```

Ok, I know this. I just need to update the package database. Fuck. This is such an ordeal. I just want to celebrate the joy of Rust on Fuchia; but at this rate I may never expereience that joy, if it exists out there at all.

Updating and then installing go:
```sh
ubuntu@whetever:~/fuchsia⟫ sudo apt update && sudo apt install golang-go -y && go version
... miles and miles of spew ...
go version go1.6.2 linux/amd64
```

Ok, now we're ready to try that magic curl-to-jiri command again. Damn this is the most boring tutorial ever: just command -> fail -> command -> fail. Let's see some Fuchsia action this time.

```sh
curl -s "https://fuchsia.googlesource.com/scripts/+/master/bootstrap?format=TEXT" | base64 --decode | bash -s topaz
```

Like last time, it quickly prints out a suggestion to add jiri to `$PATH`, along with some info about analytics that I choose to ignore for nowe.

While that script is long-running, I realize I don't have emacs installed with which to modify my `~/.bashrc`, nor do I even have Rust installed.

So here's some more useful commands:

```sh
sudo apt install emacs24-nox -y
```

That's an emacs without GUI support - terminal emacs is all I need. And here's Rust - but you probably know this:

```sh
curl https://sh.rustup.rs -sSf | sh
```

And so with emacs installed I go add jiri to `$PATH`, the same way that `rustup` did, this time curiously by editing `~/.profile`, not `~/.bashrc`. The tail of it looks like this now:

```
export PATH="$HOME/.cargo/bin:$PATH"
export PATH="$HOME/fuchsia/fuchsia/.jiri_root/bin:$PATH"
```

I guess on this machine `rustup` found `~/.profile` and preferred it to `~/.bashrc`. I always forget the exact difference, but I know `~/.profile` will only run for a login shell (whetever that is - so much to learn, or not, whatever). I `source ~/.profile` to get my `PATH` working, and again the `jiri` command spits out a bunch of stuff I am not prepared to take any action on.

Oh, and guess what?

__I come back to my magic curl script session, and it has completely failed in the same way that the WSL attempt did.__ (I mentally calculate how many EC2 dollars I just spent to accomplish more nothing).

Where's that tableflip emoticon? Ok [there][tf] it is. Come here my love.

[tf]: https://www.emojicons.com/table-flipping

(╯°□°)╯︵ ┻━┻

(╯°□°)╯︵ ┻━┻ so much.




## Conclusion?

So this has been a succinct demonstration of how to assiduously follow instructions and fail hard.

I'm going to try not to think of it that way though - that kind of
black and white thinking, that something is either a "success" or
"failure" leads me down into a cycle of self-defeating emotions.
Surely this isn't a _total_ failure right? We learned something. I
kinda don't want to think about what it is right now, but we learned
something.

We're going to try this again, and it's going to go much better.

One of my therapists at our last session gave me a new mantra to work with: "there is no success or failure". I've been using it a whole lot: so much failure here I need to mantra into unexistence....




## Building Fuchsia (yeah, right)

Ok, I stopped that EC2 instance and decided to try again under WSL, investigating what's going on with these errors.

At this point I peeked into my 'fuchsia' directory and discovered
that the 'bootstrap' script itself created a 'fuchsia' directory,
and populated it with lots of stuff, so mine isn't needed.

I'm going to start the process over, deleting my own 'fuchsia' directory
and re-running the curl script:

```sh
cd .. # get out of 'fuchsia'
rm fuchsia/ -rf 
curl -s "https://fuchsia.googlesource.com/scripts/+/master/bootstrap?format=TEXT" | base64 --decode | bash -s topaz
```

Ok, starting again.

Again it immediately splats out these tips:

```sh
Please add /home/brian/fuchsia/.jiri_root/bin to your PATH
WARN: Please opt in or out of analytics collection. You will receive this warning until an option is selected.
To check what data we collect run 'jiri init -show-analytics-data'
To opt-in run 'jiri init -analytics-opt=true "/home/brian/fuchsia"'
To opt-out run 'jiri init -analytics-opt=false "/home/brian/fuchsia"'

WARN: Please opt in or out of analytics collection. You will receive this warning until an option is selected.
To check what data we collect run 'jiri init -show-analytics-data'
To opt-in run 'jiri init -analytics-opt=true "/home/brian/fuchsia"'
To opt-out run 'jiri init -analytics-opt=false "/home/brian/fuchsia"'

Updating all projects
PROGRESS: Creating manifest: topaz
```

So, because I removed a level of `fuchsia` folders, I go back (in another window) and edit my `.bashrc` file agin
to end with:

```sh
export PATH="$HOME/fuchsia/.jiri_root/bin:$PATH"
```

And `source` it to test:

```sh
source ~/.bashrc
jiri version
```

This time I wrote `jiri version` instead of just `jiri` (actually I wrote `jiri --version`, only to discover that's not how you got the version in `jiri`), only to be greeted with:

```sh
brian@whatever:~⟫ jiri version
ERROR: cannot find .jiri_root
```

So jiri needs to be configured with a root. I bet that's this 'fuchsia' directory the curled `bootstrap` script is busy creating. Hopefully that gets set up on its own.

Oh, oh, oh - upon further inspection, if I `cd` into my `fuchsia` directory and try again then this happens:

```sh
brian@whatever:~/fuchsia⟫ jiri version
WARN: Please opt in or out of analytics collection. You will receive this warning until an option is selected.
To check what data we collect run 'jiri init -show-analytics-data'
To opt-in run 'jiri init -analytics-opt=true "/home/brian/fuchsia"'
To opt-out run 'jiri init -analytics-opt=false "/home/brian/fuchsia"'

Jiri f92b10ade2c067582aa060f5f16d51068e5fea7e 2018-06-12T21:44:46.899139
```

So inside our `fuchsia` directory we've got a `.jiri_root` ... somewhere. Look at this:

```sh
brian@whatever:~/fuchsia⟫ ls -lhA
total 0
-rw-r--r-- 1 brian brian  149 Jul  8 13:03 .jiri_manifest
drwxrwxrwx 1 brian brian 4.0K Jul  8 13:02 .jiri_root
```

So it seems like the first thing that the 'bootstrap' script did was create the `fuchsia` directory, install `jiri`, and set up the `.jiri_root`, the `bin` directory under which we manually added to the `$PATH` variable.

Let's poke around while `bootstrap` is floundering around trying to load all our Fuchsia resources.

```sh
brian@whatever:~/fuchsia⟫ ls .jiri_root
bin
brian@whatever:~/fuchsia⟫ ls .jiri_root/bin
jiri
```

That's not too interesting - `.jiri_root` contains jiri. Fine. What about that `.jiri_manifest`?

```sh
brian@whatever:~/fuchsia⟫ cat .jiri_manifest
<manifest>
  <imports>
    <import manifest="manifest/topaz" name="topaz" remote="https://fuchsia.googlesource.com/topaz"/>
  </imports>
</manifest>
```

Ok, that's nice and simple. Jiri is configured to use the "topaz" manifest, like we told the bootstrap script.

Speaking of - wtf is that script doing now?

Just sitting there, doing nothing:

```sh
Updating all projects
PROGRESS: Creating manifest: garnet
```

No errors yet like the previous attempts, but nothing useful.

A few minutes of piddling around later I check back on the running `bootstrap` script and now it says:

```sh
Updating all projects
PROGRESS: Creating manifest: third_party/boringssl
```

That's curious - I almost didn't notice, but third_party/boringssl is _not_ the same thing it was working on before; before it said "Creating manifest: garnet"; and before _that_ it said "Creating manifest: topaz".

Hm, hm, hm. That UI makes my blood pressure rise just a bit - it's a long, long, very long running process that wants to hide its progress from me - I totally thought it was just stalled doing nothing. I make a note to myself that perhaps making that less aggravating is something I could contribute.

I wonder what this progress means, so I see what's going on in `fuchsia` dir:

```sh
brian@whatever:~/fuchsia⟫ ls
build  buildtools  garnet  peridot  scripts  third_party  topaz  zircon
```

And there's stuff! Oh, and now the `bootstrap` script says

```sh
ERROR: 'git clone --no-checkout https://fuchsia.googlesource.com/third_party/jinja2 /home/brian/fuchsia/third_party/jinja2' failed:
Cloning into '/home/brian/fuchsia/third_party/jinja2'...
fatal: unable to access 'https://fuchsia.googlesource.com/third_party/jinja2/': gnutls_handshake() failed: The TLS connection was non-properly termi
nated.

command fail error: exit status 128

Wait for 5s before next attempt...: Cloning https://fuchsia.googlesource.com/third_party/jinja2

ERROR: 'git clone --no-checkout https://fuchsia.googlesource.com/third_party/libjpeg-turbo /home/brian/fuchsia/third_party/libjpeg-turbo' failed:
Cloning into '/home/brian/fuchsia/third_party/libjpeg-turbo'...
fatal: unable to access 'https://fuchsia.googlesource.com/third_party/libjpeg-turbo/': gnutls_handshake() failed: The TLS connection was non-properl
y terminated.

command fail error: exit status 128

Wait for 5s before next attempt...: Cloning https://fuchsia.googlesource.com/third_party/libjpeg-turbo

ERROR: 'git clone --no-checkout https://fuchsia.googlesource.com/third_party/iperf /home/brian/fuchsia/third_party/iperf' failed:
Cloning into '/home/brian/fuchsia/third_party/iperf'...
fatal: unable to access 'https://fuchsia.googlesource.com/third_party/iperf/': gnutls_handshake() failed: The TLS connection was non-properly termin
ated.

command fail error: exit status 128

Wait for 5s before next attempt...: Cloning https://fuchsia.googlesource.com/third_party/iperf

Attempt 2/3: Cloning https://fuchsia.googlesource.com/third_party/jinja2

Attempt 2/3: Cloning https://fuchsia.googlesource.com/third_party/libjpeg-turbo

Attempt 2/3: Cloning https://fuchsia.googlesource.com/third_party/iperf
```

Yeah, so we're back to the previous bummer-state. Curiously, the bootstrap script doesn't show any indication of the previous successes - it's just acting like it was a total failure.

Then a few minutes later it appends some green success messages to all that fail:

```sh
PROGRESS: Creating project "docs"
PROGRESS: Creating project "third_party/curl"
PROGRESS: Creating project "third_party/syzkaller"
PROGRESS: Creating project "third_party/vulkan_loader_and_validation_layers"
PROGRESS: Creating project "dart/sdk"
```

I'm developing some deep feels about this script...




## Peeking under the hood

I'm kinda bummed this doesn't all just work. What's going on? I guess I need to dig deeper - nothing is going to stop me from Rusting up some Fuchsia today God dammit.

That `curl` command (Remember? The very first step in obtaining the Fuchsia source)

```
curl -s "https://fuchsia.googlesource.com/scripts/+/master/bootstrap?format=TEXT" | base64 --decode | bash -s topaz
```

is running the `bootstrap` script from `https://fuchsia.googlesource.com/scripts` so maybe I can peek at that and do whatever it's doing.

Here's the README for this `scripts` repo:

> <https://fuchsia.googlesource.com/scripts/>

It doesn't say much interesting about `bootstrap`.

I'm going to clone it and poke around. Since I don't want to interfere with whatever the running `bootstrap` script is _supposed_ to be doing, I'll clone it into a directory other than our `fuchsia` working directory:

```sh
cd ..
mkdir fuchsia-tmp && cd fuchsia-tmp
git clone https://fuchsia.googlesource.com/scripts
```

And here's `bootstrap`:

> <https://fuchsia.googlesource.com/scripts/+/master/bootstrap>

From the comments there:

> ```
> # The fetched script will
> # - create "fuchsia" directory if it does not exist,
> # - download "jiri" command to "fuchsia/.jiri_root/bin"
> ```

Well that at least agrees with what I've discovered myself. What else does this baby do?

The next thing it does is:

```sh
curl -s "https://fuchsia.googlesource.com/jiri/+/master/scripts/bootstrap_jiri?format=TEXT" | base64 --decode | bash -s fuchsia
```

So it's curling anothing script from the same repo - wait, no, not the
same repo, from the _jiri_ repo - `bootstrap_jiri` and ultimately
passing it to `bash` with the argument `fuchsia` (I know enough bash
to know that `bash -s fuchsia` means "execute the script from stdin
and pass 'fuchsia' as an argument to it [actually, I was pretty sure I
knew that but had to go read `man bash` just to be certain]).



## A stupid tangent

I know this blog post about (lol) "Running Rust on Fuchsia" is full of tangents not related to "Running Rust on Fuchsia", but now i'm curious about an aspect of these curl scripts Goog is teeling us to blindly run:

Between the `curl` command and the `bash` command there's this `base64 --decode` command. Wha?

So presumably we're curling a base64-encoded version of the script for some reason. Does that happen because the URL ends with `?format=TEXT` (that certainly would be counter-intuitive).

Let's try:

```sh
brian@whatever:~/curl -s "https://fuchsia.googlesource.com/scripts/+/master/bootstrap?format=TEXT"
IyEvYmluL2Jhc2gKIyBDb3B5cmlnaHQgMjAxNyBUaGUgRnVjaHNpYSBBdXRob3JzLiBBbGwgcmlnaHRzIHJlc2VydmVkLgojIFVzZSBvZiB0aGlzIHNvdXJjZSBjb2RlIGlzIGdvdmVybmVkIGJ5IGEgQlNELXN0eWxlIGxpY2Vuc2UgdGhhdCBjYW4gYmUKIyBmb3VuZCBpbiB0aGUgTElDRU5TRSBmaWxlLgoKc2V0IC1lCgpmdW5jdGlvbiB1c2FnZSB7CiAgY2F0IDw8RU5ECnVzYWdlOiBib290c3RyYXAgW3ppcmNvbnxnYXJuZXR8cGVyaWRvdHx0b3Bhel0KCkJvb3RzdHJhcCBhIEZ1Y2hzaWEgZGV2ZWxvcG1lbnQgZW52aXJvbm1lbnQgZm9yIHRoZSBnaXZlbiBsYXllci4gRGVmYXVsdHMgdG8KYm9vc3RyYXBpbmcgYXQgdGhlIHRvcGF6IGxheWVyLiBGb3IgbW9yZSBpbmZvcm1hdGlvbiBhYm91dCB0aGUgRnVjaHNpYSBsYXllcgpjYWtlLCBzZWUgPGh0dHBzOi8vZnVjaHNpYS5nb29nbGVzb3VyY2UuY29tL2RvY3MvKy9tYXN0ZXIvZGV2ZWxvcG1lbnQvc291cmNlX2NvZGUvbGF5ZXJzLm1kPi4KRU5ECn0KCmlmIFtbICQjIC1ndCAxIF1dOyB0aGVuCiAgdXNhZ2UKICBleGl0IDEKZmkKCmxheWVyPSR7MTotdG9wYXp9CgppZiBbWyAiJHtsYXllcn0iICE9ICJ6aXJjb24iIF1dICYmCiAgIFtbICIke2xheWVyfSIgIT0gImdhcm5ldCIgXV0gJiYKICAgW1sgIiR7bGF5ZXJ9IiAhPSAicGVyaWRvdCIgXV0gJiYKICAgW1sgIiR7bGF5ZXJ9IiAhPSAidG9wYXoiIF1dOyB0aGVuCiAgdXNhZ2UKICBleGl0IDEKZmkKCiMgVGhlIGZldGNoZWQgc2NyaXB0IHdpbGwKIyAtIGNyZWF0ZSAiZnVjaHNpYSIgZGlyZWN0b3J5IGlmIGl0IGRvZXMgbm90IGV4aXN0LAojIC0gZG93bmxvYWQgImppcmkiIGNvbW1hbmQgdG8gImZ1Y2hzaWEvLmppcmlfcm9vdC9iaW4iCmN1cmwgLXMgImh0dHBzOi8vZnVjaHNpYS5nb29nbGVzb3VyY2UuY29tL2ppcmkvKy9tYXN0ZXIvc2NyaXB0cy9ib290c3RyYXBfamlyaT9mb3JtYXQ9VEVYVCIgfCBiYXNlNjQgLS1kZWNvZGUgfCBiYXNoIC1zIGZ1Y2hzaWEKY2QgZnVjaHNpYQoKLmppcmlfcm9vdC9iaW4vamlyaSBpbXBvcnQgLW5hbWU9IiR7bGF5ZXJ9IiAibWFuaWZlc3QvJHtsYXllcn0iICJodHRwczovL2Z1Y2hzaWEuZ29vZ2xlc291cmNlLmNvbS8ke2xheWVyfSIKCi5qaXJpX3Jvb3QvYmluL2ppcmkgdXBkYXRlCgplY2hvICJEb25lIGNyZWF0aW5nICR7bGF5ZXJ9IGRldmVsb3BtZW50IGVudmlyb25tZW50IGF0IFwiJChwd2QpXCIuIgplY2hvICJSZWNvbW1lbmRlZDogZXhwb3J0IFBBVEg9XCIkKHB3ZCkvLmppcmlfcm9vdC9iaW46XCRQQVRIXCIiCg==
```

So yup. That's same base64 goop. What about if we leave off the `?format=TEXT`?

```sh
brian@whatever:~/curl -s "https://fuchsia.googlesource.com/scripts/+/master/bootstrap?format=TEXT"
<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>bootstrap - scripts - Git at Google</title><link rel="stylesheet" type="text/css" href="/+static/base.8PwAX-dsywmU2hx_vi_YSA.cache.css"><link rel="stylesheet" type="text/css" href="/+static/prettify/prettify.pZ5FqzM6cPxAflH0va2Ucw.cache.css"><!-- default customHeadTagPart --></head><body class="Site"><header class="Site-header"><div class="Header"><a class="Header-image" href
="/"><img src="//www.gstatic.com/images/branding/lockups/2x/lockup_git_color_108x24dp.png" width="108" height="24" alt="Google Git"></a><div class="
Header-menu"> <a class="Header-menuItem" href="https://accounts.google.com/AccountChooser?service=gerritcodereview&amp;continue=https://fuchsia.goog
lesource.com/login/scripts/%2B/master/bootstrap">Sign in</a> </div></div></header><div class="Site-content"><div class="Container "><div class="Brea
dcrumbs"><a class="Breadcrumbs-crumb" ... snip ...
```

It's a webpage. I guess that makes sense. It's this web page:

> <https://fuchsia.googlesource.com/jiri/+/master/scripts/bootstrap_jiri>

I wonder why `format=TEXT` means "base64". Performance? Weird corner-cases with HTTP-transmitted control characters? Is it possible to just request
the raw text? The mysteries of Google's source control are new to me.

But fuck it. I'm not going to solve this one. I just want to Rust.

Focus.



## Ok, what's with that `bootstrap_jiri` script?

I linked it above but here it is again:

> <https://fuchsia.googlesource.com/jiri/+/master/scripts/bootstrap_jiri>

For perusal, I'm going to clone that repo, again not into the main `fuchsia` workspace, but, like I did with the `scripts` repo, to some other location that won't interfere with whatever jiri is trying (and failing so hard) to do with the `fuchsia` directory.

```sh
cd ../fuchsia-tmp # Previously created to hold by own copy of the script repo
git clone https://fuchsia.googlesource.com/jiri
```

So what does this script do? From it's comments (yay, comments):

> ```
> # bootstrap_jiri initializes a root directory for jiri.  The following
> # directories and files will be created:
> #   <root_dir>                         - root directory (picked by user)
> #   <root_dir>/.jiri_root              - root metadata directory
> #   <root_dir>/.jiri_root/bin/jiri     - jiri binary
> #
> # The jiri sources are downloaded and built into a temp directory, which is
> # always deleted when this script finishes.  The <root_dir> is deleted on any
> # failure.
> ```

The script itself is not all that interesting, and it seems like it succeeded in doing what it claims to do (we've got a working `jiri` executable in `fuchsia/.jiri_root/bin`, so the error wasn't there.

After the jiri installation there are only three more steps in the bootstrap script:

```sh
cd fuchsia

.jiri_root/bin/jiri import -name="${layer}" "manifest/${layer}" "https://fuchsia.googlesource.com/${layer}"

.jiri_root/bin/jiri update
```

Seems pretty likely that `jiri update` failed. I'm just going to run it again.




## Getting a working checkout of Fuchsia via jiri

I just `cd` into my `fuchsia` directory and run `jiri update`. The UI for this script seems to display 5 green "PROGRESS" updates at a time: my screen flashes without scrolling as `PROGRESS: Fetching remotes for project "third_party/vim"` turns into `PROGRESS: Fetching remotes for project "third_party/crashpad"` turns into "PROGRESS: Creating project "third_party/webkit".

This is a clever UI, but I dislove it so much. It leaves no trace of what actually happened - until some error happens of course - then it spews garbage everywhere.

I've realized at this point that at least _some_ of the errors I saw earlier where actually resolved by retries - those repos jiri was downloading do exist locally. But jiri sure made it look like a catastrophe.

So this time jiri has been running for a few minutes and it's now doing some deep thinking, looking like this

```sh
130 brian@DESKTOP-UCV672I:~/fuchsia⟫ jiri update
WARN: Please opt in or out of analytics collection. You will receive this warning until an option is selected.
To check what data we collect run 'jiri init -show-analytics-data'
To opt-in run 'jiri init -analytics-opt=true "/home/brian/fuchsia"'
To opt-out run 'jiri init -analytics-opt=false "/home/brian/fuchsia"'

Updating all projects
PROGRESS: Fetching remotes for project "third_party/crashpad"
PROGRESS: Fetching remotes for project "third_party/pixman"
PROGRESS: Fetching remotes for project "third_party/rapidjson"
PROGRESS: Fetching remotes for project "third_party/golibs/github.com/pkg/sftp"
PROGRESS: Fetching remotes for project "buildtools"
```

Presumably all or one of these repos is big and takes a long time to sync.

I've seen this "WARN" enough times that I'm about ready to do something about it. And I've again got time to kill, so let's see about this analytics stuff. At another terminal, but in my `fuchsia` directory:

```sh
brian@whatever:~/fuchsia⟫ jiri init -show-analytics-data
When opted in, jiri collects the following anonymized data in order to improve the user experience:

1. Tracks the commands that user runs.
2. Tracks the flags and their values passed with the commands. It does not track values for string flags unless they are true/false/number or following allowed values:
        - always
        - auto
        - never
3. Creates a uuid for each jiri repository and sends that to track the session and user workflow.
4. Tracks user's operating system and its architecture.
5. Tracks the time taken by a command to complete.
6. Tracks jiri version.
7. Tracks time between subsequent jiri updates if more than 30 mins and less than 2 weeks.
```

Hey that's pretty neat for Google's dev-tools team. I always wished we had that for Rust, even started implementing it once. I'm assuming that when it says "trackes the commands that user runs", it _must_ mean, "... in this jiri project", not like, everywhere.

But anyway, because I'm mildly annoyed at the moment and I make decisions with my heart-feelings instead of my brain-thoughts, I'm going to say "fuck you, Google, and fuck your analytics":

TODO reconsider the above

```sh
jiri init -analytics-opt=false "/home/brian/fuchsia"
```

Hm, that didn't really provided any particular catharsis. I'm still waiting on `jiri update` to do its jirithing.

I'm going to go ahead and jill jiri and see if 1) it stops showing that annoying analytics message, and 2) it recovers from whatever it was in the middle of.

(I won't bother to show that command again - it was just Ctrl-C and `jiri update`).

As hoped, it doesn't bother me about analytics, and it continues displaying a rotating list of exactly 5 "PROGRES" messages.

Oh, god I'm so bored.

I've written over 6000 words and have not yet built a working Fuchsia, let alone run a line of Rust code. This is so lame. I am regret.

Let's go get wasted for a bit. BBL.




## Does yoga improve jiri's success rate?

So instead of unproductively getting wasted, I went to a yoga class and came back, hoping against hope that jiri would have provided me a functional checkout. Instead I see (prepare to scroll)

```sh
Updating all projects
ERROR: exit status 1

Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon

ERROR: exit status 1

Wait for 5s before next attempt...: running hook(cipd-update) for project garnet

Attempt 2/3: running hook(download-prebuilt) for project zircon

Attempt 2/3: running hook(cipd-update) for project garnet

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk-update) for project topaz

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(update) for project buildtools

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk) for project topaz

Attempt 2/3: running hook(prebuilt-dart-sdk-update) for project topaz

Attempt 2/3: running hook(update) for project buildtools

Attempt 2/3: running hook(prebuilt-dart-sdk) for project topaz

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon

Attempt 3/3: running hook(download-prebuilt) for project zircon

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk-update) for project topaz

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(update) for project buildtools

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk) for project topaz

Attempt 3/3: running hook(prebuilt-dart-sdk-update) for project topaz

Attempt 3/3: running hook(update) for project buildtools

Attempt 3/3: running hook(prebuilt-dart-sdk) for project topaz

ERROR: "running hook(download-prebuilt) for project zircon" failed 3 times in a row, Last error: context deadline exceeded
Error for hook(download-prebuilt) for project "zircon"
######################################################################## 100.0%
Error: resolving package: failed to resolve package version (line 56): prefix "fuchsia_internal/firmware/intel-adsp-sst" doesn't exist or the caller is not allowed to see it.
/home/brian/fuchsia/zircon/scripts/download-prebuilt: /home/brian/fuchsia/zircon/../buildtools/cipd failed.  For direct downloads, remove cipd from PATH.
[P7243 16:38:53.081 storage.go:239 W] cipd: transient error fetching the file: unexpected EOF

output for hook(download-prebuilt) for project "zircon"

ERROR: "running hook(update) for project buildtools" failed 3 times in a row, Last error: context deadline exceeded
Error for hook(update) for project "buildtools"
######################################################################## 100.0%

output for hook(update) for project "buildtools"
Bootstrapping cipd client for linux-amd64...

ERROR: "running hook(prebuilt-dart-sdk) for project topaz" failed 3 times in a row, Last error: context deadline exceeded
Error for hook(prebuilt-dart-sdk) for project "topaz"

output for hook(prebuilt-dart-sdk) for project "topaz"

ERROR: 'git fetch -p origin' failed:
fatal: unable to access 'https://fuchsia.googlesource.com/third_party/boringssl/': gnutls_handshake() failed: The TLS connection was non-properly terminated.

command fail error: exit status 128

Wait for 5s before next attempt...: Fetching for /home/brian/fuchsia/third_party/boringssl

ERROR: 'git fetch -p origin' failed:
fatal: unable to access 'https://dart.googlesource.com/utf.git/': gnutls_handshake() failed: The TLS connection was non-properly terminated.

command fail error: exit status 128

Wait for 5s before next attempt...: Fetching for /home/brian/fuchsia/third_party/dart/third_party/pkg/utf

ERROR: 'git fetch -p origin' failed:
fatal: unable to access 'https://fuchsia.googlesource.com/third_party/googleapis/': gnutls_handshake() failed: The TLS connection was non-properly terminated.

command fail error: exit status 128

Wait for 5s before next attempt...: Fetching for /home/brian/fuchsia/third_party/googleapis

Attempt 2/3: Fetching for /home/brian/fuchsia/third_party/googleapis

Attempt 2/3: Fetching for /home/brian/fuchsia/third_party/boringssl

Attempt 2/3: Fetching for /home/brian/fuchsia/third_party/dart/third_party/pkg/utf

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(update) for project buildtools

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk) for project topaz

Attempt 2/3: running hook(update) for project buildtools

Attempt 2/3: running hook(download-prebuilt) for project zircon

Attempt 2/3: running hook(prebuilt-dart-sdk) for project topaz

ERROR: context deadline exceeded
Wait for 5s before next attempt...: running hook(update) for project buildtools

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk) for project topaz

Attempt 3/3: running hook(update) for project buildtools

Attempt 3/3: running hook(download-prebuilt) for project zircon

Attempt 3/3: running hook(prebuilt-dart-sdk) for project topaz

ERROR: "running hook(update) for project buildtools" failed 3 times in a row, Last error: context deadline exceeded
Error for hook(update) for project "buildtools"

output for hook(update) for project "buildtools"

ERROR: "running hook(download-prebuilt) for project zircon" failed 3 times in a row, Last error: context deadline exceeded
Error for hook(download-prebuilt) for project "zircon"

output for hook(download-prebuilt) for project "zircon"

ERROR: "running hook(prebuilt-dart-sdk) for project topaz" failed 3 times in a row, Last error: context deadline exceeded
Error for hook(prebuilt-dart-sdk) for project "topaz"

output for hook(prebuilt-dart-sdk) for project "topaz"

ERROR: Hooks execution failed.
```

Ong namo, guru dev namo.

TODO: reconsider the above

Let's dig. So the first error reported is

```sh
Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon
```

What`s 'download-prebuilt'?

```sh
brian@whatever:~/fuchsia⟫ rg "download-prebuilt" zircon
zircon/manifest/minimal
10:    <hook name="download-prebuilt"
12:          action="scripts/download-prebuilt"/>

zircon/prebuilt/zircon.ensure
5:# This file is used by scripts/download-prebuilt to control cipd.  When
18:# be kept in lock-step so that scripts/download-prebuilt gets the same

zircon/docs/getting_started.md
59:./scripts/download-prebuilt

zircon/docs/qemu.md
14:./scripts/download-prebuilt

zircon/docs/static_analysis.md
14:./scripts/download-prebuilt

zircon/scripts/download-prebuilt
237:PREBUILT_CHECK := $(shell $(LKMAKEROOT)/scripts/download-prebuilt --verify)
241:$(warning run scripts/download-prebuilt)

zircon/scripts/travis-builds.sh
14:./scripts/download-prebuilt

zircon/docs/ddk/firmware.md
77:scripts/download-prebuilt && scripts/update-prebuilt-versions
88:`scripts/download-prebuilt` along with the toolchain and QEMU.
95:scripts/download-prebuilt --no-cipd
```

This looks useful. From the first few hits I take it that zircon (the kernel remember, and components close to the kernel) has a manifest (a "jiri manifest" perhaps?) that defines "download-prebuilt"; I also see that zircon has its own "docs" folder that mentions "download-prebuilt".

Those docs say that the "download-prebuilt" script downloads prebuilt toolchains for Linux and macOS. I'm going to just run the zircon `script download-prebuilt` script on its own and see what happens.

```sh
brian@whatever:~/fuchsia/zircon⟫ scripts/download-prebuilt
```

No output. Success. Ok, I guess I misunderstood the error output. This stupid output is filled with statements like "Attempt 2/3"; maybe this command failed once or twice and finally succeeded. Maybe the whole problem is that these setup scripts can't cope with my flaky Wi-Fi (Comcast Xfinity fuck you fuck you fuck you fuck you fuck you fuck you fuck you fuck you fuck).

The error output is filled with "context deadline exceeded" messages. I wonder if I can increase the "context deadline".




## How I got Fuchsia downloaded and built

After some sleep I got ahead of myself and stopped writing down the commands I was issuing. In the meantime I succeeded at building and running Fuchsia under QEMU. So now I'm going to try to reconstruct those events.

After resting my mind and body, I ran `jiri init` again and it worked. IDK. I guess what you should take from this is "run `jiri init` until it succeeds fully".

That's it.

After that I used the `fx` command to build Fuchsia; the build lasted a long time but it worked. I forget the crucial details. Sorry. don't worry I'll capture them for you later. It'll all make sense.




## Updating Fuchsia

September 1, 2018.

That was all many weeks ago. I forgot to tell you that I successfully ran QEMU. It's a neat experience building and booting a new operating system for the first time. I forget how to do it now anyway, so I'm going to figure it out again.

First though I need to update the fuchsia source.

I read the [jiri readme][jiri]


```
brian@whatever:~/fuchsia⟫ jiri update
Updating all projects.
```

After a while it then prints

```
WARN: 3 project(s) is/are marked to be deleted. Run 'jiri update -gc' to delete them.
Or run 'jiri update -v' or 'jiri status -d' to see the list of projects.

PROGRESS: Updating project "dart/sdk"
```

3 entire projects were removed since I last saw Fuchsia. I don't bother to read the rest of the warning, "Run jiri update blah blah whatever".

After that the output stops scrolling, each time it updates a project
jiri replaces the last line with the latest status.

That is, the status is updated in place on the console, instead of on a new line; the console doesn't scroll. This has been a trend in console dev tools in recent years, and I find myself liking it. It's more peaceful than the constant movement of scrolling status updates.

In the meantime the output of jiri has turned to

```
brian@whatever:~/fuchsia⟫ jiri update
Updating all projects
WARN: 3 project(s) is/are marked to be deleted. Run 'jiri update -gc' to delete them.
Or run 'jiri update -v' or 'jiri status -d' to see the list of projects.

ERROR: 'git clone --no-checkout https://fuchsia.googlesource.com/third_party/rust-mirrors/rust-crypto /home/brian/fuchsia/third_party/rust-mirrors/rust-crypto' failed:
Cloning into '/home/brian/fuchsia/third_party/rust-mirrors/rust-crypto'...
error: RPC failed; curl 56 GnuTLS recv error (-110): The TLS connection was non-properly terminated.
fatal: The remote end hung up unexpectedly
fatal: early EOF
fatal: index-pack failed

command fail error: exit status 128

Wait for 5s before next attempt...: Cloning https://fuchsia.googlesource.com/third_party/rust-mirrors/rust-crypto

ERROR: 'git clone --no-checkout https://fuchsia.googlesource.com/third_party/golang/crypto /home/brian/fuchsia/third_party/golibs/golang.org/x/crypto' failed:
Cloning into '/home/brian/fuchsia/third_party/golibs/golang.org/x/crypto'...
error: RPC failed; curl 56 GnuTLS recv error (-9): A TLS packet with unexpected length was received.
fatal: The remote end hung up unexpectedly
fatal: early EOF
fatal: index-pack failed

command fail error: exit status 128

Wait for 5s before next attempt...: Cloning https://fuchsia.googlesource.com/third_party/golang/crypto

ERROR: 'git clone --no-checkout https://fuchsia.googlesource.com/tonic /home/brian/fuchsia/third_party/tonic' failed:
Cloning into '/home/brian/fuchsia/third_party/tonic'...
error: RPC failed; curl 56 GnuTLS recv error (-9): A TLS packet with unexpected length was received.
fatal: The remote end hung up unexpectedly
fatal: early EOF
fatal: index-pack failed

command fail error: exit status 128

Wait for 5s before next attempt...: Cloning https://fuchsia.googlesource.com/tonic

ERROR: 'git clone --no-checkout https://chromium.googlesource.com/external/github.com/dart-lang/test.git /home/brian/fuchsia/third_party/dart/third_party/pkg/unittest' failed:
Cloning into '/home/brian/fuchsia/third_party/dart/third_party/pkg/unittest'...
error: RPC failed; curl 56 GnuTLS recv error (-110): The TLS connection was non-properly terminated.
fatal: The remote end hung up unexpectedly
fatal: early EOF
fatal: index-pack failed

command fail error: exit status 128

Wait for 5s before next attempt...: Cloning https://chromium.googlesource.com/external/github.com/dart-lang/test.git

Attempt 2/3: Cloning https://fuchsia.googlesource.com/third_party/rust-mirrors/rust-crypto

Attempt 2/3: Cloning https://chromium.googlesource.com/external/github.com/dart-lang/test.git

Attempt 2/3: Cloning https://fuchsia.googlesource.com/third_party/golang/crypto

Attempt 2/3: Cloning https://fuchsia.googlesource.com/tonic

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk-update) for project topaz

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(update) for project buildtools

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(cipd-update) for project garnet

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk) for project topaz

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(download-libwebkit) for project topaz

Attempt 2/3: running hook(download-prebuilt) for project zircon

Attempt 2/3: running hook(prebuilt-dart-sdk-update) for project topaz

Attempt 2/3: running hook(update) for project buildtools

Attempt 2/3: running hook(prebuilt-dart-sdk) for project topaz

Attempt 2/3: running hook(cipd-update) for project garnet

Attempt 2/3: running hook(download-libwebkit) for project topaz

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(download-prebuilt) for project zircon

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk-update) for project topaz

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(update) for project buildtools

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(prebuilt-dart-sdk) for project topaz

ERROR: context deadline exceeded

Wait for 5s before next attempt...: running hook(cipd-update) for project garnet

Attempt 3/3: running hook(download-prebuilt) for project zircon

Attempt 3/3: running hook(prebuilt-dart-sdk-update) for project topaz

Attempt 3/3: running hook(prebuilt-dart-sdk) for project topaz

Attempt 3/3: running hook(cipd-update) for project garnet

Attempt 3/3: running hook(update) for project buildtools
```

and it's just hanging there, and it's been hanging exactly there for many minutes. If only I had set a timer. It's 1:19 now, and I _feel_ like it's been at least 10 minutes, and probably more, since I ran `jiri update`. I'm going to let it keep going until it decides it has done everything it possibly can.

All these errors pretty much ruin the aesthetics of the non-scrolling output, since it's all pukey-scrolly now. I wonder briefly whether there is a tool that would let me pipe the errors to a different tmux window. I try to stop wondering about this and remind myself to go check on the build again.

Unexpectedly, it's only 1:23 until jiri continues onward. It spews more errors and as of 1:30 jiri is sitting again, patiently doing whatever it's doing behind a screenful of errors.

These look like network errors, so I tested that the internet was working, several times. Maybe it's less of a network thing and more a [WSL] thing; but I doubt jiri is doing anything particularly weird wrt networking syscalls.

As of 1:34 jiri has printed more red errors and is now peacefully back to displaying green progress:

```
PROGRESS: running hook(prebuilt-dart-sdk) for project "topaz"
PROGRESS: running hook(update) for project "buildtools"
```

I recall that the last time I built Fuchsia, earlier in this draft &mdash; really the entirety of this draft to this point is about fumbling with networking errors while trying to download the source code. That's like, step1. Nearing 9000 words (most of them from jiri ofc) and I'm on step 1. This is the most boring story I've ever written. It better get better fast; I mean, it would be nice if I progressed beyond downloading the Fuchsia source code in this fantastical endeavor.

Finally an exhausted and disheartened jiri conceeds to the unending tide of errors, slumps over, releases the console, and with her final breath whispers

```
ERROR: Hooks execution failed.
1 brian@whatever:~/fuchsia⟫
```

My notes (above) from the previous time this happened say to run `scripts/download-prebuilt`. Let's see if that helps.

```
brian@DESKTOP-UCV672I:~/fuchsia⟫ scripts/download-prebuilt
bash: scripts/download-prebuilt: No such file or directory
```

More trials. Thanks, divine wisdom &mdash; I know you do this to make me stronger *fistbump*.

So that `download-prebuilt` script seems to not exist now, removed during the update I assume. I don't know what to do, so I ask for `jiri update -v`, hoping she'll do the job this time, or if not then she will be more verbose about why she is so sad.

My expectations are low ... although ... \
I don't expect much ... but ...

```
2 brian@whatever:~/fuchsia⟫ jiri update -v
Updating all projects
DEBUG: 10.18 seconds taken for operation: Fetching for /home/brian/fuchsia/third_party/android/platform/external/aac
WARN: 3 project(s) is/are marked to be deleted. Run 'jiri update -gc' to delete them.

DEBUG: List of project(s) marked to be deleted:
Name: go_x_crypto, Path: '/home/brian/fuchsia/third_party/golang/crypto'
Name: errors, Path: '/home/brian/fuchsia/garnet/go/src/thinfs/vendor/github.com/pkg/errors'
Name: glog, Path: '/home/brian/fuchsia/garnet/go/src/thinfs/vendor/github.com/golang/glog'

DEBUG: advance/rebase project "third_party/dart/third_party/pkg/args" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/args" to "1.4.4"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/async" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/async" to "2.0.8"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/bazel_worker" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/bazel_worker" to "0.1.11"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/boolean_selector" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/boolean_selector" to "1.0.4"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/charcode" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/charcode" to "v1.1.2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/collection" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/collection" to "1.14.11"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/convert" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/convert" to "2.0.2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/crypto" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/crypto" to "2.0.6"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/csslib" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/csslib" to "0.14.4+1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/dart2js_info" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/dart2js_info" to "0.5.6+4"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/dartdoc" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/dartdoc" to "v0.20.2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/fixnum" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/fixnum" to "0.10.8"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/glob" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/glob" to "1.1.7"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/html" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/html" to "0.13.3+2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/http" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/http" to "0.11.3+1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/http_multi_server" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/http_multi_server" to "2.0.5"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/http_parser" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/http_parser" to "3.1.1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/http_retry" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/http_retry" to "0.1.1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/http_throttle" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/http_throttle" to "1.0.2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/intl" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/intl" to "0.15.6"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/json_rpc_2" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/json_rpc_2" to "2.0.9"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/linter" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/linter" to "0.1.59"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/logging" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/logging" to "0.11.3+2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/markdown" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/markdown" to "2.0.2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/matcher" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/matcher" to "0.12.3"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/mime" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/mime" to "0.9.6+2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/mustache4dart" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/mustache4dart" to "v2.1.2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/oauth2" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/oauth2" to "1.2.1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/path" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/path" to "1.6.2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/pool" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/pool" to "1.3.6"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/protobuf" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/protobuf" to "0.9.0"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/pub_semver" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/pub_semver" to "1.4.2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/quiver" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/quiver" to "0.29.0+2"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/resource" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/resource" to "2.1.5"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/shelf" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/shelf" to "0.7.3+3"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/shelf_packages_handler" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/shelf_packages_handler" to "1.0.4"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/shelf_static" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/shelf_static" to "v0.2.8"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/shelf_web_socket" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/shelf_web_socket" to "0.2.2+3"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/source_map_stack_trace" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/source_map_stack_trace" to "1.1.5"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/source_span" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/source_span" to "1.4.1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/stack_trace" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/stack_trace" to "1.9.3"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/stream_channel" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/stream_channel" to "1.6.8"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/string_scanner" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/string_scanner" to "1.0.3"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/term_glyph" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/term_glyph" to "1.0.1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/test" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/test" to "1.0.0"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/test_descriptor" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/test_descriptor" to "1.1.1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/test_process" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/test_process" to "1.0.3"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/test_reflective_loader" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/test_reflective_loader" to "0.1.8"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/tuple" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/tuple" to "v1.0.1"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/typed_data" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/typed_data" to "1.1.6"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/usage" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/usage" to "3.4.0"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/utf" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/utf" to "0.9.0+5"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/watcher" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/watcher" to "0.9.7+10"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/web_socket_channel" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/web_socket_channel" to "1.0.9"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg/yaml" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg/yaml" to "2.1.15"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg_tested/dart_style" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg_tested/dart_style" to "1.1.3"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg_tested/package_config" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg_tested/package_config" to "1.0.5"
DEBUG: advance/rebase project "third_party/dart/third_party/pkg_tested/package_resolver" located in "/home/brian/fuchsia/third_party/dart/third_party/pkg_tested/package_resolver" to "1.0.4"
DEBUG: running hook(update) for project "buildtools"
DEBUG: running hook(cipd-update) for project "garnet"
DEBUG: running hook(install-fx) for project "scripts"
DEBUG: running hook(prebuilt-dart-sdk) for project "topaz"
DEBUG: running hook(cipd-update) for project "topaz"
DEBUG: running hook(symlink-.gn) for project "build"
DEBUG: running hook(download-prebuilt) for project "zircon"
DEBUG: running hook(ffmpeg-update) for project "garnet"
DEBUG: running hook(download-vulkan-sdk) for project "garnet"
DEBUG: running hook(fonts-update) for project "garnet"
DEBUG: running hook(download-libwebkit) for project "topaz"
DEBUG: running hook(prebuilt-dart-sdk-update) for project "topaz"
DEBUG: output for hook(install-fx) for project "scripts"

DEBUG: output for hook(symlink-.gn) for project "build"

DEBUG: output for hook(cipd-update) for project "topaz"

DEBUG: output for hook(ffmpeg-update) for project "garnet"

DEBUG: output for hook(download-vulkan-sdk) for project "garnet"

DEBUG: output for hook(download-libwebkit) for project "topaz"

DEBUG: output for hook(cipd-update) for project "garnet"

DEBUG: output for hook(fonts-update) for project "garnet"

DEBUG: output for hook(prebuilt-dart-sdk-update) for project "topaz"
Packages (subdir "topaz/tools/prebuilt-dart-sdk/linux-x64"):
  fuchsia/dart-sdk/linux-amd64:505bf8dc284ac4c5f5491d548a1d66d850ff05c9 (for linux-amd64)
  fuchsia/dart-sdk/mac-amd64:a459812a64a4675ca593dab0470c4a20c1c289e2 (for mac-amd64)

DEBUG: output for hook(update) for project "buildtools"

DEBUG: output for hook(download-prebuilt) for project "zircon"

DEBUG: output for hook(prebuilt-dart-sdk) for project "topaz"
Downloading prebuilt Dart SDK from: http://gsdview.appspot.com/dart-archive/channels/dev/raw/2.0.0-dev.65.0/sdk/dartsdk-linux-x64-release.zip

brian@whatever:~/fuchsia
```

It worked. Prayer worked. If you pray, jiri will pay. That's the lesson!

From my notes of my previous trail:

> I guess what you should take from this is "run `jiri init` until it succeeds fully".

Yes, yes, and yes. Let it be known.




## Building Fuchsia!

We are so close, I promise. I'm going to build it. I'm gonna do it.

From the Fuchsia [build instructions][build], I know that I build
with the `fx` command.

[build]: https://fuchsia.googlesource.com/docs/+/HEAD/getting_started.md#build

```
brian@DESKTOP-UCV672I:~/fuchsia⟫ fx set x64
+ /home/brian/fuchsia/buildtools/gn gen /home/brian/fuchsia/out/x64 --check '--args=target_cpu="x64" fuchsia_products=["topaz/products/default",] fuchsia_packages=[]'
```

That `fx set x64` configures our target platform.

This command seems to be taking a long time too. It feels like it should not; it's just setting a thing. This isn't even the build command. What is going on?

I wonder if maybe I'm low on disk space and the filesystem is having a hard time, so at another terminal I run `df`:

```
brian@whatever:~/fuchsia⟫ df -h
Filesystem      Size  Used Avail Use% Mounted on
rootfs          476G  435G   42G  92% /
root            476G  435G   42G  92% /root
home            476G  435G   42G  92% /home
data            476G  435G   42G  92% /data
cache           476G  435G   42G  92% /cache
mnt             476G  435G   42G  92% /mnt
none            476G  435G   42G  92% /dev
none            476G  435G   42G  92% /run
none            476G  435G   42G  92% /run/lock
none            476G  435G   42G  92% /run/shm
none            476G  435G   42G  92% /run/user
C:              476G  435G   42G  92% /mnt/c
```

42GBs that's fine right? I'm not confident, so I go on a tangent looking for big files and eventually google my way to a really slick unixy command for finding and sorting large files.

```
brian@whatever:~⟫ find . -type f -size +20M -exec ls -lh {} \; 2> /dev/null | awk '{ print $NF ": " $5 }' | sort -nk 2,2
```

See how unixy that is? I doubt I would ever figure that out myself. I note that it would be beneficial for me to learn awk.

Here's the output of the search:

```
./fuchsia/out/x64/obj/build/images/blob.blk: 1.3G
./fuchsia/out/x64/images/fvm.blk: 1.5G
./fuchsia/out/x64/obj/build/images/fvm.blk: 1.5G
./bloop-language/target/debug/deps/libclap-642ba2712e00e69b.rlib: 21M
./bloop-language/target/debug/deps/libclap-93bda76bb4db141b.rlib: 21M
./comrak2/target/debug/deps/libclap-85b07ad4ac8494d2.rlib: 21M
./comrak/target/debug/deps/libclap-087577ec52921025.rlib: 21M
./fuchsia/.jiri_root/bin/jiri: 21M
./fuchsia/out/x64/gen/topaz/bin/user_shell/capybara_user_shell/capybara_user_shell_kernel.dil: 21M
./fuchsia/out/x64/rust_third_party/x86_64-unknown-fuchsia/debug/deps/libclap-26785f0b1ae2c7f1.rlib: 21M
./fuchsia/third_party/boringssl/crypto_test_data.cc: 21M
./fuchsia/third_party/catapult/third_party/vinn/third_party/v8/mac/x86_64/d8: 21M
./grin/target/debug/deps/libgrin_p2p-ca1ac2ce256aa679.rlib: 21M
./reddit-dev/reddit/deb-cache/libpython2.7-dev_2.7.6-8ubuntu0.4_amd64.deb: 21M
./reddit/puppet/.git/objects/pack/pack-37869d5cd6d13a914da34585a1e0dc019481d807.pack: 21M
./reddit/snoomark2/rtj-service/target/debug/deps/libclap-e35ddd5debd495f4.rlib: 21M
./reddit/snoomark2/target/debug/deps/libclap-2b3a12f6e9325d57.rlib: 21M
./reddit/snoomark2/target/debug/deps/libclap-6bacc5c5bbbfdefb.rlib: 21M
./reddit/snoomark2/target/debug/deps/libclap-8d554c82519896d8.rlib: 21M
./reddit/snoomark2/target/debug/deps/libclap-9aa7c1ba1604c0e4.rlib: 21M
./reddit/snoomark2/target/debug/deps/libclap-bbef7b06903dd31c.rlib: 21M
./reddit/snoomark2/target/debug/deps/libclap-e8442faa50132aed.rlib: 21M
./reddit/snoomark/target/debug/deps/libclap-2b3a12f6e9325d57.rlib: 21M
./reddit/snoomark/target/debug/deps/libclap-8d554c82519896d8.rlib: 21M
./reddit/snoomark/target/debug/deps/libclap-9aa7c1ba1604c0e4.rlib: 21M
./reddit/snoomark/target/debug/deps/libclap-bbef7b06903dd31c.rlib: 21M
./rust/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f21uqpog0w-8s65iu-13fy2x5vnqdla/query-cache.bin: 21M
./rust/build/bootstrap/debug/incremental/bootstrap-ck2y1bch9wb2/s-f05tsvz1bo-14js4qi-31chhg6u1p0gw/dep-graph.bin: 21M
./rust/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86/X86GenDAGISel.inc: 21M
./rust/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86/X86GenDAGISel.inc.tmp: 21M
./rust/.git/objects/pack/pack-86ff834a90445069d2117ad76c1dbd3c82a6d9bf.idx: 21M
./rust/.git/objects/pack/pack-8cc07db66a446297fed97b35df2ae318fc3bbe23.pack: 21M
./rustup.rs/target/debug/deps/libclap-9aa7c1ba1604c0e4.rlib: 21M
./solana/target/debug/deps/libclap-7a6862873e5b7a73.rlib: 21M
./solana/target/debug/deps/libclap-e187a8f718ab9922.rlib: 21M
./solana/target/debug/deps/liblog4rs-c8cd98c60e1a3d1c.rlib: 21M
./solana/target/debug/deps/liblog4rs-f2295f80428de341.rlib: 21M
./comrak2/target/debug/deps/libclap-1f1ff992d845c4ed.rlib: 22M
./comrak/target/debug/deps/libclap-b00a46bef1db3da7.rlib: 22M
./comrak/target/debug/deps/libclap-b54cd5b4d35acef6.rlib: 22M

<... snip ...>

./reddit/snoomark2/target/debug/incremental/comrak-1dvftsl502hmr/s-f2ti8rtf67-1vtztv0-22ij93rfwm62o/query-cache.bin: 308M
./reddit/snoomark2/target/debug/incremental/comrak-1f5kpi9ev4v9/s-f2uds3m882-w949uv-1v1qxuckedgk3/query-cache.bin: 308M
./reddit/snoomark2/target/debug/incremental/comrak-2os8d6ghm52ha/s-f2udqwi0yc-1p22rms-35uyxcijuh8d7/query-cache.bin: 308M
./reddit/snoomark2/target/debug/incremental/comrak-p4djn8tt6mrr/s-f2z5z5c2sj-w2v3e3-395pmczcupyon/query-cache.bin: 308M
./reddit/snoomark2/target/debug/incremental/comrak-nfu0qxsardc5/s-f2ti8rtme2-1adusi0-la2c0042lzew/query-cache.bin: 309M
./reddit/snoomark2/target/debug/incremental/snoomark-3rlzha3opqmvy/s-f2ewqcbp2k-ggcz15-1kghv28sn3b1e/query-cache.bin: 309M
./fuchsia/third_party/catapult/.git/objects/pack/pack-9c776ecf9afa27d5164f3f383b63c09a0e1ecdfa.pack: 310M
./reddit/snoomark2/rtj-service/target/debug/incremental/snoomark-351mrykj237p5/s-f4cdnb4jxq-1u08alb-121lvjulum485/query-cache.bin: 310M
./reddit/snoomark2/target/debug/incremental/snoomark-2lny9oaokvoi9/s-f49916950r-gsjuqc-27m41952ydpg2/query-cache.bin: 310M
./reddit/snoomark/fuzz/target/x86_64-unknown-linux-gnu/debug/incremental/snoomark-1il0pwbdvuor7/s-f1bclec5t4-tqvsay-1wcfwigr2g7bu/query-cache.bin: 310M
./reddit/snoomark/target/debug/incremental/snoomark-mssirmb2j5ux/s-f37l4qp8bs-1byp3ny-1qodvxommf08u/query-cache.bin: 310M
./reddit/snoomark/target/debug/incremental/snoomark-mssirmb2j5ux/s-f37pr1qki0-1sw0zon-working/query-cache.bin: 310M
./reddit/snoomark2/target/debug/incremental/snoomark-2c8i66fjw4d2o/s-f302velifk-anr8gz-93tynk2hnpj9/query-cache.bin: 311M
./reddit/snoomark2/target/debug/incremental/snoomark-3peiogbpzneq5/s-f36qu4lqtj-1jf2xul-6qavcevtqex0/query-cache.bin: 311M
./reddit/snoomark2/target/debug/incremental/snoomark-mssirmb2j5ux/s-f3gjzuhi2s-1uoo12x-dukouuqbc5dj/query-cache.bin: 311M
./reddit/snoomark/target/debug/incremental/comrak-fegok7soj0cc/s-f2muyzpp7h-1esuadl-2i8blj6npawjt/query-cache.bin: 311M
./reddit/snoomark/target/debug/incremental/snoomark-1zibhrvjg5e81/s-f42neuadk8-1xv3aaf-2ufkkgbaa52tu/query-cache.bin: 311M
./reddit/snoomark/target/debug/incremental/snoomark-2lny9oaokvoi9/s-f42ne60hlz-i8kikm-3hjklaqvokowp/query-cache.bin: 311M
./reddit/snoomark/target/debug/incremental/snoomark-3peiogbpzneq5/s-f36qxu6ck4-tycdb7-2ynwgu9e53yyy/query-cache.bin: 311M
./comrak2/target/debug/incremental/comrak-1nzpqwh5fnz2d/s-f1qv9rj72h-jtbrus-2jsaawvk23tmy/query-cache.bin: 313M
./reddit/snoomark/target/debug/incremental/comrak-35sdnxlqch2y/s-f2muyzthge-13k7rrl-13ho49lts72dd/query-cache.bin: 313M
./reddit/snoomark2/fuzz/target/x86_64-unknown-linux-gnu/debug/incremental/snoomark-351mrykj237p5/s-f3octywioh-qs2gfk-2m7syokmb9ezb/query-cache.bin: 314M
./reddit/snoomark2/target/debug/incremental/snoomark-27b6k0cqtpgzg/s-f49alj7io9-1o1u6gw-ggfujnnh9qpe/query-cache.bin: 314M
./reddit/snoomark2/target/debug/incremental/snoomark-2y93i1jopyyzq/s-f2g8423w3w-v2rl0p-16n38k7gmt2s5/query-cache.bin: 314M
./reddit/snoomark2/target/debug/incremental/snoomark-1zibhrvjg5e81/s-f3n47pt4g2-45ix12-1nuyvq7mnqce7/query-cache.bin: 315M
./reddit/snoomark2/target/debug/incremental/snoomark-bmwkdgscym5v/s-f49amaf2nu-12vw0r6-2uaumw0ipp8ew/query-cache.bin: 315M
./reddit/snoomark/target/debug/incremental/snoomark-27b6k0cqtpgzg/s-f43kaku5t0-12tqe9w-2n8id6zebutq8/query-cache.bin: 315M
./reddit/snoomark/target/debug/incremental/snoomark-oxeklu3wkqvh/s-f2nuz3x2mv-uhqnrp-ukm753fin8w6/query-cache.bin: 315M
./fuchsia/out/x64/obj/third_party/shaderc/third_party/spirv-tools/libspirv-tools-opt.a: 320M
./fuchsia/third_party/skia/.git/objects/pack/pack-269284a4dcd717a1364577a6dcad4e3c4d65f6f4.pack: 320M
./fuchsia/out/x64/netboot.bin: 337M
./fuchsia/out/x64/netboot.zbi: 337M
./rust/.git/objects/pack/pack-86ff834a90445069d2117ad76c1dbd3c82a6d9bf.pack: 341M
./reddit-dev/desktop/.git/objects/pack/pack-ae42ac4e5770dcc81906dbeb53bdf099dd32be03.pack: 343M
./fuchsia/out/x64/images/fvm.sparse.blk: 371M
./fuchsia/out/x64/obj/build/images/fvm.sparse.blk: 371M
./grin/target/debug/build/librocksdb-sys-133d23772591e1bb/out/librocksdb.a: 371M
./grin/target/debug/deps/liblibrocksdb_sys-3782a9229d8ec248.rlib: 372M
./fuchsia/third_party/llvm/.git/objects/pack/pack-81339690d4ffb90eb9f7000c5e5bd2e8206f91a3.pack: 397M
./fuchsia/out/x64/obj/build/images/installer.blk: 409M
./rust/.git/modules/src/tools/clang/objects/pack/pack-cf2080643014c82aef34ca80379ef7f66d4c4945.pack: 411M
./tikv/target/debug/build/librocksdb_sys-2773796fb7204efb/out/build/librocksdb.a: 412M
./fuchsia/third_party/dart/.git/objects/pack/pack-5c5845fed51f830a3a83482cb6e37f5fe4d72783.pack: 649M
./rust/.git/modules/src/llvm/objects/pack/pack-9d2ee9f04de135ef2683e4db4edf65e01023cace.pack: 740M
./rust/.git/modules/src/llvm-emscripten/objects/pack/pack-f4758c8d5f35a6ba9063ac273984b5f5ca35306e.pack: 741M
./reddit/cm-examples/deep-escape-1000000000.md: 954M
```

It's almost all Rust stuff naturally, and there's so much interesting here:

- The command didn't successfully sort by filesize, as the 1GB+ files are at the very top, and the biggest <1GB files are at the bottom.
- The only 3 1GB+ files are from Fuchsia, totalling 4.3 GB. From my previous build.
- The [RocksDB] binary is huge.
- LLVM bins are huge. No surprise.
- Incremental builds leave behind massive "query caches". I don't know what those are but I bet I can delete a bunch of them.
- I once started writing a language called "bloop", and then shelved it after 1 day. As I appear to do with most of my projects.

[RocksDB]: https://rocksdb.org/

I delete those old Fuchsia files, and a variety of others. Running `rm fuchsia/out -rf` takes suspiciously long, and I wonder why my disks are so slow. Maybe it's a [WSL] thing, being backed (I think) by NTFS. Maybe it's just my underpowered laptop. During the long, peaceful time that `rm fuchsia/out -rf` is running, I wonder if deleting the entire `out` directory was the right thing to do. Maybe `fx set x64` put stuff there &emph; maybe it even created one of those 1GB+ files! I have many doubts, and many trains of thought, and this process of writing prose and running commands is becoming slower and slower.

4 minutes after running `fx set x64`, `fx` says

```
Done. Made 9420 targets from 1160 files in 225077ms
brian@whatever:~/fuchsia⟫
```

Great. Now we can run the build command.

```
brian@whatever:~/fuchsia⟫ fx build
Build directory problem, args.gn is missing.
Did you "rm -rf out" and not rerun "fx set"?
```

Yeah. Yeah, I did that to myself in this comedy of errors. I travel back in time to the moment just before I deleted the entire build directory and make smarter decisions.

I run `fx set x64 once again`.

```
brian@whatever:~/fuchsia⟫ time fx set x64
+ /home/brian/fuchsia/buildtools/gn gen /home/brian/fuchsia/out/x64 --check '--args=target_cpu="x64" fuchsia_products=["topaz/products/default",] fuchsia_packages=[]'
```

But this time I run it with the `time` command to see how long it takes. It's 2:49 PM.

```
brian@whatever:~/fuchsia⟫ time fx set x64
+ /home/brian/fuchsia/buildtools/gn gen /home/brian/fuchsia/out/x64 --check '--args=target_cpu="x64" fuchsia_products=["topaz/products/default",] fuchsia_packages=[]'
Done. Made 9420 targets from 1160 files in 269993ms

real    4m30.268s
user    0m9.328s
sys     4m14.156s
```

Only 4 and a half minutes.




## No, no, I'm totally serious &mdash; now I'm going to build Fuchsia

```
brian@whatever:~/fuchsia⟫ time fx full-build
Building zircon...
```

It's 3:06 PM.




## The zircon boot sequence

While I wait I read ["Zircon kernel to userspace bootstrapping (userboot)"][userboot]. It's a clever boot sequence and it makes me want to dig into that code. After thet bootloader loads the kernel, the kernel quickly hands the boot process over to a regular process called 'userboot', that is itself baked into the kernel image.

[userboot]: https://fuchsia.googlesource.com/zircon/+/master/docs/userboot.md

I explore the ['zircon' directory][zd] under 'fuchsia', and the ['kernel' directory][kd] under that. There's some interesting directories here. I'm looking for the bootstrap code, the kernel entry point, and the ['top' directory][td] looks promising, 'top' being the very first code run if you're looking at it top-down.

[zd]: https://fuchsia.googlesource.com/zircon/+/6b397a66c8a36b7d4488a8c965b518145942fe11/
[kd]: https://fuchsia.googlesource.com/zircon/+/6b397a66c8a36b7d4488a8c965b518145942fe11/kernel/
[td]: https://fuchsia.googlesource.com/zircon/+/6b397a66c8a36b7d4488a8c965b518145942fe11/kernel/top/

The 'top' directory contains two C++ files, [`init.cpp`] and [`main.cpp`]. The names both sound like they could contain the first code run, either the "initialization" code, or the "main" function. I pick "init" and immediately recognize it is not the code I'm looking for, though it is interesting &mdash; it's a system for registering generic initialization routines; presumably used by the "main" function to initialize the kernel's subsystems.

[`init.cpp`]: https://fuchsia.googlesource.com/zircon/+/6b397a66c8a36b7d4488a8c965b518145942fe11/kernel/top/init.cpp
[`main.cpp`]: https://fuchsia.googlesource.com/zircon/+/6b397a66c8a36b7d4488a8c965b518145942fe11/kernel/top/main.cpp

It's 3:50 PM and `fx full-build` is looking like

```
brian@whatever:~/fuchsia⟫ time fx full-build
Building zircon...
ninja: Entering directory `/home/brian/fuchsia/out/x64'
[3241/24771] CXX host_x64/obj/third_party/icu/source/i18n/libicui18n.nounit.o
```

It _look_ like it's taken 44 minutes to build about 1/7.6th of Fuchsia. Will it take 44*7.6 minutes (6 hours!) to complete?

I'm getting bored. I don't know if I can follow through on this exploration of the kernel's boot sequence, but I'll at least find a good starting point. The kernel's "main" function is [right here][main], in `main.cpp`.

[main]: https://fuchsia.googlesource.com/zircon/+/6b397a66c8a36b7d4488a8c965b518145942fe11/kernel/top/main.cpp#42

This isn't actually the first code that is run &emph; that's architecture-specific, and bootloader-specific, some of it in assembly, potentially ugly stuff that would be very fun to explore; but this function is where the architecture-independent kernel begins, and it's written cleanly for good reading.

I notice that the function is not called "main", which isn't unexpected, but it is called "lk_main". What's that "lk" mean? It's explained in the Zircon docs, which I'm finding to be useful, ["Zircon and LK"][zxlk]: Zircon is based on another kernel called [LK], or littlekernel. It looks like Zircon has been modified extensively, but I expect to see thet letters "lk" a lot in Zircon.

[zxlk]: https://fuchsia.googlesource.com/zircon/+/master/docs/zx_and_lk.md
[LK]: https://github.com/littlekernel/lk

I drift off to other fancies for a while. Ta!

---

Now I've returned from yoga. I put the laptop to sleep while I was away because I'm irrationally afraid it will explode and cause a fire that will burn down my apartment building; so it's still building.

Did you look at that main function though? Look here, it's delightful.

```c++
// called from arch code
void lk_main() {
    // serial prints to console based on compile time switch
    dlog_bypass_init_early();

    // get us into some sort of thread context
    thread_init_early();

    // deal with any static constructors
    call_constructors();

    // early arch stuff
    lk_primary_cpu_init_level(LK_INIT_LEVEL_EARLIEST, LK_INIT_LEVEL_ARCH_EARLY - 1);
    arch_early_init();

    // do any super early platform initialization
    lk_primary_cpu_init_level(LK_INIT_LEVEL_ARCH_EARLY, LK_INIT_LEVEL_PLATFORM_EARLY - 1);
    platform_early_init();

    // do any super early target initialization
    lk_primary_cpu_init_level(LK_INIT_LEVEL_PLATFORM_EARLY, LK_INIT_LEVEL_TARGET_EARLY - 1);
    target_early_init();

    dprintf(INFO, "\nwelcome to Zircon\n\n");

    dprintf(INFO, "KASLR: .text section at %p\n", __code_start);

    lk_primary_cpu_init_level(LK_INIT_LEVEL_TARGET_EARLY, LK_INIT_LEVEL_VM_PREHEAP - 1);
    dprintf(SPEW, "initializing vm pre-heap\n");
    vm_init_preheap();

    // bring up the kernel heap
    lk_primary_cpu_init_level(LK_INIT_LEVEL_VM_PREHEAP, LK_INIT_LEVEL_HEAP - 1);
    dprintf(SPEW, "initializing heap\n");
    heap_init();

    lk_primary_cpu_init_level(LK_INIT_LEVEL_HEAP, LK_INIT_LEVEL_VM - 1);
    dprintf(SPEW, "initializing vm\n");
    vm_init();

    // initialize the kernel
    lk_primary_cpu_init_level(LK_INIT_LEVEL_VM, LK_INIT_LEVEL_KERNEL - 1);
    dprintf(SPEW, "initializing kernel\n");
    kernel_init();

    lk_primary_cpu_init_level(LK_INIT_LEVEL_KERNEL, LK_INIT_LEVEL_THREADING - 1);

    // create a thread to complete system initialization
    dprintf(SPEW, "creating bootstrap completion thread\n");
    thread_t* t = thread_create("bootstrap2", &bootstrap2, NULL, DEFAULT_PRIORITY);
    thread_set_cpu_affinity(t, cpu_num_to_mask(0));
    thread_detach(t);
    thread_resume(t);

    // become the idle thread and enable interrupts to start the scheduler
    thread_become_idle();
}
```

Somebody has really loved this code. I love this code &emdash; it's so readable, top down it tells you exactly what the kernel does, in short, simple, commented statements.

TODO: continue this deep dive




## Running Fuchsia

The build finishes. `time` reports that the build took

```
real    487m58.353s
user    483m26.234s
sys     83m33.063s
```

this much time. 487 minutes is about 8 hours. But I was out of the house with the loptop asleep for ~3 hours. Maybe the build took between 4 and 5 hours. It's a weak laptop, and this is an entire operating system.

Now I should be able to run Fuchsia under [QEMU]. Still following the "Fuchsia getting started" instructions, section ["Boot from QEMU"][bfqe], I just have to

[QEMU]: https://www.qemu.org/
[bfqe]: https://fuchsia.googlesource.com/docs/+/HEAD/getting_started.md#boot-from-qemu

```
brian@whatever:~/fuchsia⟫ fx run
Formatting '/tmp/tmp.97IPSYcGAg/fuchsia.qcow2', fmt=qcow2 size=1577222144 backing_file=/home/brian/fuchsia/out/x64/obj/build/images/fvm.blk cluster_size=65536 lazy_refcounts=off refcount_bits=16
CMDLINE: kernel.serial=legacy TERM=screen kernel.entropy-mixin=0f0b6f18ae9cc735c4493a5560f42684005e1238ec67f03413e6d9cb3bcc948a kernel.halt-on-panic=true
+ exec /home/brian/fuchsia/buildtools/linux-x64/qemu/bin/qemu-system-x86_64 -kernel /home/brian/fuchsia/out/build-zircon/build-x64/zircon.bin -initrd /home/brian/fuchsia/out/x64/fu
chsia.zbi -m 2048 -nographic -drive file=/tmp/tmp.97IPSYcGAg/fuchsia.qcow2,format=qcow2,if=none,id=mydisk -device ich9-ahci,id=ahci -device ide-drive,drive=mydisk,bus=ahci.0 -net n
one -smp 4,threads=2 -machine q35 -device isa-debug-exit,iobase=0xf4,iosize=0x04 -cpu Haswell,+smap,-check,-fsgsbase -append 'kernel.serial=legacy TERM=screen kernel.entropy-mixin=
0f0b6f18ae9cc735c4493a5560f42684005e1238ec67f03413e6d9cb3bcc948a kernel.halt-on-panic=true '
[00000.000] 00000.00000> multiboot: info @ 0xffffff8000009500 flags 0x24f
[00000.000] 00000.00000> multiboot: cmdline @ 0xffffff80002bf038
[00000.000] 00000.00000> multiboot: ramdisk @ 002c0000..00803c30
[00000.000] 00000.00000> zbi: @ 0xffffff80002c0000 (5520432 bytes)
[00000.000] 00000.00000> UART: FIFO depth 16
[00000.000] 00000.00000> PMM: boot reserve add [0x100000, 0x2bcfff]
[00000.000] 00000.00000> PMM: boot reserve add [0x2c0000, 0x803fff]
[00000.000] 00000.00000> PMM: boot reserve marking WIRED [0x100000, 0x2bcfff]
[00000.000] 00000.00000> PMM: boot reserve marking WIRED [0x2c0000, 0x803fff]
[00000.000] 00000.00000>
[00000.000] 00000.00000> welcome to Zircon
```

The boot log continues, until it ends like

```
[00007.281] 01045.01048> devcoord: driver 'wlantapctl' added
[00007.304] 01045.01048> devcoord: driver 'bthog' added
[00007.304] 01045.01048> devcoord: driver 'wlan' added
[00007.304] 01045.01048> devcoord: driver 'bt_passthrough_hci' added
[00007.304] 01045.01048> devcoord: driver 'intel_disp' added
[00009.542] 04452.04464> netstack: started
[00009.551] 04452.04464> netstack: socket server started
[00009.589] 04452.04666> netstack: watching for ethernet devices
[00009.596] 04452.04666> netstack: starting http pprof server on 0.0.0.0:6060
[00012.873] 05937.05949> [INFO:main.cc(52)] no kernel crash log found
[00013.380] 06169.06181> netcfg: started
[00014.304] 06309.06328> [INFO:main.cc(35)] Trace Manager starting with config: /pkg/data/tracing.config
[00014.428] 06545.06561> wlanstack2 [I]: Starting
[00014.581] 06545.06561> wlanstack2::telemetry [I]: Telemetry started
[00015.913] 03286.03344> [WARNING:garnet/bin/appmgr/service_provider_dir_impl.cc(52)] Component file://device_settings_manager is not allowed to connect to fuchsia.logger.LogSink because this service is not present in the component's sandbox.
[00015.914] 03286.03344> Refer to https://fuchsia.googlesource.com/docs/+/master/the-book/sandboxing.md#services for more information.
[00016.080] 06900.06912> [INFO:cobalt_main.cc(80)] Cobalt client schedule params: schedule_interval=180 seconds, min_interval=10 seconds.
[00019.044] 07786.07798> [INFO:hid_decoder.cc(241)] hid-parser succesful for 000 with usage page 1 and usage 128
[00020.946] 07786.07798> [INFO:presentation1.cc(102)] Disconnected from a11y toggle broadcaster.
[00021.082] 08030.08042> [INFO:display_watcher.cc(46)] Scenic: Acquired display controller /dev/class/display-controller/000.(000)
[00021.408] 03286.03678> dlsvc: could not open 'libvulkan_intel.so'
[00022.000] 08030.08042> ERROR: setupLoaderTermPhysDevs:  Failed to detect any valid GPUs in the current config
[00022.000] 08030.08042> ERROR: setupLoaderTrampPhysDevs:  Failed during dispatch call of 'vkEnumeratePhysicalDevices' to lower layers or loader to get count.
[00022.003] 08030.08042> [ERROR:garnet/lib/ui/gfx/gfx_system.cc(137)] No Vulkan on device, Graphics system exiting.
[00022.004] 08030.08042> [INFO:input_system.cc(34)] Scenic input system initialized.
[00022.027] 07786.07798> [ERROR:garnet/bin/ui/root_presenter/app.cc(247)] Scenic died, destroying view trees.
[00022.050] 07347.07364> [ERROR:garnet/bin/ui/view_manager/view_registry.cc(120)] Exiting due to session connection error.
[00022.069] 03699.03721> [ERROR:garnet/bin/sysmgr/app.cc(99)] Singleton scenic died
[00022.086] 03699.03721> [ERROR:garnet/bin/sysmgr/app.cc(99)] Singleton view_manager died

```

Those errors look bad, but they are related to graphics stuff, and graphics are not expected to work, mostly because Fuchsia's graphics stack renders through [Vulkan], and Vulkan is not supported under QEMU]

There's no prompt; it looks hung. But it's not. I press enter and the cursor moves from a blank line with a simple promt

```
[00022.086] 03699.03721> [ERROR:garnet/bin/sysmgr/app.cc(99)] Singleton view_manager died

$
```

I run a variety of commands.

```
$ dm help
dump              - dump device tree
poweroff          - power off the system
shutdown          - power off the system
suspend           - suspend the system to RAM
reboot            - reboot the system
reboot-bootloader - reboot the system into boatloader
reboot-recovery   - reboot the system into recovery
kerneldebug       - send a command to the kernel
ktraceoff         - stop kernel tracing
ktraceon          - start kernel tracing
devprops          - dump published devices and their binding properties
drivers           - list discovered drivers and their properties
$ ls
blob
boot
data
dev
hub
install
pkgfs
svc
system
tmp
volume
$ pwd
/
$ help
/boot/bin/sh: 4: help: not found
$
```

Interesting reading I guess.

I wish I knew the list of available commands. In the output of `ls`, which is printing
the contents of the root directory, I don't see any `/bin`, `/usr/bin`, etc., but
I do see `install` and `pkgfs`. My intuition tells me `pkgfs`, so I `ls /pkgfs`

```
$ ls /pkgfs
install
metadata
needs
packages
system
```

Looks like a package management system's stuff. Maybe every program is installed as a package and lives here, including basic system stuff like `cat`, `echo`, `ls`, `rm`.
Continuing browsing `/pkgfs` I find `/pkgfs/system/bin` and it's a bingo.

```
$ ls pkgfs/system/bin
amber_ctl
appmgr
aslr-analysis
audio
audio-codec
backlight
basename
biotime
blktest
bssl
bt-cli
bt-fake-hci
bt-hci-tool
bt-intel-tool
bt-le-central
bt-snoop
cal
cat
channel-perf
chrealm
cksum
cmp
cols
comm
core-tests
cowsay
cp
crasher
crashpad_database_util
cut
date
debug_agent
debugserver
dirname
display-test
driverctl
du
dump1
echo
ed
env
evil-tests
expand
expr
false
far
find
fold
fortune
gfxfractal
gfxlatency
gfxtest
google_auth_provider
grep
guest
head
hid
hidsensor
hidtouch
hostname
i2c
ifconfig
ifinfo
ihda
input
insntrace
installblob
iochk
iotime
jank_view
join
kilo
kstress
link
loadgen
log_listener
ls
lspwr
lz4
md5sum
mediaplayer_skia
midiecho
mkdir
mktemp
msd-test
mv
namespace
netfilter
netreflector
netstat
nl
oauth_token_manager
od
paint_view
paste
pathchk
pm
present_view
print_input
printenv
printf
publish-data-helper
pwd
readlink
report_result
rev
rm
rmdir
rng-trials
run
run_integration_tests
run_test_component
runtests
scp
screencap
sed
seq
serial-test
set_renderer_params
set_root_view
sftp-server
sha1sum
sha224sum
sha256sum
sha384sum
sha512-224sum
sha512-256sum
sha512sum
signal_generator
sleep
sort
spawn-child
spawn-launcher
split
sponge
spotify_auth_provider
ssh-keygen
sshd
strerror
strings
suspendtest
sync
sysconfig
sysinfo
system-version
tail
tar
tee
tee-test
test
tftp
thinfs
thread-depth-test
thread-stress-test
tiles_ctl
time
touch
tpmctl
tr
trace
trace-benchmark
trace-example
traceme
true
tsort
tty
tz-util
uname
unexpand
uniq
unlink
usb-test-fwloader
usbctl
uudecode
uuencode
vdso-variant-helper
vim
vol
vu_meter
waitfor
watch
wc
web_view_test
which
whoami
wlan
wlantool2
xargs
xdc-test
xinstall
yes
$
```

Then I recall that on Unix this folder would be in the `$PATH` variable. Does Fuchsia have such a thing?

```
$ echo $PATH
/system/apps:/system/bin:/boot/bin
```

Yes it does. And it's mercifully small. It doesn't though contain my new-found directory,
`/pkgfs/system/bin`. I suspect the same files are in both `/pkgfs/system/bin` and `/system/bin`, and an `ls /system/bin` and cursory inspection confirms it.

From this list of three directories, it's pretty easy to imagine how binaries are organized in Fuchsia.

`/system/apps` doesn't exist, but oooooh, `/boot/bin` has some good stuff.

```
$ ls /boot/bin
blobfs
clkctl
clock
crashsvc
dd
devhost
devmgr
df
dlog
driverinfo
ethtool
fixfs
fsck
fsck-msdosfs
fshost
gpt
install-disk-image
kill
killall
kstats
lsblk
lsdev
lsusb
memgraph
minfs
mkfs
mkfs-msdosfs
mount
netdump
netsvc
ping
ps
pwrbtn-monitor
run-vc
sh
spawn
svchost
threads
top
umount
virtual-console
vmaps
vmos
$
```

So many commands to play with. I wonder where these commands come from and whether [uutils] can be ported to Fuchsia.

[uutils]: https://github.com/uutils/coreutils




## Running code on Fuchsia

What I really need to do is run Rust code on Fuchsia. Still following the "Fuchsia getting started" instructions, section ["Explore Fuchsia"][expf], I'm going to learn how to push a package to the running QEMU instance.

[expf]: https://fuchsia.googlesource.com/docs/+/HEAD/getting_started.md#explore-fuchsia

There's only a few steps here:

> Make a change to the rolldice binary in garnet/bin/rolldice/src/main.rs.
>
> In a separate shell, start the development update server, if it isn't already running:
>
> > fx serve -v
>
> Re-build and push the rolldice package to a running Fuchsia device with:
>
> > fx build-push rolldice
>
> From a shell prompt on the Fuchsia device, run the updated package with:
>
> > run rolldice

We're going to edit the example program, `rolldice`, then push it to the running QEMU instance, then run it in Fuchsia.

Hey! Did you notice `rolldice`s path? __.../main.rs__. It's a Rust program! The Fuchsia tutorial involves Rust!

`rolldice` lives in [`garnet/bin/rolldice/src/main.rs.`][rdm], and is part of _Garnet_,
the second layer of the ["Fuchsia Layer Cake"][flc], above Zircon. And notice that to run `rolldice` we have to type `run rolldice`, not just `rolldice`; so `rolldice` must not be on the `$PATH`, and live somewhere other than the places I'm already looked: `system/apps`, `system/bin, and `system/boot`.

[rdm]: https://fuchsia.googlesource.com/garnet/+/6e51bae742541bb09fe6ddb84a507bb5dad4b09f/bin/rolldice/src/main.rs
[flc]: https://fuchsia.googlesource.com/docs/+/master/development/source_code/layers.md

The Garnet [`bin/` directory][gbn] contains many more programs that would be fun to play with, including some that istr from `/system/bin`, like `time` and `uname`; but many of them aren't in `/system/bin`, like `rolldice`. So not everything in Garnet`s `bin/` source directory ends up in the same place on the device. Where does `rolldice` live?

There's a curious, non-Rusty file in the `rolldice` directory, [`rolldice/BUILD.gn`][rbgn], and I'm wondering if it could give be a clue as to where `rolldice` is installed.

[gbn]: https://fuchsia.googlesource.com/garnet/+/6e51bae742541bb09fe6ddb84a507bb5dad4b09f/bin/
[rbgn]: https://fuchsia.googlesource.com/garnet/+/6e51bae742541bb09fe6ddb84a507bb5dad4b09f/bin/rolldice/BUILD.gn

Seems to contain a build description and a package description, with contents like

```
rustc_binary("bin") {
  name = "rolldice"
  edition = "2018"
  deps = [
    ":lib",
    "//third_party/rust-crates/rustc_deps:rand",
    "//third_party/rust-crates/rustc_deps:structopt",
  ]
}
```

and

```
package("rolldice") {
  deps = [
    ":bin",
  ]

  binary = "rolldice"
}
```

Well that first snippet seems to be instructions for how to build a Rust binary, seemingly without cargo, and yeah there's no Cargo.toml for `rolldice`. That's usually a bad smell. I'd guess they're avoiding cargo because they don't want cargo picking it's own non-vetted packages and dependencies, maybe also because they don't want cargo to hit the internet. So they've probably got a folder full of whitelisted dependencies.

I hope there's a path forward that solves whatever problems they have with cargo, within cargo.

---

So I came to this file hoping it would lead me to the on-device path of `rolldice` but there's not much to go off here &mdash; maybe the ":bin" dep is meaningful to somebody with the right context. I'm starting to lose faith in this direction of exploration, like maybe I should just go poke around the Fuchsia filesystem until I find it (maybe Fuchsia has a working `find` command), but real quick I want to see if I can trace that ":bin" clue anywhere useful.

First though, I recall that _some_ of these Garnet bins are installed to the system directory but most arne't. I wonder if there's a suggestive difference between [`rolldice/BUILD.gn`][rbgn] and [`time/BUILD.gn`][tbgn], which includes this definition:

[tbgn]: https://fuchsia.googlesource.com/garnet/+/6e51bae742541bb09fe6ddb84a507bb5dad4b09f/bin/time/BUILD.gn#23

```
package("time") {
  deprecated_system_image = true
  deps = [
    ":bin",
  ]
  binaries = [ {
        name = "time"
      } ]
}
```

`time` is part of a "deprecated system image"! Seems to explain it. But I still don't know where ":bin" projects go when they _aren't_ part of a "deprecated system image".

I'm going to try one other thing. I ripgrep for ":bin" in `.gn` files.

```
brian@whatever:~/fuchsia⟫ rg --type-add "gn:*.gn" -tgn :bin --no-heading
third_party/cobalt_config/BUILD.gn:16:  tool = "//third_party/cobalt/config/config_parser/src:bin"
garnet/bin/chrealm/BUILD.gn:28:    ":bin",
garnet/bin/cowsay/BUILD.gn:19:    ":bin",
garnet/bin/appmgr/BUILD.gn:135:    ":bin",
...
```

Many hits, and I poke at a few of them but learn nothing. Ok, now I'm going to see if I can search the Fuchsia device for `rolldice` (I don't have to &mdash: right now &mdash; be doing this pedantic work of figuring out how `rolldice` gets from the local build to wherever it is on the device, but hopefully I learn something).

Back on my still-running Fuchsia QEMU box, I see if the [`find`][find] command exists.

[find]: https://linux.die.net/man/1/find

```
$ find --help
usage: find [-H | -L] path ... [expression ...]
```

I am very curious where these unix utils come from, whether they are bespoke Fuchsia commands or come from another project (like [busybox] but not busybox because busybox is GPL and big companies like Google tend to shy away from the GUL &mdash; this is why folks should look into [uutils]!). I'm also curious where their source code is. I'm not going investigate that matter now though. I am going to try to run `find` to find `rolldice`.

[busybox]: https://www.busybox.net/

I run find

```
$ find / -name rolldice
[02279.196] 04845.04857> No service found for path .
find: failed to stat /hub/r/sys/3862/c/wlancfg/4845/out: Broken pipe
find: failed to stat /hub/r/sys/3862/c/listen/5797/out: Broken pipe
```

It it sits right there for many minutes. It's 1:48 PM.

I try to figure out how to get a second login to the device with `fx`, but nothing in `fx --help` is obviously it. `fx run` creates a second instance.

(Aside from an aside. I've noticed that when commands inside Fuchsia run for longer than I want that I can't Ctrl-C them, or Ctrl-D them or do seemingly anything to kill them. If I could log in from a second shell then maybe I could kill them).

Also, command-line tool documentation is poor. Here's `ls`'s `--help output`:

```
$ ls --help
usage: ls [-1AacdFfHhiLlnpqRrtUu] [file ...]
```

I did though finally get a find command to finish _and_ report something helpful.

```
$ find /pkgfs -name rolldice
find: failed to stat /pkgfs/install/pkg: Not supported
find: failed to stat /pkgfs/install/blob: Not supported
/pkgfs/packages/rolldice
[00224.429] 02996.03269> pkgsvr: 2018/09/02 20:58:41 pkgfs:unsupported(/metadata): dir stat
find: [00224.433] 02996.03269> pkgsvr: 2018/09/02 20:58:41 pkgfs:unsupported(/metadata): dir close
failed to stat /pkgfs/metadata: Not supported
```

`/pkgfs/packages/rolldice`. And, yeah, `/pkgfs/packages` contains all the other stuff from `garnet/bin`. So the `run command`, as in `run rolldice` runs commands out of `/pkgfs/packages/rolldice`, perhaps among other things.

---

That was a long diversion, but informative (to me, maybe you feel differently), but now I'm going to finally follow the instructions and run my customized rolldice on Fuchsia.

First &mdash; and I swear this'll only take a second &mdash; let's see what the stock `rolldice` does:

```
$ run rolldice
+---+
|  *|
|   |
|*  |
+---+
```

Far out.

So back in [`garnet/bin/rolldice/src/main.rs`][rdm] I edit the main function.

```rs
fn main() {
    println!("Let's roll some dice!\n");

    let config = Config::from_args();

    ...
```

While I'm logged into my Fuchsia QEMU instance, in another shell i run

```
brian@whatever:~/fuchsia⟫ fx serve -v
2018-09-02 14:08:47 [bootserver] listening on [::]33331
2018-09-02 14:08:48 [serve-updates] No device found, waiting...
```

Fuuuuuuu...rustrating.

I have the notion that the QEMU support and hardware device support are pretty different, maybe that Googlers have moved on to mostly working on hardware.

This output sure doesn't look like success, but I'm going to head on to the next step anyway and see if it really works.

With my Fuchsia shell running in one terminal, by `fx serve` in another, I open 1 more and run

```
brian@whatever:~/fuchsia/zircon⟫ fx build-push rolldice
ninja: Entering directory `/home/brian/fuchsia/out/x64'
[21/121] ACTION //build/images:system_image.manifest(//build/toolchain/fuchsia:x64)
WARNING: no debug file found for x64-shared/libwebkit.so
[69/120] STAMP obj/build/images/amber_publish_index.stamp
```

Things are being built. At least it looks like there's only going to be 121 steps involved, instead of the ~24k for the full build.

Once it reaches 120/120 complete ... it hangs. More hanging. It's hard to know what's going on in either the Fuchsia buildsystem or at the Fuchsia commandline.

My _guess_ is that it's trying to do the "push" step and because `fx serve` didn't connect to a device, that they're both blocking on waiting for a Fuchsia device to come that never will.

Feels wrong.

Holy hell, y'all we are so close. Like, I just want to upload and run some Rust code and we can call this done, get some fresh air, reevaluate our values, goals and priorities, take a long, tearful, shower. Whatever it takes. We can work it out.

Right now I've got to do something else, anything else, blah. Blah, blah. Bleh, bleh, blah.
