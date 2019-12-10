---
layout: basic
title: brson's writings
---

I enjoy writing, and sometimes think I'm pretty ok at it. Like my
code, I like my words to be a mixture of engineering and art. It tends
to be an arduous process, so I don't produce much, but here are some
of the pieces I've written.

* [The Minimally-nice Open Source Software Maintainer][nice]. This is
  the most fully-realized thing I've written, a piece about how to
  treat your contributors with respect, in a format inspired by "How
  to Win Friends and Influence People". I like it because it is a mix
  of personal and professional subject matter, and in a unique voice.

* [Fireflowers: The Rust programming language in the words of its
  practitioners][ff]. A project to capture a particular moment in time
  for the purpose of marketing, while celebrating the Rust community.
  I was proud that I was able to put this simple concept together in a
  single weekend. I also remain satisfied with the visual design,
  though some felt the colors were eyeball-searing.

* [Reddit-flavored Markdown v2][rfmv2]. I was responsible for finalizing the
  implementation of the Markdown Parser for Reddit's 2018 redesign (which was
  itself a fork of [comrak]). This is the public portion of the giant braindump
  I left in my wake.

* [Practical Networked Applications in Rust][pnar]. A course on building
  real-world applications in Rust. Developed for [PingCAP] as a recruiting tool.
  Incomplete, but I have ambitious hopes for it.

* [Refactoring `std` for Ultimate Portability][r]. A proposal for how
  to refactor the standard library into portable components. I like
  how complete the argument is here and that I took the time to make
  illustrations.

* [How Rust is tested][t]. A comprehensive report on one of my
  favorite subjects.

* [The rustup readme][ru]. It's pretty comprehensive and well organized. I'm
  sure others have modified it since, but I wrote most of it.

* [Future updates to the rustup distribution format][rustup]. A description of
  how the rustup infrastructure and protocol works.

* [Starting a new Rust project right, with error-chain][ec]. A simple
  instruction on using the error-chain crate. I like this one because
  I wrote it in an afternoon, where most of my writing takes much
  longer.

* [The End of Unsafety][unsafety]. A presentation I gave with Alex
  Crichton. The repo contains a complete [script], for my parts at
  least. I was quite satisfied with the theme, aesthetics, and
  attention to detail in the deck, particularly the animated ["balloon
  borrowing" demonstration][bb].

* [An annotation of the Rust standard library][astd]. This was a small
  project to illustrate some of the neat little mysteries and hacks
  employed by the standard library. I like it mostly because it uses
  the GitHub pull request review as the medium.

* [The "North Star" RFC][ns]. I mostly stay away from the RFC process
  because the open debate is unpleasant to me, but this is one of the
  better I've written. It lays out a process for establishing the Rust
  roadmap. Our actual conformance to the process established therein
  ended up being quite low.

* [Redesigning the Rust runtime][rt]. Back when Rust had green threading and the
  runtime was written in Rust, this was my plan to rewrite it in Rust, which I
  subsequently did with Ben Blum. In retrospect I didn't know much about
  building thread schedulers, but I still saw the runtime through two rewrites,
  and it all worked. All that code is long gone now though.

* [Abandoning segmented stacks in Rust][stacks]. This was the moment that Rust
  began its shift away from green threading to native threading. At the time we
  naively hoped we could still do green threading by having the OS map stack
  pages lazily, but nope. Bye, bye, green threads.

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
  the community management aspect. The 1.15 release had a glaring
  soundness bug, and this is the mea culpa. The resulting 1.15.1
  release _itself_ was difficult, and resulted in a [post mortem].

* [Rust CI / release infrastructure changes][ci]. Just technical
  communication about CI.

* [How the Rust issue tracker works][i]. A modest exposé of tribal
  knowledge.

* [The Rust Standard Library][std]. I wrote the crate-level
  documentation for the standard library, that is, the stuff that is
  on the landing page. This is a link to 1.18, which is current
  as of mid-2017. It could use a refresh, and I'm surprised it's
  lasted as long as it has.

* [The Rust compiler 0.1 is unleashed][0.1]. An announcement of the
  first release of Rust, in 2012. It's small, but I'm proud that I got
  to make that announcement. I'm also proud of the second paragraph,
  that _explains what Rust is_, a simple thing that many release
  announcements omit. I'm good at anticipating important details like
  that. I also remain tickled that I got away with writing "unleashed"
  in the title. That would not fly today.

* [prev.rust-lang.org]. I'm responsible for a large amount of the
  content on the Rust website (as of 2017), including the
  (increasingly outdated) [contribution guides], the [install pages],
  and the long-standing tagline, "Rust is a systems programming
  language that runs blazingly fast, prevents segfaults, and
  guarantees thread safety". I also am largely responsible for the
  visual design, which admittedly is highly-derivitive of stock
  bootstrap, but I'm quite proud of my aesthetic sense, and ability to
  work within my own limitations. Plenty of people have criticised the
  Rust website over the years, but it does what it needs to pretty
  well. Ask me about my feels re the current Rust website.

[0.1]: https://mail.mozilla.org/pipermail/rust-dev/2012-January/001256.html
[151]: https://blog.rust-lang.org/2017/02/09/Rust-1.15.1.html
[astd]: https://github.com/brson/annotated-std-rs/commit/e50c2b16455ceff29488bf1f058b6c10906ef990
[bb]: https://brson.github.io/the-end-of-unsafety/#/BAL
[blitz1]: https://blog.rust-lang.org/2017/05/05/libz-blitz.html
[blitz2]: https://internals.rust-lang.org/t/rust-libz-blitz/5184
[ci]: https://internals.rust-lang.org/t/rust-ci-release-infrastructure-changes/4489
[comrak]: https://github.com/kivikakk/comrak
[contribution guides]: https://prev.rust-lang.org/en-US/contribute.html
[ec]: http://brson.github.io/2016/11/30/starting-with-error-chain
[ff]: https://brson.github.io/fireflowers
[httptest]: https://github.com/brson/httptest
[i]: https://internals.rust-lang.org/t/how-the-rust-issue-tracker-works/3951
[install pages]: https://prev.rust-lang.org/en-US/install.html
[nice]: http://brson.github.io/2017/04/05/minimally-nice-maintainer
[ns]: https://github.com/rust-lang/rfcs/blob/master/text/1728-north-star.md
[post mortem]: https://internals.rust-lang.org/t/rust-1-15-1-release-postmortem/4766
[prp]: https://internals.rust-lang.org/t/perfecting-rust-packaging/2623
[r]: https://internals.rust-lang.org/t/refactoring-std-for-ultimate-portability/4301
[rfmv2]: https://www.reddit.com/wiki/markdown
[rt]: https://brson.github.io/2013/02/02/redesigning-the-rust-runtime
[script]: https://github.com/brson/the-end-of-unsafety/blob/master/script.md
[stacks]: https://mail.mozilla.org/pipermail/rust-dev/2013-November/006314.html
[std]: https://doc.rust-lang.org/1.18.0/std/
[t]: https://brson.github.io/2017/07/10/how-rust-is-tested
[tls]: https://groups.google.com/d/msg/mozilla.dev.servo/3mfkRehXAo4/MXTzpqsFAQAJ
[unsafety]: https://brson.github.io/the-end-of-unsafety/#/INTRO
[prev.rust-lang.org]: https://prev.rust-lang.org
[ru]: https://github.com/rust-lang/rustup.rs/blob/master/README.md
[PingCAP]: https://pingcap.com
[pnar]: https://github.com/pingcap/talent-plan/tree/master/rust
[rustup]: https://internals.rust-lang.org/t/future-updates-to-the-rustup-distribution-format/4196#the-static-rust-lang-org-layout