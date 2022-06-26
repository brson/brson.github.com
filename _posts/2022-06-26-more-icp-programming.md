---
layout: post
title: Another look at programming The Internet Computer (ICP / Dfinity)
tags: [blockchain]
---

It's been more than a year since my [last look at programming on ICP][ll].
With their [May 2022 hackathon][hack] it was a good opportunity to try again.

[ll]: https://brson.github.io/2021/01/30/dfinity-impressions
[hack]: https://supernova.devpost.com/

My partner and I want to make a project that works well with,
and takes advantage of,
ICP's architecture.
Some things we think ICP should be good at:

- Static websites - ICP can serve large amounts of static data, like IPFS.
- Authentication - ICP has a unique Web Authentication-based identity system.
- Single page applications - the project's we have seen so far appear to be SPAs.

We eventually decided to make [a wiki][aw],
with hopes of also creating a wiki factory &mdash;
a web app for creating wikis,
envisioned as a decentralized Wikia.
This seems like something that should fit the ICP model pretty well,
and relatively easy to prototype.

[aw]: https://github.com/brson/icp-hackathon-project


## How it turned out

ICP has improved a lot in the last year,
and we enjoyed ourselves a lot more this time.

At times we even felt inspired:
though we didn't get to test it ourselves,
ICP has a model that lets contracts pay for themselves,
where every user transaction does not require the user
to interact with a crypto wallet.
That combined with their [`internet-identity`]
authentication that wraps [WebAuthn]
makes me think it could eventually be a dapp platform that
average non-crypto-enthusiasts can interact with.

[WebAuthn]: https://webauthn.guide/

Every ICP canister (smart contract)
is self-describing using [candid],
can be called from the command line,
and automatically creates a web page that contains
a simple UI through which the canister's methods can be called.
This is super nice.
On some chains there is no interface description language.

We wrote our code in [Motoko],
Dfinity's smart contract language,
not Rust.
This worked out fine:
the compiler produces pretty good error messages
and didn't have any major problems.
The documentation was ok,
though we did find ourselves reading the Motoko base library source code.

[Motoko]: https://github.com/dfinity/motoko

ICP canisters are actor-oriented and asynchronous,
and we found this to be fairly intuitive.
I get the impression though that this causes some difficulties
adapting Ethereum conventions since communication between canisters is not atomic.
We didn't get far enough to experience those difficulties though.

We didn't get our project to a working state,
and didn't submit anything for the hackathon.
We may continue the project at some point in the future,
and may revisit ICP again.


## Things we learned

- The local path to my identity is `~/.config/dfx/identity/default/identity.pem`.

- Building and deploying the [`internet-identity`] canister
  required [some changes][icch] to what was described in the examples.

- If the `internet-identity` caninster won't deploy with

  > Error: The Replica returned an error: code 5, message: "Canister rwlgt-iiaaa-aaaaa-aaaaa-cai trapped explicitly: stable memory header: invalid magic: [68, 73, 68]"

  then deleting the `.dfx` directory and trying again will work.

- WASM blobs can be encoded [with a shell pipeline][blob-shell] and passed to
  `dfx canister call`.

[icch]: #internet-canister-changes
[blob-shell]: #blob-shell
[`internet-identity`]: https://github.com/dfinity/internet-identity


## The project log

The rest of this is just a log of our experience.
It probably won't be that useful to read straight through,
but might be useful for ICP developers or those searching for problems like we ran into.


## Getting started (2022/05/10)

Day 1 of the hackathon.
We have a few ideas for the project we want to make,
but are not sure of the capabilities of ICP,
so the first thing we are going to do is go through the tutorials,
create and deploy a simple canister (ICP's smart contract / dapp bundles) with backend and frontend
that can read and write certified data.

ICP is unique in existing blockchains in that it appears to incorprate
decentralized storage as a first-class feature &mdash; there should be
no need to leverage e.g. IPFS to store big blogs. In theory this
should open up new possibilities for the kinds of apps one might deploy,
but from perusing the documentation I don't understand how to use the feature.

ICP's other storage mechanism is a kind of persistent RAM,
suitable for small state.

OK, let's do some tutorials.


## Hello world

I'm going to go through the [Hello World Quick Start][hwqs]
I hope this will get me a toolchain and teach me to deploy and run a canister.

[hwqs]: https://smartcontracts.org/docs/current/developer-docs/quickstart/hello10mins

First thing I learn:

- Dapps are at least two canister's: a backend and a frontend.

More learnings:

- "Cycles" are gas
- Each cannister has a cycles account and pays its own gas
- ICP can be converted to cycles
- There are cycles wallets - not clear on if every canister has one

## Installing tools

I run

```
sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"
```

It says

```
Executing dfx install script, commit: edfe002ed7fcf06228a72cef959989d4b10f10f0
Version found: 0.9.3
Creating uninstall script in ~/.cache/dfinity
uninstall path=/home/brian/.cache/dfinity/uninstall.sh
Checking for latest release...
Will install in: /home/brian/bin
Installed /home/brian/bin/dfx
```

Seems ok.

This `/home/brian/bin` directory is not part of my path,
and `dfx` is the only thing in it.

I add

```bash
export PATH="/home/brian/bin:$PATH"
```

to my `~/.bashrc` file and restart my shell and I can run `dfx`.
My `~/.bashrc` is filled with these augmentations of `PATH` for custom tooling.

I run `dfx --version:`

```
$ dfx --version
dfx 0.9.3
```

ICP requires node.js 12, 14, and 16, and I have 16.5.

I run `dfx new hello` and the first lines it prints are:

```
Fetching manifest https://sdk.dfinity.org/manifest.json
You seem to be running an outdated version of dfx.

You are strongly encouraged to upgrade by running 'dfx upgrade'!
  Version v0.9.3 installed successfully.
```

It said my `dfx` was out of date and upgraded,
even though I just installed `dfx` and had the same version it upgraded to.

After the upgrade it continued to printed

```
Creating new project "hello"...
CREATE       hello/dfx.json (481B)...
CREATE       hello/README.md (2.25KiB)...
CREATE       hello/.gitignore (165B)...
CREATE       hello/src/hello_assets/assets/sample-asset.txt (24B)...
CREATE       hello/src/hello/main.mo (99B)...
CREATE       hello/package.json (1.14KiB)...
CREATE       hello/webpack.config.js (3.52KiB)...
CREATE       hello/src/hello_assets/assets/main.css (537B)...
CREATE       hello/src/hello_assets/assets/logo.png (24.80KiB)...
CREATE       hello/src/hello_assets/assets/favicon.ico (15.04KiB)...
CREATE       hello/src/hello_assets/src/index.html (625B)...
CREATE       hello/src/hello_assets/src/index.js (526B)...
⠄ Installing node dependencies...

added 407 packages, and audited 408 packages in 22s

85 packages are looking for funding
  run `npm fund` for details

  Done.
```

Then it printed

```
Creating git repository...
```

creating a git repository that I had to immediately delete because I am already in a git repository.
Cargo does this too and it is annoying.

Finally it printed a big Dfinity / ICP logo and some help info:

```
To learn more before you start coding, see the documentation available online:

- Quick Start: https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html
- SDK Developer Tools: https://sdk.dfinity.org/docs/developers-guide/sdk-guide.html
- Motoko Language Guide: https://sdk.dfinity.org/docs/language-guide/motoko.html
- Motoko Quick Reference: https://sdk.dfinity.org/docs/language-guide/language-manual.html

If you want to work on programs right away, try the following commands to get started:

    cd hello
    dfx help
    dfx new --help
```


## Local deployment

In one window I run `dfx start` and in the other `npm install` then `dfx deploy`.

I see

```
$ dfx deploy
Creating the "default" identity.
  - generating new key at /home/brian/.config/dfx/identity/default/identity.pem
Created the "default" identity.
Creating a wallet canister on the local network.
The wallet canister on the "local" network for user "default" is "rwlgt-iiaaa-aaaaa-aaaaa-cai"
Deploying all canisters.
Creating canisters...
Creating canister "hello"...
"hello" canister created with canister id: "rrkah-fqaaa-aaaaa-aaaaq-cai"
Creating canister "hello_assets"...
"hello_assets" canister created with canister id: "ryjl3-tyaaa-aaaaa-aaaba-cai"
Building canisters...
Building frontend...
Installing canisters...
Creating UI canister on the local network.
The UI canister on the "local" network is "r7inp-6aaaa-aaaaa-aaabq-cai"
Installing code for canister hello, with canister_id rrkah-fqaaa-aaaaa-aaaaq-cai
Installing code for canister hello_assets, with canister_id ryjl3-tyaaa-aaaaa-aaaba-cai
Uploading assets to asset canister...
Starting batch.
Staging contents of new and changed assets:
  /main.css 1/1 (537 bytes)
  /main.css (gzip) 1/1 (297 bytes)
  /index.js 1/1 (613369 bytes)
  /index.js (gzip) 1/1 (146412 bytes)
  /index.js.map 1/1 (663443 bytes)
  /sample-asset.txt 1/1 (24 bytes)
  /favicon.ico 1/1 (15406 bytes)
  /index.html (gzip) 1/1 (382 bytes)
  /index.js.map (gzip) 1/1 (150713 bytes)
  /index.html 1/1 (663 bytes)
  /logo.png 1/1 (25397 bytes)
Committing batch.
Deployed canisters.
URLs:
  Frontend:
    hello_assets: http://127.0.0.1:8000/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai
  Candid:
    hello: http://127.0.0.1:8000/?canisterId=r7inp-6aaaa-aaaaa-aaabq-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai
```

I learn:

- the path to my (I assume it's mine) identity is `~/.config/dfx/identity/default/identity.pem`

I open the web page for `hello_assets` and see
a page with a text entry box and a button that says `Click Me`.

I enter my name and click the button.

Nothing happens.

The web console shows errors:

```
POST http://127.0.0.1:8000/api/v2/canister/rrkah-fqaaa-aaaaa-aaaaq-cai/call 400 (Bad Request)
call @ index.js:458
await in call (async)
caller @ index.js:200
handler @ index.js:221
(anonymous) @ index.js:17881

Uncaught (in promise) Error: Server returned an error:
  Code: 400 (Bad Request)
  Body: Specified ingress_expiry not within expected range:
Minimum allowed expiry: 2022-05-10 09:03:31.088945300 UTC
Maximum allowed expiry: 2022-05-10 09:09:01.088945300 UTC
Provided expiry:        2022-05-10 16:00:29.709 UTC
Local replica time:     2022-05-10 09:03:31.088946700 UTC

    at HttpAgent.call (index.js:462:19)
    at async caller (index.js:200:45)
    at async HTMLFormElement.<anonymous> (index.js:17881:20)
```

I see errors about time and suspect this is a common WSL issue
where the WSL clock gets out of sync.

I run

```
sudo hwclock -s
```

kill and restart `dfx start` and try again, and the dapp works.

I learn:

- interacting with ICP requires an accurate clock


## Testing on the command line

I can interact with the canister on the command line:

```
$ dfx canister call hello greet everyone
("Hello, everyone!")
```

I visited the webpage before by visiting the dfx server,
but I can apparently also start node with `npm start`
and visit at `localhost:8080`.
I don't bother to do that for now.


## Deploying to the network (2022/05/11)

Still going through the quick start hello world tutorial.
I need to acquire _cycles_, gas.

I ping the internet computer:

```
$ dfx ping ic
{
  "ic_api_version": "0.18.0"  "impl_hash": "b1d54efe7bc5a93a707f64afdbe6d95d172dd976873d4a44989cfdf9fd8d1f45"  "impl_version": "3d6fc111c09d316b2ed28208e4a8202d9293ecb0"  "replica_health_status": "healthy"  "root_key": [48, 129, 130, 48, 29, 6, 13, 43, ... (many bytes)
}
```

It's just splatted debug formatting of some kind. Not even valid JSON that I can pipe through a pretty-printer (maybe it is [candid]). Look at that huge byte array...

[candid]: https://github.com/dfinity/candid

At least it works though.

I am going to acquire cycles from the faucet.

## Claiming a cycles airdrop

Following the instructions here:

> [https://smartcontracts.org/docs/current/developer-docs/quickstart/cycles-faucet/](https://smartcontracts.org/docs/current/developer-docs/quickstart/cycles-faucet/)

I navigate to

> [https://faucet.dfinity.org](https://faucet.dfinity.org)

The instructions in the developer docs say I'll have to authenticate with GitHub,
but the faucet app says I have to authenticate with Twitter.
I was not enthusiastic about authenticating with GitHub,
and am even more turned off by having to link my Twitter account to this faucet.

Is there no devnet for IC where I can just get an airdrop for free?

I ask in the dfinity Supernova Hackathon #general channel:

> I am trying to follow the instructions to use the cycles faucet and the faucet
  app wants me to authenticate with twitter. Is there any other way to get an
  airdrop?

> These instructions and hoops make it seem like they are for the mainnet. Is
  there a devnet with free airdrops?

I probably shouldn't have asked &mdash; I'm really just airing my frustration,
and I know the answer.

For now I am going to skip the step of deploying to a real network.
I can work with the localnet.


## Starting from an example

I decide that I want to just start writing something.
I know that for my dapp I will need to know how to store data on behalf of a user.
So I am going to look for an example that shows how to write data.

The [examples] repo contains an example called `svelte-motoko-starter` that sounds
exactly like what I want: Svelte on the frontend, Motoko on the backend, and
the description says it uses Internet Identity, ICP's authentication mechanism.

[examples]: https://github.com/dfinity/examples

This example contains an ICP elevator pitch that is compelling:

> The Internet Computer is a novel blockchain that has the unique capability to
  serve web content while not requiring the end users to use a browser
  extension, such as Metamask.

No browser extension.
Blockchain marketers are so bad at succinctly explaining themselves,
but this I understand.
No browser extension.

When I try to run `dfx start` under the `svelte-motoko-starter` example
it fails and I see this

```
$ dfx start
Warning: The version of DFX used (0.9.2) is different than the version being run (0.9.3).
This might happen because your dfx.json specifies an older version, or DFX_VERSION is set in your environment.
We are forwarding the command line to the old version. To disable this warning, set the DFX_WARNING=-version_check environment variable.

Error when trying to forward to project dfx:
Unknown version '0.9.2'.
Installed executable: 0.9.3
```

This seems easy enough to fix: I'll just change the `dfx.json` config to use the latest dfx.
And `dfx start` works.

I continue following the example instructions to install Internet Identity on my local network.

It fails again:

```
$ II_ENV=development dfx deploy --no-wallet --argument '(null)'
Deploying all canisters.
Creating canisters...
Creating canister "internet_identity"...
"internet_identity" canister created with canister id: "rwlgt-iiaaa-aaaaa-aaaaa-cai"
Building canisters...
Executing 'src/internet_identity/build.sh'
could not find ic-cdk-optimizer
ic-cdk-optimizer version 0.3.1 is needed, please run the following command:
  cargo install ic-cdk-optimizer --version 0.3.1
Error: The build step failed for canister 'rwlgt-iiaaa-aaaaa-aaaaa-cai' with an embedded error: The custom tool failed.
```

It seems tell me how to fix the problem though so I run:

```
cargo install ic-cdk-optimizer --version 0.3.1
```

After that succeeds I am able to build and deploy the Internet Identity canister.
I can browse to its auto-generated Candid web UI.
I like that ICP has an interface definition that allows these UIs to be auto-generated.

Per the instructions I deploy the backend and frontend canisters,
but the frontend doesn't work when I load it in the browser.
The console says

```
bundle.js:2 Uncaught ReferenceError: process is not defined
    at bundle.js:2:387929
(anonymous) @ bundle.js:2
```

When I run `npm run dev` I see this in the logs:

```
process/browser (imported by node_modules/js-sha256/src/sha256.js)
(!) Circular dependency
```

I guess a circular dependency has caused rollup to not inject "process/browser"
into my bundled environment?

I discover a [`rollup-plugin-inject-process-env`] plugin that
is supposed to resolve this problem,
but this project doesn't use it,
and I'm not confident I can adapt the project to use it,
not understanding the `rollup.config.js` file.

[`rollup-plugin-inject-process-env`]: https://www.npmjs.com/package/rollup-plugin-inject-process-env,

I work around the problem by putting this as the first script in `index.html`:

```html
<script>window.process = { }</script>
```


<a name="internet-canister-changes"/>

## Creating a skeleton dapp (2022/05/12)

With only a few modifications I got the `svelte-motoko-starter` example working,
so I am going to duplicate that into my own repo and start using it as a base.

Since this example contains a git submodule for `internet-identity`,
copying it isn't trival &mdash; I have to reproduce that submodule in my own repo.

After about an hour of hacking I can't get through the Internet Identity authentication flow.
The web console says "Error: Fail to verify certificate" and the website just hangs forever.

There is a [forum thread on the subject][authf] but no definite solutions.

[authf]: Error: Fail to verify certificate

I think I figure out how to work around the problem by

1) Upgrading the `internet-identity` submodule to commit e4b23633ea738b75f28d6d0ef14fe6b538d99bc7
2) Setting the `II_FETCH_ROOT_KEY` environment variable to `1` as in

  > `II_FETCH_ROOT_KEY=1 dfx deploy --no-wallet --argument '(null)'`

I don't know what this `--argument '(null)'` thing is about,
but it was in the example instructions.
The auth flow requires a captcha and it can be disabled with another env var.

I settle on this command to build and deploy `internet-identity`

> `II_FETCH_ROOT_KEY=1 II_DUMMY_CAPTCHA=1 II_DUMMY_AUTH=1 dfx deploy --no-wallet --argument '(null)'`

I succeed at authenticating my dapp on the local network.

I would run into this "Fail to verify certificate" problem again throughout this project,
with no solution.


## Creating a canister from within a canister (2022/05/23)

After some time off I have come back to the task.
I still am unsure of what I actually want to build,
but I know that the dapp I create will need to create canisters.
So today my goal is to figure out how to create a canister from my dapp.

I am open to creating canisters either directly from my dapp's canister or through the frontend using the RPC interface.
I just need to figure out how to programmatically create canisters.

Where is the motoko base library documentation?

The link to the motoko reference documentation goes to a [blank page] dead end,
but the docs do exist here:

> https://smartcontracts.org/docs/current/references/motoko-ref/array

[blank page]: https://smartcontracts.org/docs/current/references/motoko-ref/

There's just no navigable entrypoint for them.
The above link instead goes to the docs for `Array`.

The base library is mostly data types,
so probably you don't create a new canister by calling a syscall.
There is an [`ExperimentalInternetComputer`] ... type, I think,
for calling other canisters.

[`ExperimentalInternetComputer`]: https://smartcontracts.org/docs/current/references/motoko-ref/experimentalinternetcomputer

So probably I would need to call some system canister to create a canister.
I find the a description of the [IC syscalls][icsys],
and there's nothing there about creating canisters.

[icsys]: https://smartcontracts.org/docs/current/references/ic-interface-spec/#system-api-imports

There is an [IC management canister][icmc].
It looks like a canister but is not implemented as a canister.
It's like a system canister.

[icmc]: https://smartcontracts.org/docs/current/references/ic-interface-spec/#ic-management-canister

Maybe that's what I need.
Yeah, everything for creating and managing canisters is here.

I spend about 30 minutes unable to redeploy the `internet_identity` canister
until I delete `internet_identity/.dfx`.


## Sketching a wiki page canister (2022/05/24)

We've decided to try to create a wiki.
There are going to be at least three canisters:

- `page_backend` - a single page in the wiki
- `wiki_backend` - a controller for the entire wiki, to create and lookup pages
- `wiki_frontend` - the website, an SPA

If we succeed at those we will probably also make two more:

- `wiki_factory_backend` - a service that creates wikis
- `wiki_factory_frontend` - the frontend to the wiki creation service

The idea is to be a "Permissionless Wikia".

We've started by restructuring the example project to have the three above canisters,
and outlining the `page_backend` actor:

```
import Principal "mo:base/Principal";
import Error "mo:base/Error";

actor {
    public query func getFullPageMarkup() : async Text {
        throw Error.reject("poop");
    };

    public func setFullPageMarkup(text: Text) {
        throw Error.reject("poop");
    }
};
```

That should be good enough for a first prototype.
I give that to Aimee to fill in while I hack on the frontend.
She comes back later with this:

```
import Principal "mo:base/Principal";

actor {
    var pageContent : Text = "New page.";
    
    public query func getFullPageMarkup() : async Text {
        return pageContent;
    };

    public func setFullPageMarkup(text: Text) {
        pageContent := text;
    };
};
```

With IC's Candid interface specification and auto-generated UIs
we can test it without a frontend. This is pretty great.

This seems good enough for basic testing.
Now I outline the `wiki_backend` canister:

```
import Principal "mo:base/Principal";
import Error "mo:base/Error";

actor {
    public func initialize() : async Bool {
        // if already initialized, return false
        // todo create index page
        // use the ic management canister:
        // https://smartcontracts.org/docs/current/references/ic-interface-spec/#ic-management-canister
        // todo return true
        throw Error.reject("poop");
    };

    public query func getIndexPagePrincipal() : async ?Principal {
        // todo if index page has been created, return it,
        // otherwise null.
        throw Error.reject("poop");
    };

    public query func getPagePrincipal(name: Text) : async ?Principal {
        throw Error.reject("poop");
    };

    // Returns a new Principal on success, null otherwise.
    public query func createPage(name: Text) : async ?Principal {
        // todo if exists return the existing principal
        // if not exists create a new principal and store in hash map
        throw Error.reject("poop");
    };
};
```

This one is going to be a lot harder to implement
since it requires creating a new canister dynamically.
I know it will involve calling the [IC management canister][icmc],
but that's about all I know.
I give that to Aimee to chew on for a while and continue hacking on
the frontend and learning Svelte.

[icmc]: https://smartcontracts.org/docs/current/references/ic-interface-spec/#ic-management-canister


## Hacking on a wiki frontend (2022/05/06)

Svelte is fun.

Today I managed to write an `Article.svelte` component
that successfully loads the markup from the article canister (previously the "page" canister),
mostly by copying the `auth.js` file from the IC svelte template.

```html
<script>
  // The name of the article.
  // We will look up the article's canister in the wiki
  // and load its markup from there.
  export let articleName;

  import Loading from "./Loading.svelte";
  import ArticleDisplay from "./ArticleDisplay.svelte";

  import { Actor, HttpAgent } from "@dfinity/agent";

  import * as PageBackendDid from "../../../declarations/page_backend/page_backend.did.js";

  let loaded = false;

  let icHost = "http://localhost:8000"; // todo
  let agentOptions = {
    host: icHost
  };

  let articleCanisterId = process.env.PAGE_BACKEND_CANISTER_ID; // todo
  let articleAgent = new HttpAgent({ ...agentOptions });

  // todo put this somewhere more sensible
  // Fetch root key for certificate validation during development
  if (process.env.NODE_ENV !== "production") {
    articleAgent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  let articleActor = Actor.createActor(PageBackendDid.idlFactory, {
    agent: articleAgent,
    canisterId: articleCanisterId
  });

  let articleMarkupPromise = articleActor.getFullPageMarkup();

</script>

<div id="article-container">
  {#await articleMarkupPromise}
    Loading article markup...
  {:then articleMarkup}
    <ArticleDisplay {articleMarkup} />
  {/await}
</div>
```

I spent too long trying to get MediaWiki's markup
working in `ArticleDisplay`, failed to get either
of the JavaScript implementations building
in my environment,
decided it was just too complex,
and this wiki can use [CommonMark].
The [JavaScript implementation][cmjs]
built without problems and has a simple interface.

[CommonMark]: https://commonmark.org/
[cmjs]: https://github.com/commonmark/commonmark.js

My CommonMark `ArticleDisplay` svelte component in its entirety:

```html
<script>
  export let articleMarkup;

  import * as commonmark from "commonmark";

  let parser = new commonmark.Parser();
  let writer = new commonmark.HtmlRenderer({
    safe: true
  });
  let parsed = parser.parse(articleMarkup);
  let rendered = writer.render(parsed);
</script>

<div>
  {@html rendered}
</div>
```

Pretty fun.
I'm liking svelte a lot.
Web frontend programming is sometimes fun these days.


## Making the wiki work round-trip (2022/05/27)

I'm kinda liking ICP too.

So far the actor-orientation is intuitive,
and though Aimee is doing most of the Motoko programming,
as a relative novice she is having a lot of success.
She almost has the `wiki_backend` creating new pages.

In the meantime I am still working on the page-editing frontend components.

I added an `ArticleEdit` svelte component,
and added [CodeMirror 6] to it for editing
the wiki markup.
I originally tried using [Monaco],
but it was difficult to get to build correctly
with `rollup`,
and the build took a long time.

[CodeMirror 6]: https://codemirror.net/6/docs/guide/
[Monaco]: https://microsoft.github.io/monaco-editor/


## Prettifying the UI (2022/06/09)

We took a big break from the ICP hackathon,
because the Polkadot hackathon started.
We also have real work todo on occassion,
so suddenly we are too busy.

Our goal for the hackathon now is to get the basic wiki
functionality working in the contracts: create and edit pages;
and to create a pretty UI to drive it.
Use the wiki itself to present the wiki at the end of the hackathon.

Ideally we would have a "wiki factory" too for making new wikis,
but I have a feeling we won't get that far.
Just figuring out how to create new page canisters is going to be a challenge.

The UI can already edit pages,
so today my task is to focus on making it look good.

When I deploy internet-identity to my devnet lately I see an error
from `ic-cdk-optimizer` about an unexpected "frontend" argument:

```
$ `II_FETCH_ROOT_KEY=1 II_DUMMY_CAPTCHA=1 II_DUMMY_AUTH=1 dfx deploy --argument '(null)'`
Creating a wallet canister on the local network.
The wallet canister on the "local" network for user "default" is "rdmx6-jaaaa-aaaaa-aaadq-cai"
Deploying all canisters.
All canisters have already been created.
Building canisters...
Executing 'src/internet_identity/build.sh'
    Finished release [optimized] target(s) in 0.07s
Original:          2.87 MiB
Stripping Unused Data Segments...
    Size:          2.06 MiB (28.3% smaller)
Execute a binaryen optimization pass on your WASM....
    Size:          1.94 MiB (5.7% smaller)

Final Size: 1.94 MiB (32.3% smaller)
Installing canisters...
Deployed canisters.
URLs:
  Candid:
    internet_identity: http://0.0.0.0:8000/?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai&id=rwlgt-iiaaa-aaaaa-aaaaa-cai
error: Found argument 'frontend' which wasn't expected, or isn't valid in this context

If you tried to supply `frontend` as a PATTERN use `-- frontend`

USAGE:
    ic-cdk-optimizer --output <output> [input]

For more information try --help
```

It doesn't appear to cause the deploy to fail though,
so for now I am just ignoring it.


## Hacking on the page UI and history (2022/06/10)

After making a nice simple UI that can display and edit wiki articles,
I am waiting for Aimee to figure out how to install wasm into a new canister,
so that I can create new pages.

In the meantime I have begun trying to add edit history to the `page_canister`.

I have started to see this error in my web console:

```
Error: Fail to verify certificate
```

This appears to be the same error I saw when first setting up `internet-identity`,
and that I thought I wad worked around.
I don't know what it means, and the canister call that is throwing the
exception seems to complete correctly.


## Trying to figure out how to load a wasm blob into our canister (2022/06/16)

Our `wiki_backend` canister needs to install the `page_backend` wasm blob into every page it creates.
We've spent a lot of days now trying to figure out how to first load the `page_backend` blob
into the `wiki_backend` canister so that it has the wasm to install.

The obvious thing we wanted to do first was use something like Rust's `include_bin!` to just
include it directly in the code for wiki_backend, but motoko doesn't seem to have such a thing.

Now we are trying to add a `loadPageWasmBlob` method to `wiki_backend` and call it from the comand line with `dfx canister call`.
We so far don't know how to pass a blob as an argument to `dfx canister call`.

At the moment we are pursuing this process:

- Encode the wasm to hex, probably with the `od` command line tool
- Call `didc encode` with all the proper arguments, including our hex-encoded blob
- Call `dfx canister send` to send the raw message

But I am hoping that maybe `dfx` will interpret hex as a blob argument and we can just
call `dfx canister call loadWasmBlob $HEX_BLOB` or whatever.

I am trying to understand what `dfx` does,
but so far haven't been able to find the source code to it.

I have asked in the `#s-general` Discord chat room for the hackathon:

```
where is the source code to the dfx tool? I am trying to understand it better
```

TedR tells me it is at https://github.com/dfinity/sdk

I feel a bit dumb for not finding it.


<a name="blob-shell"/>

## Converting wasm to candid's encoding and uploading it to a canister (2022/06/19)

Still we need our `wiki_backend` canister to load the `page_backend` wasm into
new page canisters on creation.
We decided on adding a `initWasmBlob` method to `wiki_backend`,
that has to be called by an administer prior to creating any pages.

The Motoko method looks like

```
public func initWasmBlob(wasmModuleBlob : Blob) {
    pageBackendWasmBlob := ?wasmModuleBlob;
}
```

Unable to find a way to get `dfx canister call` to load a parameter from a file,
we decided that what we needed to do was encode our `pagke_backend.wasm` file
in a way that `dfx canister call` would accept.

After some trail-and-error experimentation, we descovered that
we could call our `initWasmBlob` like this:

```
dfx canister call <canister_id> initWasmBlob "blob \"\00\61\73\6D\01\00\00\00\""
```

That argument to `initWasmBlob`, without shell quotes and escapes looks like:

```
blob "\00\61\73\6D\01\00\00\00"
```

This is the [candid syntax for a blob][csb].
In this case it's just an empty wasm program.
A real wasm blob is much longer.

[csb]: https://internetcomputer.org/docs/current/references/candid-ref#type-blob

To get our wasm blob encoded in this format we came up with
the following shell pipeline that works on both Mac OS and Linux:

```
od -An -tx1 -v .dfx/local/canisters/page_backend/page_backend.wasm | sed -E "s/[[:space:]]+/\\\/g" | tr -d "\n" | sed '$ s/\\$//' > page_backend.wasmblob
```

And we called our `wiki_backend` canister with `dfx` to send and initialize the wasm blob:

```
dfx canister call wiki_backend initWasmBlob "blob \"$(cat page_backend.wasmblob)\""
```

This worked fine on Mac OS,
but on Linux it failed with

```
$ dfx canister call wiki_backend initWasmBlob "blob \"$(cat page_backend.wasmblob)\""
-bash: /home/brian/bin/dfx: Argument list too long
```

This is because on Linux arguments are passed on the call stack,
and the size of the wasm blob exceeds the size of the call stack.
This can't be fixed with `xargs` but requires messing with `ulimit`

At this point we decided to give up and not submit anything for the hackathon.
Not enough time left to get something working.

