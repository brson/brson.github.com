---
layout: post
title: Powerletters for Rust
tags: [rust]
---

There are a bunch of methods we call in Rust all the time
that do a small, common thing, with too much visual noise.
Think `clone` and `to_owned`. Etcetera.

For quite a few months now I've been using some convenience functions
to make a few common Rust operations less visually cluttered,
and I'm finding I like it - every time I see an `unwrap`
I wish I had my … POWERLETTERS!

Haha.

It's super simple, as described in
[the crate docs](https://docs.rs/powerletters).
The powerletters are single all-caps functions and methods:

- `C` - `Clone`
- `O` - `ToOwned`
- `S` - `ToString`
- `I` - Ignore `Result`
- `X` - `expect` for `Result` and `Option`

In action:

```rust
use powerletters::*;

// Create some owned strings
let bagostuff = vec![S("a"), S("b"), S("c")];
// Clone a vector
let newbag = bagostuff.C();
// Unwrap an option we know is Some
let maybe_bag = Some(newbag);
let stuff = maybe_bag.X();
```

Pretty obvious stuff.
The only non-obvious powerletter is `I`.
It doesn't correspond to a trait or method.
It instead makes the `let _ = some_result` pattern typesafe.
And less of a syntactic odball.

Consider the example:

```rust
let _ = do_something_important();
```

What is actually going on here?
Probably ignoring a `Result`?
What is the return type of `do_something_important`?
It doesn't matter; we're ignoring it.

What if we changed `do_something_important` from

```rust
fn do_something_important() -> Result<(), Box<dyn Error>>
```

to

```rust
async fn do_something_important() -> Result<(), Box<dyn Error>>
```

Now we're just silently dropping a future
and it looks intentional.
It's happened to me!

How about this:

```rust
do_something_important().I();
```

Anyway enjoy some powerletters in action:

```rust
let mut parents: Vec<_> = path.ancestors().skip(1).map(|path| {
    if path == Path::new("") {
        let parent_path = pathgen.from_path(path).X();
        let parent_label = S("<root>");
        (parent_path, parent_label)
    } else {
        let parent_path = pathgen.from_path(path).X();
        let parent_label = path.iter().last().X().to_string_lossy().S();
        (parent_path, parent_label)
    }
}).collect();
```