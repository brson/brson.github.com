---
layout: post
title: "A land of little locks"
tags: [rust]
---

Through an analogy with reader-writer locks you will understand that
the single thing that makes Rust so powerful is it reasons strongly
about the aliasability of memory. Reader-writer locks allow either a
single writer at a time *or* many readers, but never both readers and
writers.

Rust's shared and mutable references describe a system of statically
verified, fine-grained, reader-writer locks over a graph (usually a
tree) of memory. They are extended through the `Send`, `Sync`, and `Deref`
traits to integrate *dynamic* locking of memory that the compiler
statically sees only as potentially aliased.

Rust has two main pointer types,`&mut T`, and `&T`. The former is a
*mutable reference*, which grants its owner the permission to write to
its referent. The second type of pointer, `&T`, is sometimes called an
*immutable reference*, though not usually, as we'll come to
momentarily. The Rust compiler loves these pointers because they are
the tools it uses to know when it is ok to read or write to any given
region of memory.

In Rust, a mutable reference is the ultimate power over a tree values.
With an `&mut T` you know that there are no other readers or
writers of that memory, nor are there readers or writers of any memory
that is transitively uniquely owned by the reference. With very few
exceptions, every mutation of memory in Rust is through a mutable
reference. Every time you call a mutating method on a value, it's
first silently borrowed to a mutable reference.

I want to show you this.

Rust does several types of automatic coerceions to reduce boilerplate.
In examples, I will be very explicit with the type conversions, to
show exactly what is happening. I'm going to use locking terminology
a lot, though that is not how you would normally talk about references
in Rust.

We're going to use the same basic setup for all the
examples. Ok, take this in.

```rust
// allow(warnings) will make rustc significantly less
// complainy about dead code, unused imports, and other boring
// things. Damn, rustc, just let us have some fun.
#![allow(warnings)]

// Rust is so stupendously amazing
struct Age(i32);
struct Weight(i32);

// A typical brie is aged 5 weeks and weighs 6 oz.
let mut precious_data = (Age(5), Weight(6));

fn read_age_and_weight(_: &(Age, Weight)) { }
fn write_age_and_weight(_: &mut (Age, Weight)) { }

fn read_age(_: &Age) { }
fn read_weight(_: &Weight) { }

fn write_age(_: &mut Age) { }
fn write_weight(_: &mut Weight) { }
```

`precious_data` is a tuple that we are going to want to read and write
in different sequences. It lives in a mutable slot on the stack, so it
can be borrowed mutably, or immutably even. We've got some functions
that either write or read, taking their argument by mutable or
immutable reference.

TODO: think of references as locks.

So to write to our data we must procure a mutable writer lock
on `precious_data`, because `write_age_and_weight` takes `&mut (Age, Weight)`.

```rust
let mut precious_data = (Age(5), Weight(6));

// Take a writer lock
let writable_precious_data: &mut _ = &mut precious_data;
write_age_and_weight(writable_precious_data);
```

This is the most basic of borrows. Mutably dereference the mutably
owned value, `precious_data`. This is a *writer lock*. It is
guaranteed to suceed because `precious_data` is uniquely owned so
nothing can stop us from writing to it.

For the duration of the writer lock, which is the same as the lifetime
of the borrow, it's impossible to acquire another writer lock, nor a
reader lock. The following does not compile.

```rust
let mut precious_data = (Age(5), Weight(6));

// Take a writer lock
let writable_precious_data: &mut _ = &mut precious_data;
write_age_and_weight(writable_precious_data);

// Take another writer lock
let writable_precious_data2: &mut _ = &mut precious_data; // ERROR
write_age_and_weight(writable_precious_data2);

// Take a reader lock
let readable_precious_data2: & _ = &precious_data; // ERROR
read_age_and_weight(readable_precious_data2);
```


```sh
$ rustc land-of-little-locks.rs

land-of-little-locks.rs: error: cannot borrow `precious_data` as mutable more than once at a time
land-of-little-locks.rs: let writable_precious_data2: &mut _ = &mut precious_data;
                                                                    ^~~~~~~~~~~~~
land-of-little-locks.rs: error: cannot borrow `precious_data` as immutable because it is also
                                borrowed as mutable
land-of-little-locks.rs: let readable_precious_data2: & _ = &precious_data;
                                                             ^~~~~~~~~~~~~
```

Once that mutable reference, `writable_precious_data`, takes the
stage, its referent can't be accessed from any other path. Since the
mutable reference has unique access to its referent its trivial to
convert it to an immutable reference as long as the compiler proves
there is no further mutation during its lifetime. To do so is
simply to atomically reacquire the writer lock as a reader lock.
In Rust terms we reborrow the reference, with the borrow operator
and the dereference operator, `&*`.

```rust
let mut precious_data = (Age(5), Weight(6));

// Take a writer lock
let writable_precious_data: &mut _ = &mut precious_data;
write_age_and_weight(writable_precious_data);

// Reaquire as a reader lock, reborrowing.
let readable_precious_data: & _ = &*writable_precious_data;
read_age_and_weight(readable_precious_data);

// Take a second reader lock
let readable_precious_data2: & _ = &*writable_precious_data;
read_age_and_weight(readable_precious_data2);
```

So after handing out those immutable references we can't do any more
writing through the original mutable `writable_precious_data`.  That's
how Rust ensures memory safety: during any given scope, memory
is always mutated TODO

Locks are released when they go out of scope, that is, when
the lifetime of the references end. So to write again
to `precious_data` after taking the reader locks, those
locks can be enclosed in their own scope after which they
will be released.

```rust
let mut precious_data = (Age(5), Weight(6));

// Take a writer lock
let writable_precious_data: &mut _ = &mut precious_data;
write_age_and_weight(writable_precious_data);

{
    // Aquire the reader lock
    let readable_precious_data: & _ = &*writable_precious_data;
    read_age_and_weight(readable_precious_data);
    // Release the reader lock
}

// After the reader locks have released, the original writer
// lock can be used again, either directly or by reborrowing.
let writable_precious_data2: & _ = &mut *writable_precious_data;
write_age_and_weight(writable_precious_data2);
```

TODO: Show iterator invalidation

So it's obvious that Rust's novelty is that it divides all memory into that
which can be written and that can be read, and it does this with no runtime overhead,
right? Rust gives control over mutability.

No. While maintaining invariants about mutability and immutability is one of the
most important results of the Rust ownership model, ultimately the borrow checker
is not tracking mutability. The borrow checker cares about *aliasability*.
The distinction between the two is critical, without which much of Rust's
expressivity as a safe *concurrent* language would not be possible.

## Dynamic locking with `RefCell`





# Notes

RefCell knows dynamically how to ensure that aliased memory is unaliased for a little while.

It's not often that you want unique aliasing of memory for reasons
*other* than writing to it, so as a concession to familiarity Rust
calles unique references 'mutable references', even though what it's
really providing is subtly but crucially different.

Deref trait makes it possible to integrate custom dynamic locks.

Reference -> RefCell -> RwLock

Both RefCell and RwLock are implemented with UnsafeCell.

`&T where T: Sync`

`&mut` references are *always guaranteed* to be unaliased for the
duration of the reference, which means it will not be read or written
through any other pointer, nor be read or written by the value's
owner, from any thread. 

