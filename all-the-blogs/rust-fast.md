---
layout: post
title: "Rust is the fastest programming language in the World"
tags: [rust]
---

The evidence is not yet overwhelming, but it will be: Rust is the
fastest programming language in the world.

If that is hyperbolic, I believe it is only slightly. Rust truly is
the fastest programming language in the World, even today, when there
are known deficiencies in Rust's performance. In the relatively near
future it will become undeniable by concientious hackers.

## Real evidence from real software

How can we know that Rust is the fastest programming language in the
World?

- trail of slow https://jackmott.github.io/2017/02/27/trail-of-slow.html

## Real evidence from microbenchmarks

- http://benchmarksgame.alioth.debian.org/u64q/rust.html
- http://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp
- http://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=ifc
- http://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=go
- http://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=swift
- http://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=java
- https://news.ycombinator.com/item?id=13743980
- https://www.reddit.com/r/programming/comments/8jbfa7/naive_benchmark_treap_implementation_of_c_rust/

## Why is Rust so fast (in the small)?

## Why is Rust so fast (in the large)?

## But what about ...?

### C++ does this stuff too!


### Everybody knows Fortran is the fastest programming language?

### C has restricted pointers!

- SIMD

### What about this other programming language?!

At this level, I would suggest that the only languages that matter are
C, C++, Fortran, and now Rust. There are other languages in this
performance tier, like ATS, but as good as those languages may be they
do not have the support, adoption, generality, familiarity, quality,
or other characteristics necessary to tempt developers away from the
incredibly entrenched, but incredibly error-prone, C and C++
languages. Rust is extremely powerful.

Other languages known for having high performance

- C / C++
- Fortran
- ATS
- Julia
- LuaJIT
- Felix
- Pony
- Nim
- Zig

## Mark my words!

Rust is the fastest general purpose programming language in the world
today. In limited cases a few other programming languages can match,
or even beat, Rust's performance in the small — primarily due
to lack of stable SIMD intrinsics — but it's rare that this
matters in practice. In practice, characteristics of the Rust type
system steer authors toward performant architectures that are
impractical in any other language, and this pays off consistently in
real software written by typical programmers, not just in benchmarks. In
the future Rust will almost certainly introduce optimizations that
have never been possible before in production compilers, cementing the
case that __Rust is the fastest programming language in the world__.

## Other links

https://dennisforbes.ca/index.php/2017/03/03/rust-faster-than-c-not-so-fast/
https://www.reddit.com/r/programming/comments/5xdk4x/comment/dehgdl6
https://www.reddit.com/r/programming/comments/5xdk4x/comment/dehyv6g

https://arxiv.org/abs/1702.02951 - Rust in astrophysics

Shows Rust matching fortran at n-body

Aparently the experiment is flawed https://news.ycombinator.com/reply?id=13635226&goto=item%3Fid%3D13632894%2313635226

https://bitbucket.org/ewanhiggs/csv-game

http://blog.burntsushi.net/ripgrep/

http://benchmarksgame.alioth.debian.org/u64q/rust.html

https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1

https://www.reddit.com/r/rust/comments/6jt4l9/rayon_gets_parallel_sorts_benchmark_against_java/

https://www.fpcomplete.com/blog/2017/07/iterators-streams-rust-haskell
https://www.reddit.com/r/rust/comments/6mf2sz/iterators_and_streams_in_rust_and_haskell/

http://robert.ocallahan.org/2018/03/speeding-up-dwarfdump-with-rust.html

https://www.reddit.com/r/rust/comments/8pnv3k/crazy_performance_story/
  - several anecdotal reports here