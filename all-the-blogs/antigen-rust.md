---
layout: post
title: "Antigen Rust"
tags: [rust]
---

A safe and fast language for the most highly-constrained systems,
compatible with C, compatible with Rust.

Rust does not work well for all systems programming purposes yet. In
particular, while people _are_ having success on microcontrollers, in
operating systems, and in other constrained environments, some aspects
of Rust are suboptimal for such systems. Antigen Rust is a project to
explore that design space, with the goals of informing future Rust
evolution, upstreaming components to Rust.

The boldest difference between Rust and Antigen Rust is that Antigen
Rust does not have generics. Generics require callers to monomorphize
Rust functions, which increases compile time, causes code bloat, and
can't be performed by a C compiler. Antigen Rust is fully
ABI-compatible with C, creating a low-level computing model that has
the performance of C, is memory safe, works in every environment, and
has an ABI that can be fully represented by all languages that can
speak C.

## Features

- __Runtime characteristics of C, not C++__

  Rust is a fantastic "better C++", but a less fantastic C. The
  patterns enabled and encouraged by each language determines to a
  large degree how they behave at runtime, and also how they 'feel' to
  work in. Although the patterns used in Rust and C++ are expressive
  and convenient, their compilation strategy results in code bloat and
  slow compilation. As a guiding principle, Antigen Rust tries to
  be more like C than like C++ (while still being Rusty).

- __Predictible and stable ABI__


- __No monomorphized generics__


- __More virtual dispatch__


- __Custom core and collections crates__


- __Fallible allocations__


- __Automatic emission of C headers__


- __Panic on abort__


- __Full interop with Rust__

