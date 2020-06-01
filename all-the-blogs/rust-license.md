---
layout: post
title: "The Rust programming language licensing fiasco"
tags: [rust]
---

Think Rust is [Apache-2.0 / MIT licensed][l]? Oh, sweet summer child.

[l]: https://github.com/rust-lang/rust/tree/d7270712cb446aad0817040bbca73a8d024f67b0#license

Let me tell you a story.

I've always tried to properly obey the open source licenses of the software I
write and use, have a long history of experience with Rust's licensing, and
after filing six new issues ([1], [2], [3], [4], [5], [6]) about Rust's license,
have all that old context swapped into my brain again. So here's the dump.

[1]: https://github.com/rust-lang/rust/issues/63232
[2]: https://github.com/rust-lang-nursery/compiler-builtins/issues/307
[3]: https://github.com/rust-lang-nursery/libm/issues/215
[4]: https://github.com/rust-lang/backtrace-rs/issues/234
[5]: https://github.com/sfackler/rust-openssl/issues/1147
[6]: https://github.com/rust-lang/rust/issues/63238


## TL;DR

The Rust distribution is composed of software carrying at least 12 different
licenses. The Rust _runtime_ &mdash; the standard library and any code linked to
your Rust code &mdash; is permissively licensed (almost entirely MIT /
Apache-2.0); but parts of the Rust toolchain have an [MPL-2.0] copyleft license,
and carry the OpenSSL [advertising clause], along with a variety of other
permissive licenses.

The Rust runtime is almost-entirely MIT licensed, or dual MIT / Apache-2.0
licensed, though at least one component is BSD-3-Clause licensed
([libbacktrace]).

[MPL]: https://opensource.org/licenses/MPL-2.0
[advertising clause]: https://tldrlegal.com/license/openssl-license-(openssl)

The Rust runtime is _incorrectly licensed_ &mdash; it does not contain a
[runtime exception][e], a common clause in language runtimes that allows binary
redistribution without attribution.

[e]: https://en.wikipedia.org/wiki/GPL_linking_exception

Both eliminating the copyleft code and introducing a runtime exception, are
almost unimaginably impossible tasks.

Neither of these facts &mdash; the copyleft licensing, nor the missing runtime
exception &mdash; are widely discussed in the Rust community, and probably not
widely known or understood.

The runtime exception issue extends to the broader Rust crate ecosystem.

Neither Rust, nor most Rust projects, are obeying the many licenses of their
many dependencies, as they are not providing proper attribution. This is a
problem with most modern package-managed open source language ecosystems.


## In the beginning

Rust, as [Graydon] originally released it, was purely [MIT] licensed. It was a
small, self-contained codebase, and Graydon could choose a single, consistent
license to his preference. Even though he worked for Mozilla, the originators of
the [copyleft] [MPL] license, he recognized that a [permissive] license would
allow for a wider audience.

Replacing the original hand-written code-generator with [LLVM] introduced Rust's
first licensing complexity. Now it also carried LLVM's [NCSA] license, which
needed to be clarified in the [COPYRIGHT] file].

It only got more confusing from there.


## The relicensing

In 201X (TODO) the Mozilla lawyers insisted that the project be [relicensed] to
the [Apache-2.0] license. The reason for this is that the Apache license
includes "patent non-aggression" clauses that prevent contributors from suing
each other and users over their own patents represented in the codebase.

Apache-2.0 though is [incompatible with GPL-2.0], so Rust retained the MIT
license, and today Rust is broadly considered dual MIT / Apache-2.0 licensed,
meaning it may be distributed under the terms of either license.

The reality, per this post your are reading, is of course much more complex.

The relicensing process was relatively easy, a matter of getting a written
relicensing acknowledgement from _every single contributor_ up to that point in
Rust's history. This was not so difficult at the time since Rust had few
contributors. For the few contributors who didn't respond, each's contributions
were considered for their copyrightability, and most were determined to be
insignificant enough to be uncopyrightable.

Somewhere there is an archive of all those acknowledgements, signed with
Graydon's GPG key.

Doing another such relicense would be unimaginably difficult (though projects
like LLVM [are attempting such heroic relicenses]).


## Runtime exceptions

Sadly, it wasn't for a few more years until we realized that the MIT /
Apache-2.0 license was not the correct choice for Rust.

TODO


## The MPL encroaches on Rust


## The Rust licenses

The following is a list of licenses carried by software that is part of the Rust
distribution. A good reference for this is the license [EXCEPTIONS] list in the
Rust [tidy] tool, though that list is incomplete &mdash; it only accounts for
licenses declared by crates, and not all the software included in Rust is in the
form of a crate, or properly declared by the "-sys" crate that links to it.

The Rust runtime components (std and other crates linked to your code) carry the
following licenses:

- [MIT] / [Apache-2.0] - Rust's license. Most Rust mainline code carries this
  license; most ecosystem code that wants to be maximally Rust-compatible
  carries this license.
- MIT - The [libm] port from musl, probably other small bits of code.
- MIT / [NCSA] - [compiler-builtins] port from LLVM's compiler-rt
- MIT / [Unlicense] - [BurtSushi]'s projects usually carry the Unlicense,
  which is effectively the same as a dedication to the public domain.

The common license here is MIT &mdash; everything in the Rust runtime carries
the MIT license, and maybe alternately some other license (Apache-2.0). It would
be reasonable for the runtime to carry other equivalently-permissive licenses,
like BSD, but at the moment, the rule is that the runtime must be MIT,
preferably MIT / Apache-2.0.

Note that the "/" notation here means "or". Some crates will make this explicit
by declaring their license as "MIT OR Apache-2.0".

I do know of one exception to this runtime-is-MIT-only rule: [libbacktrace]
is part of the runtime on many platforms, and is BSD-3-Clause licensed. This
isn't reflected in the [EXCEPTIONS] list.

Various components of [mdbook], [rustdoc], [rls], [cargo], and [rustfmt] carry
the following additional licenses:

- [MPL-2.0] 
- MPL-2.0+
- Apache-2.0
- BSD + advertising clause - [OpenSSL]
- [BSD-3-Clause] - [libbacktrace], [fuchsia] crates
- [BSD-2-Clause]
- [CC0-1.0]
- [NCSA] - [LLVM]

Of these, only the MPL is a copyleft license. The Apache license, though
permissive, is more restrictive than MIT, so on its own is not compatible with
the Rust runtime. CC0, like the Unlicense, is effectively a dedication to the
public domain. LLVM contains the uncommon NCSA license, but it is similar to
BSD.

[libm]: https://github.com/rust-lang-nursery/libm
[compiler-builtins]: https://github.com/rust-lang-nursery/compiler-builtins
[LLVM]: https://llvm.org


## Attribution

TODO:

LLVM Apache 2? https://github.com/rust-lang/rust/issues/63232#issuecomment-518347936

GPL-3 code!
https://github.com/rust-lang/compiler-builtins/issues/319
