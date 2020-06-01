---
layout: post
title: "Aimee learns Rust"
tags: [rust]
---

Aimee is trying to build a bot.
She is not a programmer.
She wants the bot to do ... a million things.
Lots of ideas. Downloading lots of other bots.

# Session 1

The MVP is a bot that can help produce RIB.
Get something small and productive first, then expand.

To that end we're going to scrape GitHub pull requests.

She has not read the book.

First task is just to call the github api for listing pulls for a project.

For a prototype we do so using a url with a hard-coded repo name.

She recognizes that we can use the crate-level example from reqwest to call the github api.

reqwest has good docs.

But reqwest by default is async. I explain that we do not want to deal with that and need the 'blocking' apis.

how would a newbie know that?

I say that `await` has to do with asynchronous rust and it's crazy complex and we don't want to deal with it.

We use reqwest::blocking::get().

For now we have main return reqwest::Error, and we use `?` without explanation.

GitHub returns an error saying that the user agent must be set.

need to create a request Client to set http request errors.

We use `println!` to print the result of the api.

We change .text() to .json().

todo


# Session 2

At one point she suggests the next step is to post to twitter. We have not even done anything useful with our github scraping yet.

The next step is to call the github api in a loop, replacing the hard coded url with a url generated from a list of repos.

She suggests this step herself, and she suggests we use a file to store the repo names. I tell her that's a great next step.

I help her decide that the list of repo names should live in the `src/` directory and be included directly in the program with `include_str!`.

I think this is a reasonable and convenient first step for a prototype. Deciding this on own's own, discovering and understanding `include_str!`, as a beginner, would be hard.

She is hasty and often starts typing without knowing what to type.

She is fixated on typing ".pull" in order to turn strings of repo names into urls pointing to the github api.
When I ask her she is not clear on why she keeps typing that.

She keeps getting distracted. I keep trying to focus her attention. I ask "what is the problem we are trying to solve now"?

I tell her what steps to do next. She has problems breaking down the big problem into tiny steps.

It takes a long time to find and us the `lines` iterator on `str`. She understands that the string needs to be broken into lines, but doesn't recognize that the most promising way to do that is to look for a method on `str`.

At the moment she doesn't understand the difference between `str` and `String`. Since we're just starting I explain only the basic differences and tell her we can discuss details later.


# Session 3

adding database support

she did the research, found the postgres crate, found the example on the front page, adapted it for her purposes.
she adds a "create table" query before her main loop, and an "insert" query every time she hits the api.

Good girl!

I notice that docs.rs has the orange "see latest version" link. For now I ignore it.

I only point out a few syntax mistakes. it's late - i usually let her make her own mistakes and follow
the compiler, but i'm impatient right now.

# Session 3

task: try to build 

#[non_exhaustive] feature trying to compile postgres crate.

"experimental feature" - what's that?

how would she figure this out?

she had RUST 1.39.0, nov 4, pre async/await. i tell her to 'rustup update'

we go to docs.rs and 

after recompiling with the right compiler She has an import collision, postgres Client and reqwest Client. I have her search for "rust import renaming". She finds rust-by-example, which serves the purpose fine. I was hoping she would find the book.

She tries to as-rename postgres::{Client, NoTls} as PostgresClient. Compiler error is a lexer error and not explanatory. I have to explain the difference between single-imports and multiple-imports.

She hits a ? error about converting postgres::Error to reqwest::Error. I have her use unwrap for now and read the Result::unwrap docs.

When she decides to go in her own direction, i remember to step back and breath and let her follow her own ideas.

Now we are trying to insert the raw data into a database. We need to stop using JSON and start using String again.

```
error[E0308]: try expression alternatives have incompatible types
  --> src/main.rs:41:27
   |
41 |         let body: Value = builder.send()?.text()?;
   |                           ^^^^^^^^^^^^^^^^^^^^^^^
   |                           |
   |                           expected enum serde_json::value::Value, found struct std::string::String
   |                           help: try using a variant of the expected enum: serde_json::value::Value::String(builder.send()?.text()?)
```

Terrible error message. I just tell her the solution is to remove `: Value`.

Program compiles.

PostgresClient::connect fails and the program panicks. "password missing".

I walk her through RUST_BACKTRACE=1, she finds the place in main that calls unwind.

We run into the error where calling `CREATE TABLE` twice fails and have to search google for how to conditionally create a table. I give a fair bit of guidance but she more-or-less finds the answer on her own.

