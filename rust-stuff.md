---
layout: basic
---

I've been [doing the Rust thing](http://www.rust-lang.org) for a long
time, and at this point it most of my significant accomplishments are
part of the Rust legacy. So here's where I remind myself of the good
work I've done.

* Converted Rust from a statement language to an expression
  language. Yes, Rust was originally statement-oriented.

* A lot of my work the first few years was on the runtime. I did much
  of the work (along with [Eric Holk](http://github.com/eholk) and
  others) to bring multithreading to the original (C++-based)
  green-threading runtime. [I did most of the rewrite of the runtime
  in
  Rust](https://mail.mozilla.org/pipermail/rust-dev/2013-August/005158.html). Most
  of this work did not survive once green-threading was removed from
  Rust.

* I did much of the work to remove C and C++ from the Rust runtime.

* I [created the `Result`
  type](https://github.com/rust-lang/rust/commit/c1092fb6d88efe51e42df3aae2a321cc669e12a0),
  an alternative to Haskell's Either, [adopted by the Elm
  language](https://github.com/rust-lang/rust/commit/c1092fb6d88efe51e42df3aae2a321cc669e12a0).

* [Wrote the standard testing
  support](https://github.com/rust-lang/rust/commit/09982784c6ad1c78f9480c3c2c0c3a2b2bf7f969)
  along with
  [compiletest](https://github.com/rust-lang/rust/commit/2573fe7026eb696841acbba8f3d1c09e2224acf0),
  the first parallel Rust program.

* I will claim much credit for establishing the 'character' of Rust
  (casual, playful, welcoming, straightforward), from designing the
  website, the README, the std docs, to hiring [Steve
  Klabnik](http://www.steveklabnik.com/) to write the
  documentation. My greatest claim here may be that [I added an easter
  egg to the runtime that output random Lovecraft quotes during fatal
  errors](https://github.com/rust-lang/rust/issues/13871). The
  worthiness of Lovecraft in a systems language was debated far and
  wide.

* I wrote the first version of rustdoc and created the original
  conventions for using Markdown in Rust doc comments. Rustdoc was
  later rewritten by [cmr](http://github.com/cmr/).

* I designed Rust's attributes.

* Maintained [This Week in Rust](http://this-week-in-rust.org), the
  Rust newsletter, along with [cmr](http://github.com/cmr/) and
  others. I believe the success of TWiR influenced the [proliferation
  of similar development
  newsletters](http://lwn.net/Articles/650527/).

* Created the [Friend of the
  Tree](https://github.com/rust-lang/rust-wiki-backup/blob/master/Doc-friends-of-the-tree.md)
  tradition (after Mozilla's own tradition) for acknowleding community
  contributions.

* Created the Unix distribution tooling including
  [rustup.sh](http://github.com/rust-lang/rustup),
  [rust-installer](http://github.com/rust-lang/rust-installer) and
  [multirust](http://github.com/brson/multirust).

* Bootstrapped
  [Servo](https://github.com/servo/servo/graphs/contributors), the web
  browser written in Rust, along with
  [pcwalton](http://github.com/pcwalton) back when Rust barely worked.

* Established and supported Rust's community meetup tradition, with
  [Erick Tryzelaar](http://githtub.com/erickt) hosting the infamous
  [San Francisco Bay Area Rust
  Meetup](http://www.meetup.com/Rust-Bay-Area/) and supporting [many
  others](https://users.rust-lang.org/t/a-list-of-rust-1-0-launch-meetups/1171/16).