---
layout: post
title: "Writing Rust in easy-mode"
tags: [rust]
---

Rust has a reputation for being difficult to learn and to write.

I write a lot of Rust, some most every day. I think writing Rust is a lot of
fun, but I have learned how to write Rust in a way that is simple, and avoids
the frustrating complexities of its advanced type-system features.

Here I'll try to explain some of the techniques I use to write Rust without
dealing with the hard stuff.

I am going to focus on writing command line programs, since that is what I find
myself writing the most in Rust. Rust is great for simple command line
applications &mdash; I use it often for the type of one-off, throw-away
experiments that one might use Python for.


## Set up the build environment

I have a standard Rust development environment that I set up on all my (Linux)
machines. I have it scripted so that every new development machine looks the
same and I don't end up going through a familiar process of remembering how to
install my tools lazily as I inevitably need them.

Most of my work is done on Ubuntu 18.04 on EC2, but also under WSL.

I install Rust using [rustup]. There are few good reasons to use other methods.
Many Rust repositories expect to be able to pick the Rust toolchain using
[rustup]. If you install Rust in any other way you will immediately be off the
Rust happy-path.

[rustup]: https://www.rustup.rs

Rust code of significant size tends to depend on a few non-external tools,
the most common of which are [GCC], [CMake], [pkg-config], and [OpenSSL].

[GCC]: todo
[CMake]: todo
[pkg-config]: todo
[OpenSSL]: todo

These can be installed on Ubuntu (and probably Debian) like

```
sudo apt install build-essential cmake pkg-config libssl-dev
```

As a Rust developer, I am accustomed to using various CLI tools written in Rust,
so I also `cargo install` these in every environment:

- [`ripgrep`] - todo
- [`cargo-edit`] - todo
- [`tokei`] - todo
- [`fd-find`] - todo
- [`basic-http-server`] - todo
- [`gist`] - todo

There are [other Rust tools I use sometimes][tools], but I use these often
enough that I always have them installed.

[tools]: https://github.com/brson/my-rust-lists/blob/master/rust-cli-tools.md

A command for this is

```
cargo install ripgrep tokei fd-find basic-http-server cargo-edit
```


## Know your crates

Newcomers to Rust have a difficult time because it is not reasonable to write
good Rust code using just the standard library. Experienced Rust hackers have
invested a lot of trial-and-error to find the best-practice libraries for their
purposes, and the library ecosystem is constantly evolving.

This is a hard guideline to follow. Knowing which crates to use takes time and
experience, and requires keeping up with community news.

There are sites like [awesome-rust] and [lib.rs] that provide a good, if
incomplete, overview of some of the good stuff available, but they tend to be
overwhelming, lacking in guidance, and non-authoritative.

[awesome-rust]: https://github.com/rust-unofficial/awesome-rust/
[lib.rs]: https://lib.rs/

For what it's worth, I [keep a list] of some of the Rust crates I regularly use.

[keep a list]: https://github.com/brson/my-rust-lists/blob/master/rust-libraries.md


## Starting a CLI project fast

When writing a CLI in Rust, if you don't know what you are doing and don't pick
the right tools, it can be very painful just to get started.


## Guidelines

