---
layout: basic
---

I've been [doing Rust] for [a while now]. Here are some of my most
interesting Rust works.

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

* Wrote a lot of documentation.

* Created the original website that survived at least into 2017.

* Added an easter egg to the runtime that output [random Lovecraft
  quotes during fatal errors][lovecraft]. The worthiness of Lovecraft
  in a systems language was [hotly debated].

* Wrote much of the Rust build system, after Graydon Hoare, which is a
  notoriously brain-searing abuse of make. It does though have an
  advanced [self-documenting] help [system]. That's my best idea
  there. It has since been rewritten in Rust, to everybody's great
  relief.

* I wrote the first version of rustdoc and created the original
  conventions for using Markdown in Rust doc comments. Rustdoc was
  later rewritten by [Corey Richardson].

* Created the Rust culture of testing documentation by baking the
  feature into rustdoc from the beginning; further developed with
  [rust-skeptic].

* I designed and implemented Rust's attributes.

* Instigated and maintained [This Week in Rust], the Rust newsletter,
  along with [Corey Richardson], [Vikrant Chaudhary], and [Andre
  Bogus]. The success of This Week in Rust has been influential on
  other open source projects.

* Created the [Friend of the Tree] tradition (after Mozilla's own) for
  acknowleding community contributions.

* Created the Unix distribution tooling including [rustup.sh],
  [rust-installer] and [multirust].

* Created, with [Diggory Blake], the unified [rustup] installer, the current
  preferred installation method on all supported platforms.

* Created [Servo], the web browser written in Rust, along with
  [Patrick Walton] back when Rust barely worked.

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

* Created the widely-used [error-chain] crate for ergonamic error
  handling.

* Coordinated with Linux distributions to [get their Rust packaging
  in order][prp] in time for Rust to be adopted in Firefox.

* Coordinated the effort to [port Rust to WebAssembly].

* Coordinated a large-scale effort to [improve the quality of the
  Rust crate ecosystem][blitz].

<!-- links -->

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