---
layout: basic
title: brson's Rust stuff
---

I've been [doing Rust] for [a while now]. Here are some of my most notable
Rust works, more-or-less from oldest to newest. Since I rarely contribute
directly to Rust lately, this list becomes less indicative of my current
interests as time passes.

* Converted Rust from a statement language to an expression
  language. That's right, Rust once was not even expression-oriented.

* Introduced multithreading to the original (C++-based)
  green-threading runtime (along with [Eric Holk] and others), and
  [rewrote the runtime in Rust]. Most of this did not survive once
  green-threading was removed from Rust.

* Wrote the [segmented stack] implementation of the green threading
  runtime.

* [Created the Result type], an alternative to Haskell's Either
  (which I *also* added to Rust, and later removed). Result was also
  [adopted by the Elm language].

* [Created the notorious `Option::unwrap` method][unwrap].

* [Wrote the standard testing support] along with [compiletest], the
  rustc test driver. Compiletest was the the first "real" parallel
  Rust program and barely worked.

* Wrote a lot of documentation, including the [crate-level docs
  for the standard library][stddocs].

* Created the [original website] that survived until about 2018,
  along with the phrase,
  “Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety”.

* Added an easter egg to the runtime that output [random Lovecraft
  quotes during fatal errors][lovecraft]. The worthiness of Lovecraft
  in a systems language was [hotly debated]. This was mentioned
  in [an ACM Queue article][acm] in 2022.

* Wrote much of the Rust build system, after Graydon Hoare, which is a
  notoriously brain-searing abuse of make. It does though have an
  advanced [self-documenting] help [system]. That's my best idea
  there. It has since been rewritten in Rust, to everybody's great
  relief.

* I designed and implemented Rust's attributes, including the oft-questioned
  notion of "inner" and "outer" attributes and doc-comments. Inner doc comments
  appeared in limited form in Zig as "top-level" doc comments.

* I wrote the first version of rustdoc and created the original
  conventions for using Markdown in Rust doc comments. Rustdoc was
  later rewritten by [Corey Richardson].

* Created the Rust culture of testing documentation by baking the
  feature into rustdoc from the beginning; further developed with
  [rust-skeptic].

* I designed Rust's error codes, though almost all the implementation of error
  reporting was done by others.

* Instigated and maintained [This Week in Rust], the Rust newsletter,
  along with [Corey Richardson], [Vikrant Chaudhary], and [Andre
  Bogus]. The success of This Week in Rust has been influential on
  other open source projects.

* Started the TWIR [Quote of the Week] tradition,
  with its epic forum thread.

* Created the [Rust Forge].

* Created the [Friend of the Tree] tradition (after Mozilla's own) for
  acknowleding community contributions. It has since died.

* Created the Unix distribution tooling including [rustup.sh],
  [rust-installer] and [multirust].

* Created, with [Diggory Blake], the unified [rustup] installer, the current
  preferred installation method on all supported platforms.

* Wrote the [rustup.rs] website that remains live as of 2021, and the the rustup
  docs, which others improved and converted into [a book][rustupbook].

* Created [Servo], the web browser written in Rust, along with
  [Patrick Walton], back when Rust barely worked.

* Established and supported Rust's community meetup tradition, with
  [Erick Tryzelaar] hosting the infamous [San Francisco Bay Area Rust
  Meetup] and supporting [many others].

* Managed the release process [since the original 0.1 release in
  January 2012][0.1]. Maintained the [release notes].

* I am responsible for [demanding Rust switch from an 80 character
  limit to 100][100chars], a subject of much consternation over the
  years.

* I both added and removed the ternary operator from Rust.

* Defined Rust's characteristic 101 exit code, so that the test runner
  could distinguish 'good failure' from 'bad failure'.

* [Added the `transmute` function], the legendary waraxe of unsafety.

* Pioneered Rust's practice of ecosystem-wide regression testing
  with [crater] and [cargobomb].

* Created the infamous [`weird-exprs`] test case.

* Created the widely-used [error-chain] crate for ergonamic error handling. It
  has since been deprecated by better crates and practices.

* Coordinated with Linux distributions to [get their Rust packaging
  in order][prp] in time for Rust to be adopted in Firefox.

* Coordinated the effort to [port Rust to WebAssembly].

* Coordinated a large-scale effort to [improve the quality of the
  Rust crate ecosystem][blitz].

* Coordinated a community effort to create [Rust API guidelines][api].

* Coordinated a community effort to create a [Rust cookbook][cook].

<!-- links -->

[acm]: https://queue.acm.org/detail.cfm?id=3534857&doi=10.1145%2F3534857
[api]: https://rust-lang-nursery.github.io/api-guidelines/
[cook]: https://rust-lang-nursery.github.io/rust-cookbook/
[rustup]: https://github.com/rust-lang-nursery/rustup.rs
[rustup.sh]: http://github.com/rust-lang/rustup.sh
[rust-installer]: http://github.com/rust-lang/rust-installer
[multirust]: http://github.com/brson/multirust
[Servo]: https://github.com/servo/servo
[doing Rust]: http://www.rust-lang.org/team.html
[a while now]: https://github.com/rust-lang/rust/commit/0633c7ae6e54edebde8421cef14267ad1ba1e30c
[rewrote the runtime in Rust]: https://mail.mozilla.org/pipermail/rust-dev/2013-August/005158.html
[segmented stack]: https://mail.mozilla.org/pipermail/rust-dev/2013-November/006314.html
[Created the Result type]: https://github.com/rust-lang/rust/commit/c1092fb6d88efe51e42df3aae2a321cc669e12a0
[adopted by the Elm language]: https://github.com/rust-lang/rust/commit/c1092fb6d88efe51e42df3aae2a321cc669e12a0
[unwrap]: https://github.com/rust-lang/rust/commit/910a32c7c777296be0992bf0d6f2d66261c407d6
[Wrote the standard testing support]: https://github.com/rust-lang/rust/commit/09982784c6ad1c78f9480c3c2c0c3a2b2bf7f969
[compiletest]: https://github.com/rust-lang/rust/commit/2573fe7026eb696841acbba8f3d1c09e2224acf0
[lovecraft]: https://github.com/brson/rust/blob/71a71ce4f948dd5ae792db4a88c9cc2fae94dfb0/src/libstd/rt/util.rs#L124
[hotly debated]: https://news.ycombinator.com/item?id=8869572
[self-documenting]: https://github.com/rust-lang/rust/blob/e4e93196e16030ebf7a20c473849534235d676f8/mk/main.mk#L592
[system]: https://github.com/rust-lang/rust/blob/e4e93196e16030ebf7a20c473849534235d676f8/Makefile.in#L11
[Corey Richardson]: https://github.com/cmr
[Graydon Hoare]: https://github.com/graydon
[Eric Holk]: https://github.com/eholk
[Patrick Walton]: https://github.com/pcwalton
[This Week in Rust]: http://this-week-in-rust.org
[Vikrant Chaudhary]: https://github.com/nasa42
[Andre Bogus]: http://github.com/llogiq
[Friend of the Tree]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Doc-friends-of-the-tree.md
[Erick Tryzelaar]: http://githtub.com/erickt
[San Francisco Bay Area Rust Meetup]: http://www.meetup.com/Rust-Bay-Area/
[many others]: https://users.rust-lang.org/t/a-list-of-rust-1-0-launch-meetups/1171/16
[0.1]: https://mail.mozilla.org/pipermail/rust-dev/2012-January/001256.html
[release notes]: https://github.com/brson/rust/blob/relnotes/RELEASES.md
[100chars]: https://github.com/rust-lang/rust/pull/5340
[Added the `transmute` function]: https://github.com/rust-lang/rust/commit/f12adcbf930122ef6d98790b53d80d511dc62406
[crater]: https://github.com/brson/taskcluster-crater
[cargobomb]: https://github.com/brson/cargobomb
[error-chain]: https://github.com/brson/error-chain
[prp]: https://internals.rust-lang.org/t/perfecting-rust-packaging/2623
[port Rust to WebAssembly]: https://internals.rust-lang.org/t/need-help-with-emscripten-port/3154
[rust-skeptic]: https://github.com/brson/rust-skeptic
[blitz]: https://blog.rust-lang.org/2017/05/05/libz-blitz.html
[Diggory Blake]: https://github.com/Diggsey
[Quote of the Week]: https://users.rust-lang.org/t/twir-quote-of-the-week/328/8
[rustup.rs]: https://rustup.rs
[original website]: https://prev.rust-lang.org/
[Rust Forge]: https://forge.rust-lang.org/
[rustupbook]: https://rust-lang.github.io/rustup/
[stddocs]: https://doc.rust-lang.org/std/index.html
[`weird-exprs`]: https://github.com/rust-lang/rust/blob/master/src/test/ui/weird-exprs.rs
