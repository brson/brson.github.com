---
layout: basic
---

I enjoy writing, and sometimes think I'm pretty ok at it. Like my
code, I like my words to be a mixture of engineering and art. It tends
to be an arduous process, so I don't produce much, but here are some
of the pieces of written.

* [The Minimally-nice Open Source Software Maintainer][nice]. This is
  the most fully-realized thing I've written. I like it because it a
  mix of personal and professional subject matter, and in a unique
  voice.

* [Fireflowers: The Rust programming language in the words of its
  practitioners][ff]. A project to capture a particular moment in time
  for the purpose of marketing, while celebrating the Rust community.
  I was proud that I was able to put this simple concept together in a
  single weekend. I also remain satisfied with the visual design,
  though some felt the colors were eyeball-searing.

* [Refactoring `std` for Ultimate Portability][r]. A proposal for how
  to refactor the standard library into portable components. I like
  how complete the argument is here and that I took the time to make
  illustrations.

* [Starting a new Rust project right, with error-chain][ec]. A simple
  instruction on using the error-chain crate. I like this one because
  I wrote it in an afternoon, where most of my writing takes much
  longer.

* [The End of Unsafety][unsafety]. A presentation I gave with Alex
  Crichton. The repo contains a complete [script], for my parts at
  least. I was quite satisfied with the theme, aesthetics, and
  attention to detail in the deck, particularly the animated ["balloon
  borrowing" demonstration][bb].

* [An annotation of the Rust standard library][std]. This was a small
  project to illustrate some of the neat little mysteries and hacks
  employed by the standard library. I like it mostly because it uses
  the GitHub pull request review as the medium.

* [The "North Star" RFC][ns]. I mostly stay away from the RFC process
  because the open debate is unpleasant to me, but this is one of the
  better I've written. It lays out a process for establishing the Rust
  roadmap. Our actual conformance to the process established therein
  ended up being quite low.

* [The Rust Libz Blitz][blitz1]. A blog post announcing a [major
  project][blitz2] I was responsible for. I was extremely unsatisfied
  with how this came out, and basically gave up and had others edit it
  to completion.

* [Let's make a web service and client in Rust][httptest]. This uses
  the awesomely powerful technique of doing a thing, and writing it
  down as you go so that others can learn from it.

* [Perfecting Rust Packaging][prp]. Not amazing writing, but this was
  the announcement of a major cross-project coordination effort. I
  like the title.

* [Announcing Rust 1.15.1][151]. This is mostly interesting because of
  the community aspect management. The 1.15 release had a glaring
  soundness bug, and this is the mea culpa. The resulting 1.15.1
  release _itself_ was difficult, and resulted in a [post mortem].

* [Rust CI / release infrastructure changes][ci]. Just technical
  communication about CI.

* [How the Rust issue tracker works][i]. A modest exposé of tribal
  knowledge.

* [The Rust compiler 0.1 is unleashed][0.1]. An announcement of the
  first release of Rust, in 2012. It's small, but I'm proud that I got
  to make that announcement. I'm also proud of the second paragraph,
  that _explains what Rust is_, a simple thing that many release
  announcements omit. I'm good at anticipating important details like
  that. I also remain tickled that I got away with writing "unleashed"
  in the title. That would not fly today.

[nice]: http://brson.github.io/2017/04/05/minimally-nice-maintainer
[r]: https://internals.rust-lang.org/t/refactoring-std-for-ultimate-portability/4301
[i]: https://internals.rust-lang.org/t/how-the-rust-issue-tracker-works/3951
[ns]: https://github.com/rust-lang/rfcs/blob/master/text/1728-north-star.md
[ec]: http://brson.github.io/2016/11/30/starting-with-error-chain
[tls]: https://groups.google.com/d/msg/mozilla.dev.servo/3mfkRehXAo4/MXTzpqsFAQAJ
[ci]: https://internals.rust-lang.org/t/rust-ci-release-infrastructure-changes/4489
[151]: https://internals.rust-lang.org/t/rust-ci-release-infrastructure-changes/4489
[post mortem]: https://internals.rust-lang.org/t/rust-1-15-1-release-postmortem/4766
[blitz1]: https://blog.rust-lang.org/2017/05/05/libz-blitz.html
[blitz2]: https://internals.rust-lang.org/t/rust-libz-blitz/5184
[prp]: https://internals.rust-lang.org/t/perfecting-rust-packaging/2623
[unsafety]: https://brson.github.io/the-end-of-unsafety/#/INTRO
[script]: https://github.com/brson/the-end-of-unsafety/blob/master/script.md
[bb]: https://brson.github.io/the-end-of-unsafety/#/BAL
[ff]: https://brson.github.io/fireflowers
[0.1]: https://mail.mozilla.org/pipermail/rust-dev/2012-January/001256.html
[std]: https://github.com/brson/annotated-std-rs/commit/e50c2b16455ceff29488bf1f058b6c10906ef990
[httptest]: https://github.com/brson/httptest

