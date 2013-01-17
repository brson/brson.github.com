---
layout: post
title: "Emulating try/finally in Rust"
tags: [rust]
---

Over the last week I started writing some new runtime code in Rust,
and was reminded of how awkward it can be to execute code on failure.
In Go you might call a function with `defer`,
or in Java `try` then `finally`:

[sometimes]: https://github.com/mozilla/rust/issues/1523


     try {
         do_some_fallible_work();
     } finally {
         but_alway_run_this_function();
     }

In Rust you have to put that code into a finalizer,
something like this:

    struct Finally(());

    impl Finally: Drop {
        fn finalize(&self) {
            but_always_run_this_function();
        }    
    }

    let _finally = Finally(());

    do_some_fallible_work();

There's not a lot going right in that sequence of code.
It's awkward, and I've written things like it many times.
So to break that habit I recently tried [again] to create
something that looks like `try`/`finally`
and found that Rust can at last express it in a way that I like:

[again]: https://github.com/mozilla/rust/issues/1523

    do (|| {
        some_fallible_work();
    }).finally {
        but_always_run_this_function();
    }

That looks nice enough to use, sparingly at least.
Those extra parens are required to disconfuse the parser,
but I can imagine the grammer being tweaked so they can be omitted.
At first glance it is a somewhat mysterious string of code,
but it's clear what's going on if you break it down a little:

    let try_fn: &fn() = || {
        some_fallible_work();
    };
    do try_fn.finally {
        but_always_run_this_function();
    }

We're using a `do` expression to call `finally`, a method on the closure type `&fn()`.
Finally takes as an argument a block of cleanup code to execute on exit.
From there one could use the existing `task::failing` function to determine if the task
is unwinding due to a failure.

    do (|| ...).finally {
        if task::failing() {
            monitor.send(HelpIAmFailing);
        }
    }

The implementation is simple (note this only works on Rust's incoming branch).

    pub trait Finally<T> {
        // FIXME #4518: Should not require a mode (+) here
        fn finally(&self, +dtor: &fn()) -> T;
    }

    struct FinalizerStruct {
        dtor: &fn()
    }

    impl FinalizerStruct: Drop {
        fn finalize(&self) {
            (self.dtor)();
        }
    }

    impl<T> &fn() -> T: Finally<T> {
        fn finally(&self, +dtor: &fn()) -> T {
            let _d = FinalizerStruct {
                dtor: dtor
            };

            (*self)()
        }
    }

Depending on your experience with Rust that small bit of code may
contain a few gotchas (like applying parentheses when calling a closure in a field, e.g. `(self.dtor)()`),
but one thing I want to point out specifically relates to the name of variable `_d`.
This is the value that holds the FinalizerStruct,
which will run its finalizer when it goes out of scope,
in turn calling the `finally` block.
Since it's only being created to run the finalizer it is never accessed directly.
If we gave it the name `d` Rust would emit an unused variable warning,
but prefixing the name with `_` turns that warning off.


That's useful, but not the interesting bit. One might be tempted to shorten it even further,
replacing the variable with `_`:

    let _ = FinalizerStruct {
        dtor: dtor
    };

Since `_` is a pattern that means 'ignore this value',
this is a perfectly legal thing to write.
It also means that `_` is not a variable and does not keep the struct alive.
If you were to write this the 'finally' block would run before the 'try'.
This is a hazard and a reason to prefer using a `finally` method
instead of open coding a deferred block with a struct.

So this `Finally` trait is simple.
Both of the closures involved are stack closures and the struct is
created on the stack.
With inlining hints this little abstraction should mostly optimize away.
The news isn't all good though,
as the implementation takes advantage of an [unsoundness] in Rust's finalizers,
here relating to the following facts:

* Types containing borrowed pointers can have finalizers
* Types containing borrowed pointers can be put into managed boxes
* Finalizers on managed boxes may run an arbitrary time after the box becomes unreachable

[unsoundness]: http://smallcultfollowing.com/babysteps/blog/2013/01/17/destructors-and-finalizers-in-rust/

Once this hole in the type system is fixed it may not be possible to implement `Finally` with safe Rust code.
Fortunately the interface itself is sound so can still be implemented with unsafe code.

This hasn't been committed to incoming,
but I hope something like it is added to the standard library.
