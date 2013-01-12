---
layout: post
title: "New for loops in Rust"
tags: [rust]
---

How to handle iteration in Rust has been a hotly debated topic for
many months. We have several loop constructs but generally prefer to
loop using higher order functions. A recent [change by marijn][1] makes
`for` aware of how to iterate using higher-order functions and
stack-based lambdas, while still allowing loop control flow using
`break`, `cont` and even `ret`.

The bottom line is that you can now write:

    for vec::each(v) {|elt|
        alt elt {
          some(143) { cont; }
          some(val) { io::println(#fmt("%?", val)) }
          none { ret "done"; }
        }
    }

and it will behave like you expect: `cont` proceeds to the next iteration,
`break` ends the loop, and `ret` returns from the outer function scope.

If, instead of a vector, you had a list to iterate over, then you would
write something like `for list::each(l) {...`. Whereas previously `for`
only worked for specific builtin types (vec and str), the new `for` will
work on any higher-order function with the appropriate signature (in this
case `vec::each` has the right signature).

### How does it work?

The `for` keyword is now followed by a call to a function where the last
paramater is a lambda block, using the lambda block call sugar. As a
refresher, Rust permits some syntactic sugar to make higher order functions
look more like control structures so `each(v) {|i| ... }` is the sugared
form of `each(v, {|i| ... })`.

To be used in a for loop the lambda block must have a `bool` return type
which is used to indicate whether the loop should continue (true) or break
(false).

The following is a typical higher-order `each` that breaks early by
returning `false`:

    each(v) {|elt| if elt == 0 { true } else { false } }

The for loop syntax adds even more sugar to that though and will let you
write `cont` and `break` which behave as though the lambda block returned
`true` or `false`. This is equivalent to the above example:

    for each(v) {|elt| if elt == 0 { cont; } else { break; } }

In the absense of a value to return, `for` loops will modify the the
lambda block as if it resulted in `true` (or `cont`):

    for each(v) {|elt|
        // implicit `cont`
    }

_But that's not all!_

The new for syntax even allows you to write `ret` statements that
return not from the lambda but from the outer function just like you
expect from any other loops. It does this by implicitly stashing the
return value into a captured stack variable, causing the `for` loop
to break, then returning the stashed value. Writing

    for each(v) {|elt|
        if elt == 0 { ret "foo"; }
    }

behaves (more or less) as if you had written

    let retval = none;
    for each(v) {|elt|
        if elt == 0 { retval = "foo"; break }
    }
    if retval { ret retval.get(); }

As an example of how to write functions that are compatible with `for`
here is how `vec::each` is currently implemented:

    #[inline(always)]
    fn each<T>(v: [const T], f: fn(T) -> bool) unsafe {
        let mut n = len(v);
        let mut p = ptr::offset(unsafe::to_ptr(v), 0u);
        while n > 0u {
            if !f(*p) { break; }
            p = ptr::offset(p, 1u);
            n -= 1u;
        }
    }

The old form of `for` is deprecated and will likely be removed from the
language.

[1]: https://mail.mozilla.org/pipermail/rust-dev/2012-March/001490.html