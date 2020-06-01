---
layout: post
title: "Understanding Rust 2018"
tags: [rust]
---

- https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html
- https://www.ncameron.org/blog/rust-2018/
- https://blog.rust-lang.org/2018/03/12/roadmap.html
- https://blog.rust-lang.org/2018/07/27/what-is-rust-2018.html

# Subjects

- what _all_ features are in 2018? How do you find that info?
- `try` - https://github.com/rust-lang/rfcs/pull/2388
- async / await
- uniform paths https://rust-lang-nursery.github.io/edition-guide/rust-2018/module-system/path-clarity.html
  - crate keyword
- macro imports https://rust-lang-nursery.github.io/edition-guide/rust-2018/macros/macro-changes.html
- pub(crate) struct Foo;
  - pub(in a::b::c) struct Bar;
- nested use statements
- ? in main
  - which cases is it suitable for?
  - https://rust-lang-nursery.github.io/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
  - failure crate
  - Termination trait

resume research here https://rust-lang-nursery.github.io/edition-guide/rust-2018/control-flow/index.html
