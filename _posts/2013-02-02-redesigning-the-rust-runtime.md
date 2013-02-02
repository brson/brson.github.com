---
layout: post
title: "Notes on the Rust runtime"
tags: [rust]
---

There have been a number of discussions lately about fixing various aspects of the rust runtime,
and over the next year it looks like we're going to be making a lot of changes that should
simplify the implementation and improve performance.

'The Rust runtime', which I'll try to define later, provides a number of services with sometimes complex
interactions, so there's a lot to think about when making big changes.
Much of the code involved was written long before Rust was even self-hosting, in C++.

My personal long term goal is to have a well factored Rust runtime,
with a simple and fast implementation written in Rust (typical Rust, not some massively unsafe dialect).
I think we have the tools in the language to do this now.
In the short term I'm going to be focusing on writing a task scheduler in Rust that [is driven by a generic event loop](https://github.com/mozilla/rust/issues/4419),
then make our non-blocking I/O as fast as possible by integrating it into the scheduler.

This post mostly for my own sake, organizing some thoughts I have on the subject,
and collecting links to related issues and discussions.

## Defining the current runtime

First I want to capture the broad architecture of the current runtime,
then describe some of the problems it has.

Today, there is a library written in C++ called `librustrt`, and all crates depend on it.
It contains a function called `rust_start` that begins execution of a Rust program by
passing a Rust function pointer to a new instance of `rust_kernel`.
`rust_kernel` is the runtime, and it manages a dynamic number of `rust_scheduler`s,
each of which in turn manages a fixed number of `rust_sched_loop`s that schedule `rust_task`s.
No Rust code can run outside of a task.

When Rust code needs to access a runtime service it first aquires a pointer to
its `rust_task` instance from thread-local storage.
If it needs to it can get a `rust_kernel` pointer from the `rust_task`.
The runtime exports an interface to Rust through C functions,
mostly defined in `rust_builtin.cpp` and `rust_upcall.cpp`,
most of which encapsulate the lookup of the task pointer.

Services provided by the runtime can be broadly categorized as 'task' services, and 'kernel' services.
Task services have task-local effects and kernel services have kernel-global effects.

Kernel services:

* The exchange heap
* Logging
* Task counting - an atomic count of all tasks, used to trigger shutdown
* Scheduler registration - schedulers are identified by ID and stored in a table
* Default scheduler - all tasks are spawned by default onto a specific scheduler, stored in the kernel
* Platform scheduler - a special scheduler that runs on the `main` thread
* Environment settings - various environment variables are captured at startup and propagated around
* at_exit - runs Rust functions in tasks during shutdown, for building other runtime services in Rust, like weak tasks and 'global data'
* Global data - kernel wide keyed data, mostly implemented in Rust
* Weak tasks - tasks that don't keep the runtime alive

Kernel services necessarily involve global thread synchronization,
so are potential bottlenecks, and good candidates for removal and simplification.

Task services:

* Local heap
* Scheduling - yielding, blocking, signaling
* Linked failure - tasks propagate failure flags around in complex ways
* Stack growth - maintaining a linked list of Rust stack segments
* Stack switching - when calling into foreign code we request a special large stack to run on
* FFI - Rust's FFI is tightly coupled to stack switching currently
* Task-local data
* Unwinding - the task maintains some state during unwinding and performs some cleanup after

I also consider parts of `core::pipes` and `core::task` to be part of the runtime,
because they contain a lot of private implementation details built on the services
exported from `librustrt`.
The GC (such as it is) is also written in Rust, but considered part of the runtime and depends on implementation details
of the local heap.
`librustrt` additionally includes some third party code that is not tied to `rust_kernel`
and might or might not be considered part of the runtime.
`linenoise` for instance is a line reading library used by `std`, and only
lives in `librustrt` as a convenience.

## Problems

### Too much foreign code

The runtime is the only part of Rust not written in Rust.
The interface between Rust and the Rust runtime consists of a handful
of C functions taking opaque pointers,
so can't take advantage of any of the Rust's nice features.
The runtime could evolve much faster if it was written in Rust.

### Task context

There are a lot of use cases for running Rust code outside of a Rust task,
one of which is for writing the scheduler itself,
but it's impossible to do anything without a `*rust_task`.
We need a finer seperation of responsibilities.

### Off-thread I/O

Non-blocking I/O must be done in a different thread.
Under the current implementation we dispatch I/O requests to a global `iotask`,
but this is thought to have a lot of overhead.

### Locking bottlenecks

The current runtime is arranged as a heirarchy of entities,
with the kernel coordinating schedulers that in turn manage a number of threads.
Each level of this heirarchy contains one or more locks or atomic operations,
and simply spawning a task hits several of them.

### Stack management complexity

Most of the `rust_task` code is dedicated to managing stacks,
even though the stack has little to do with tasks and scheduling.
There are two completely separate code paths for extending the Rust stack
('stack growth') and running foreign code ('stack switching'),
but these two use cases have very much in common.

### Lifecycle management and internal complexity

It's quite difficult to trace through the relationship between the kernel,
schedulers, scheduler threads and tasks.
The conditions for triggering runtime shutdown and cleaning up the schedulers
have been particularly hard to reason about.
Linked failure, e.g. is still done by propagating a flag around task objects
and checking for it in the runtime when it could be expressed through higher-level
policies involving pipes.
A lot of the complexity is for historical reasons and exacerbated by the
inexpressiveness of interfaces between foreign code and Rust.

### Task migration

Tasks never [migrate between threads](https://github.com/mozilla/rust/issues/3095).

### Logging

Rust's logging implementation is very old and lives in the C++ runtime.
It should be completely rewritten to operate at a much higher level,
using tasks and pipes.

## Redefining the Rust runtime

In order to write the scheduler in Rust we need to be able to execute Rust code
outside of a Rust task.
Doing this requires some reconsideration of what the runtime is,
in particular we need access to some services globally.
Here is how I am starting to think of the runtime and it's components,
even if this doesn't reflect current reality.

* `core` is the Rust runtime.
* When linking to the runtime you are either running in 'global context' or 'task context'. Most code will never want to use 'global context' because it is very limited, but it is essential for integration with other systems.
* The runtime provides various services, some only available in particular contexts
* Code that doesn't link to `core` (nor anything else) is 'freestanding'. We will probably not achieve this for a long time, but refactoring the various runtime services so they don't all require task context is a big step in that direction.
* Schedulers are single threaded
* Schedulers may work in groups to load balance work
* There is no kernel entity, though there is a minimal amount of shared state,
  probably retaining the name 'kernel state' for lack of a better word and to distinguish it from truly global state

In the new regime `core` is the Rust runtime and librustrt doesn't exist.
Some runtime services are accessible globally,
the exchange heap being the most important.
Another might be a global fallback console logging service.
Some features may need to detect whether they are running inside or outside task context
and change their behavior to use either a task, kernel, or global service as appropriate.
For example I have a change that makes the FFI work either with or without a task.

Some day I would like all these services to be organized as such (global, kernel, task, etc.),
all documented and living in intuitive places instead of being scattered all over core.

## Current work

I've recently deleted the old message passing system, `oldcomm`, from the tree,
and that took with it a big chunk of runtime code.
With that out of the way I feel prepared to start prototyping a new scheduler.

I intend to begin by creating a very simple single-threaded `Scheduler` that schedules
`Task`s and is driven by a generic `EventLoop` trait implemented with uv.
Once the scheduler is properly creating and scheduling tasks then I will start experimenting
with integrating I/O.
My goal will be to make the single-threaded case very fast,
capture that performance in benchmarks,
then extend that work to groups of work-stealing schedulers.
The new code will be scheduler-centric, with one scheduler per thread,
instead of schedulers being a group of scheduler threads.
And instead of having a heirarchy of entities (kernel -> scheduler -> scheduler_thread -> task)
I will aim have just a federation of schedulers,
some of which steal tasks from each other.
In the process of rebuilding the scheduler from the ground up I hope to be able to discard
some of the assumptions made by the previous,
ending up with something simpler.

I intend to write the scheduler itself using mostly typical Rust abstractions,
so pretty soon I will want to port pipes to pthreads.
For the single threaded case they won't be needed though - only once schedulers need to talk to each other will it matter.

I'm still thinking about how I want these runtime services to be represented and organized,
but as I work on the scheduler I will probably create some sort of `RuntimeService`
type that makes it clear what runtime capabilities your Rust code has access to at any time.

Tasks on deck:

* Prototype a single-threaded scheduler with integrated uv event loop
* Upgrade libuv (pfox__ is working on this)
* Port pipes to pthreads
* Simplify stack management and FFI in the existing runtime

## Links

* [Discussion of scheduler-I/O integration](https://mail.mozilla.org/pipermail/rust-dev/2013-January/002958.html)
* [Discussion of foreign stack switching](https://mail.mozilla.org/pipermail/rust-dev/2013-January/003013.html)
* [My current preferred runtime interface to the stack](https://mail.mozilla.org/pipermail/rust-dev/2013-February/003069.html)
* [Recent pull request making foreign calls and the exchange heap work globally](https://github.com/mozilla/rust/pull/4619)
* [Platform event loop integration ('platform thread') issue](https://github.com/mozilla/rust/issues/2058)
* [Integrating uv and the scheduler issue](https://github.com/mozilla/rust/issues/4419)
* [Freestanding Rust](https://github.com/mozilla/rust/issues/3608)
* A sampling of the problems with stack management: [stack1], [stack2], [stack3]
* [Python PEP 3156 - Asynchronous IO Support Rebooted](http://www.python.org/dev/peps/pep-3156/)

[stack1]: https://github.com/mozilla/rust/issues/4480
[stack2]: https://github.com/mozilla/rust/issues/4479
[stack3]: https://github.com/mozilla/rust/issues/1804
