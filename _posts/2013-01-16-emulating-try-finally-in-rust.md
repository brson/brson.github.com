---
layout: post
title: "Emulating try/finally in Rust"
tags: [rust]
---

Over the last week I started writing some new runtime code in Rust,
with the ultimate goal of rewriting the scheduler in Rust (it is currently C++),
and in the process I had to deal with a number of failure scenarios.
The difficulty of doing so was giving me the blues.

TODO

In Go you might write `defer always_run_this_function()`,
or in Java `try` then `finally`:

     try {
         do_some_fallible_work();
     } finally {
         but_alway_run_this_function();
     }

In Rust you're going to write something like this:

    struct Finally(());

    impl Finally: Drop {
        fn finalize(&self) {
            but_always_run_this_function();
        }    
    }

    let _finally = Finally(());

    do_some_fallible_work();

And then your friends are going to laugh at you.
That's a lot of work to call a function.

The `Drop` trait is the only mechanism in Rust for running code during stack unwinding (failure),
but it is designed as a type finalizer and not convenient to use for running arbitrary blocks of code.
This has been [bugging me for a while](https://github.com/mozilla/rust/issues/1523).
I have occasionally tried to implement something like `defer` or `finally` in library code,
but have been unable find a solution that both worked within the confines of Rust's type system looked reasonably appealing.

TODO

I tried yet again to implement `try`/`finally`
and found that Rust can at last express it in a way that I like:

    do || {
        some_fallible_work();
    }.finally {
        but_always_run_this_function();
    }

As written it doesn't quite parse, but I don't know a reason it shouldn't.
In present-day Rust some extra parens are needed to disconfuse the parser:

    do (|| {
        some_fallible_work();
    }).finally {
        but_always_run_this_function();
    }

That looks nice enough to use, sparingly at least.
At first glance it is a somewhat mysterious string of code,
but it's clear what's going on if you break it down a little:

    let try_fn: &fn = || {
        some_fallible_work();
    };
    do try_fn.finally {
        but_always_run_this_function();
    }

We're using `do` to call `finally`, a method on the closure type `&fn`.
`finally` calls the function `&self` (here `try_fn`), afterwards calling `always_run_this_function`.

I would also expect to be able to write `some_fallible_work.finally(but_always_run_this_function)`,
but it does not type check when `some_fallible_work` is a function item (vs. a closure).
Rust considers `some_fallible_work` an `extern fn` - which is misleading if not wrong -
and won't coerce it to `&fn`.

The implementation is simple (note this only works on the incoming branch).

    // Define an impl on a stack closure
    impl<T> &fn() -> T: Finally<T> {
        // FIXME #4518: Should not require a mode here (+)
        fn finally(&self, +dtor: &fn()) -> T {
            // Put the finally block into a struct.
            // The struct will execute it from
            // its finalizer.
            let _d = FinalizerStruct {
                dtor: dtor
            };

	    // The &self type is &&fn() -> T,
	    // so dereference self to call the function.
	    // Function calls have higher precedence
	    // than * so we need parens.
	    // Note that we're passing on the return
	    // value of the call.
            (*self)()
        }
    }

    // Our method must be part of a trait because
    // we did not define the self type, &fn() -> T.
    // Methods that don't implement a trait can only
    // be defined in the crate that defines the self
    // type. Otherwise multiple crates could define
    // conflicting methods, and that's problematic.
    pub trait Finally<T> {
        fn finally(&self, +dtor: &fn()) -> T;
    }

    struct FinalizerStruct {
        dtor: &fn()
    }

    impl FinalizerStruct: Drop {
        fn finalize(&self) {
	    // Method calls and field access are distinguished
	    // the presence or absence of `()`. So here
	    // `self.dtor` is parenthesized to show the parser
	    // that we are calling a closure in a field,
	    // not calling a method.
            (self.dtor)();
        }
    }

Both of the closures involved are stack closures and the struct is
created on the stack. With inlining hints this little abstraction
could concievably optimized to little more than a function call.

I put several comments about nuances in there,
but one thing I want to point out specifically is the name of variable `_d`.
This is the value that holds the FinalizerStruct,
which will run its finalizer when it goes out of scope,
in turn calling the `finally` block.
Since it's just being created to run the finalizer it never gets used directly.
If we just gave it a name  `d` Rust would emit an unused variable warning,
but prefixing the name with `_` turns that warning off.

One might be tempted to shorten it even further,
replacing the variable with `_`:

    let _ = FinalizerStruct {
        dtor: dtor
    };

Since `_` is a pattern that means 'ignore this value',
this is a perfectly legal thing to write.
It also means that `_` is not a variable and does not keep the struct alive.
If you were to write this the 'finally' block would run before the 'try'.
This is a hazard and a reason to prefer using a try/finally
abstraction instead of open coding a deferred block with a struct.

This hasn't been committed to incoming yet, nor has it been reviewed,
but I would expect something like this to show up in the standard libraries
eventually.