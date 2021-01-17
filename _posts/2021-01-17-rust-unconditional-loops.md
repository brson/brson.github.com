---
layout: post
title: Unconditional loops are unconditionally awesome
tags: [rust]
---

Here's a thing I don't see appreciated enough about Rust: `loop`.
I know I don't think about it all that much,
but pretty much every time I use it I feel a bit of satisfaction.

`loop` is just an unconditional loop:
it loops forever, or until you write `break`, or `return`.

Most languages don't have it.

Instead, loop constructs usually have some kind of termination condition,
your `while` and `for` loops.
Apparently _[Sather]_ has an unconditional `loop` keyword like Rust.
I only know this because a programming language historian mentioned it [on the bug tracker][bug2].

Why do I love `loop`?

Because it frees me from thinking about how to end a loop
before I've even started writing it.

Many problems are easy to recognize as ones that require a looping solution:
I quickly realize, "I have to do something multiple times".

Sometimes that loop just involves iterating over a container of things,
one at a time.
That's easy to recognize as a `for thing in things { }` situation.
Other loops have more complex conditions though.
Often when I'm solving a looping problem,
I will know one or some of the steps I intend to do in a loop,
but will not envision the complete solution ahead of time.

So I just write

```rust
loop {
}
```

and start coding what I do know needs to happen,
and work from there.

For some reason this feels great.

Once I have solved my looping problem,
there will be a `break` or `return` somewhere in there,
or multiple `break`s and/or `return`s.
Maybe it makes sense to convert it into a `while` loop,
maybe not. But there's not a great need
for `while` when you've got `loop`, `break`, and `return`.
Do readers really need to know the loop termination condition
before reading what happens in the loop?
Or, in languages with `do { } while (…)` loops,
after the very end of the loop?
I don't know,
but writing the termination condition
naturally wherever it "wants" to live in the loop
seems reasonable to me.
One does though need to be considerate of
their readers by keeping the loop body a readable length.

Anecdotally, a small project I'm working on right now
contains 2 instances of `loop`, 2 of [`while let`],
and 8 of `for … in`;
no standard `while` loops.
And I think the loops read better than if I had
tried to convert them to `while` loops.

[`while let`]: https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html

Here's one example:

```rust
let (tx, rx) = async_channel::unbounded();
let handle = thread::spawn(move || {
    let mut context = FsThreadContext::new();
    loop {
        let msg = block_on(rx.recv()).expect("recv");
        match msg {
            Message::Run(f) => {
                f(&mut context);
            },
            Message::Shutdown(rsp_tx) => {
                context.shutdown();
                rsp_tx.send(()).expect("send");
                break;
            }
        }
    }
});
```

Here's the other:

```rust
let mut header = String::new();
let mut line = String::new();

loop {
    line.truncate(0);
    io.read_line(&mut line)?;

    if line.is_empty() {
        return Err(anyhow!("broken frame header"));
    }

    let maybe_body_marker = &line[..line.len() - 1];
    if maybe_body_marker == FRAME_BODY_MARKER {
        break;
    }

    header.push_str(&line);
}
```

It's common in C-like languages to write an unconditional loop
with `while (true) { }`.

I'm steeped enough in Rust that I don't know if writing `while (true) { }`
brings others the same satisfaction as I get from `loop { }`,
but I suspect not: it looks and feels just like a tiny bit of a hack.
If I go into writing a loop by first writing `while …`
then I am immediately presented with the question,
"while _what_?",
and sometimes I just don't want to think about that yet.

Besides _feeling_ good,
there is [a technical reason that Rust has `loop`][bug]:
it helps analyze control flow.
With it, the compiler can trivially know that any code
after the loop is unreachable.
In Rust at least this is important for type checking.
This kind of analysis _is done_ in other languages,
but by my recollection they sometimes actually
special case `while (true) { }` for this purpose.

Here's an example of the differences in how Rust
treats `loop` vs. `while true` [on the playground][pg].
Run it and check out the warning the compiler
issues; try to alter the example as suggested in the comments.

[pg]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=010dce12fdd4c0818322717e161a2f91

Bonus Rust trivia:
did you know that `loop` is an expression,
with the same type as its `break` statements
and the result of `loop` can be [assigned to a value][lv]?

I did not!

[lv]: https://doc.rust-lang.org/stable/reference/expressions/loop-expr.html#break-and-loop-values

The [issue on the bug tracker][bug] appears
to be the only remains of the design discussion around `loop` in Rust,
though it is insightful to the designers' original thinking.
The [meeting minutes][mins] where it was approved
just say there was consensus to add it.

[bug]: https://github.com/rust-lang/rust/issues/1906
[bug2]: https://github.com/rust-lang/rust/issues/1906#issuecomment-4240501
[mins]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2012-03-06.md
[Sather]: https://en.wikipedia.org/wiki/Sather

```rust
loop {
    println!("Unconditional loops are unconditionally awesome");
}
```
