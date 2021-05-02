---
layout: post
title: Rust's Most Unrecognized Contributor
tags: [rust]
---

I think the Rust language is a big success.
When I think back on it I am in awe:
so much had to go right to get where we are,
and there were so many opportunities to go wrong.
It took many tiny miracles for the Rust language to become what it has.
Those miracles didn't happen by accident though:
each one was created by a real person,
and real people orchestrated them to become something great.

There are many who contributed to Rust becoming what it is.
One of the people most responsible for Rust's success though is almost completely unrecognized.


## The Beginnings of Mozilla Research

By 2009, Mozilla had built up a sizable warchest of cash from its lucrative search deal with Google.
And by my understanding,
management decided it was time to invest that money,
and the company entered a period of rapid expansion.

As part of that expansion Mozilla created a new department, Mozilla Research.
The mission of this department would be to experiment with ambitious new ideas,
independently of the Firefox product,
while establishing relationships with the academic computer science community.

The first big idea Mozilla Research was formed around was [Servo].
And in turn, Rust.

[Servo]: http://venge.net/graydon/talks/intro-talk-2.pdf

One of the people chosen to lead this department was [Dave Herman].

[Dave Herman]: https://github.com/dherman


## Who is Dave Herman?

Dave Herman is a programming language theorist, and a macrologist (someone who super-loves macros),
and was one of Mozilla's representatives on the ECMAScript committee.
Both he and Graydon Hoare,
the engineer who created Rust,
had collaborated on the abandoned [ECMAScript 4] standard.

[ECMAScript 4]: https://en.wikipedia.org/wiki/ECMAScript#4th_Edition_(abandoned)

Both had a big appetite to create programming languages.

There was a lot more going on at Mozilla Research than Servo, Rust, and Dave Herman,
but this is not that story.

This is a story about how Dave Herman quietly shaped the outcome of the Rust project.


## Dave's contributions to Rust

While Rust was announced in June 2010,
work on it inside Mozilla actually started in late 2009.
The only public record of Rust's development at this
time is in the [rust-prehistory] repo.

[rust-prehistory]: https://github.com/graydon/rust-prehistory

During the months leading up to June 2010 there was
a rush to get Rust presentable to the public,
and Dave was one of those hacking on this task:

```
~/rust-prehistory $ git shortlog -sn
  1156  Graydon Hoare
   163  Andreas Gal
   104  Dave Herman
    59  graydon@pobox.com
    55  Patrick Walton
    37  Graydon Hoare ext:(%22)
    13  Roy Frostig
     9  graydon@mozilla.com
     6  Brendan Eich
     5  Michael Bebenita
     1  Brian Campbell
```

Thereafter he ceded coding work to the growing Rust team,
lead by Graydon,
but for the next few years of Rust's development
Dave was always in the room.

Back then,
most of those who worked on Rust actually worked in the same office
(except, most notably, Graydon, who was remote),
and regularly gathered around a single table in some tiny conference room in Mozilla's Mountain View HQ:
a handful of full time staff, a rotating horde of interns, and Dave Herman.

I imagine he thought of himself as a mentor,
gently nudging the team in productive directions,
based on his own experience with ECMAScript,
and based on his own language-design interests.
He never exerted authority in any way,
and to this day does not attempt to claim credit for anything about Rust.

He made his contributions almost entirely behind the scenes,
exercising his influence softly.

A lot of the early debates in that conference room were the fundamental ones,
ones that seem irrelevant today,
trivial things like "what is the keyword to return from a function?",
or less trivial things like "how do you safely hold pointers to a field of a struct?"
The questions from that time feel distant and unimportant today,
but that's because they were debated into the ground then,
and the language repeatedly shaped and reshaped until
all the little questions were resolved into a consistent whole.
Dave though was one of the few people in the room with
experience in the design of real production programming languages,
and Rust would surely have been a mess without him
steering the team through the trivial little language-design obstacles.

Dave's tastes shaped the team's tastes,
which shaped the Rust language.
And so most of the time Dave was happy with the team's decisions.

Although Dave had a say in all early Rust design subjects,
there were a few major subjects that occupied Dave's interests with respect to the Rust language,
and if you know Rust at all,
I suspect they will be familiar to you:

- **Pedagogy**, that is, "the approach to teaching".

  Dave has an academic background, and always considered every design decision in light of how
  it could be taught and understood.

- **Governance**, and community management.

  Rust was designed as a community project from the first moment,
  and I think this was in great respect influenced by Dave's
  experience of language standards bodies.
  There was always the assumption that the most successful languages are not owned,
  but are cooperatively designed by a large group of individuals and corporations,
  all with their own interests and motivations.

  As such the processes Rust was developed under were always intended
  to be as inclusive as reasonable at the time.
  Just as one example,
  much of Rust's earliest development is documented in [published meeting minutes][mins].
  I attribute that almost entirely to Dave's own discipline,
  and know that many of those minutes were recorded by him personally.

  Placing the highest importance on nurturing a language's community may be
  Dave's greatest Rust legacy.

- **Macros**.

  As stated earlier, Dave is a macrologist; he is an expert on macros,
  with a background in the [Racket] language.

  Although implemented mostly by interns, particularly [Paul Stansifer],
  with some crucial contributions from [John Clements],
  it is because of Dave that Rust has powerful and hygienic declarative macros
  (`macro_rules!`).

  Although it's not at all an area I was involved in,
  I recall he and Paul spending many hours debating
  how to design an hygienic macro system in the Lispish tradition
  into the world of C-like languages that Rust inhabits.

[mins]: https://github.com/rust-lang/meeting-minutes
[Racket]: https://racket-lang.org/
[Paul Stansifer]: https://github.com/paulstansifer
[John Clements]: https://github.com/jbclements

Other key decisions Dave had a hand in include:

- Converting Rust from a statement language to an expression language
- Hiring Niko, who designed Rust's ownership system
- Hiring [Yehuda Katz] to design Cargo

[Yehuda Katz]: https://github.com/wycats

Beyond those easily-recognizable contributions,
Dave served another crucial Role in Rust:
being Rust's advocate to Mozilla management.

Of all the miracles that led to Rust,
perhaps the greatest was that Mozilla paid for so much of it.
During Rust's entire existence at Mozilla
there was a clear sense within the team that the project could be canceled at any time.
This was especially true once Brendan Eich &mdash; who was solidly on team Rust &mdash; left Mozilla.
This is one of the reasons it was so imperative to create a strong community around the language &mdash;
the clock was always ticking.

Dave was a Rust believer though,
the highest-positioned in the company.
And he did everything he could to advocate the importance of Rust to the company's mission,
to keep Rust staffed and resourced.
Really, I don't know all he dealt with in his management role,
but that's definitely the point &mdash;
he made it so the team could focus on just Rust.

Regardless,
Rust was always understaffed.
I remember being furious about it:
how could we compete with Googles and Apples with so few full-time engineers?
Half of the answer to this question was of course to nurture an invested and diverse community of contributors,
but that's a slow and uncertain process.
The other half of Rust's answer to this question was again thanks to Dave:
interns. So many interns.
Rust often had more interns than full time staff,
and they were hired by Dave,
who could easily recruit PL talent with his bona-fides in the academic world.
It was part of the Mozilla Research strategy.

A little appreciated fact: Rust was largely built by students,
and many of them interned at Mozilla.


## An anecdote about the design of Rust

I felt like I should include some personal anecdote about Dave's design contributions,
and the first one that came to mind was one where he disagreed with the team's decision.
So, not a great example of Dave's contributions, but still perhaps worth sharing,
and also an example of how he guided the team while also trusting their decisions.

It's hard to remember exact details so far back,
but I distinctly remember one single time Dave definitely disagreed with a decision the team made.
It was when we were introducing a distinction between mutable and immutable variable bindings.
We were just deciding what syntax to use for each, a simple thing.
At the end of the debate there were two obvious choices for immutable and mutable bindings, respectively:

- `let` and `let mut`
- `let` and `var`

The first is what we ended up with, the second borrows directly from JavaScript,
and there are good reasons for both.
The major question at hand was whether it was appropriate to make it "harder", or "uglier",
to pick the less preferable of two options.
Forcing the user to type two keywords to create a mutable binding is the language designers
quietly influencing programmers to think a little bit extra about introducing mutability.

I recall Dave did not agree with the decision the team made on this one.
In retrospect I assume he disagreed with the principle of punishing the user for their choice of coding patterns,
but I don't remember for certain.
I also still think the team was right about this one though:
not only does the extra `mut` annotation add a bit of extra work
to add mutability to a variable, but `mut` annotations naturally extended
to other areas of the Rust type system, like `&mut` references;
and the identification and management of mutability has become
a defining feature of the language.


## Setting the stage for success

Dave's direct involvement in the design of Rust ended in 2014 or 2015 I'd guess,
probably without most people in the Rust community even aware that he existed.

Dave didn't create Rust.
He only contributed a total of six commits to the public Rust repo.
He only spoke on the mailing list four times.

What he did was create a team that he believed could deliver a unique vision to the world,
and subtly planted in that team a set of values that
would enable Rust to thrive beyond the borders of Mozilla,
and beyond the involvement and personalities of any single individual.

And that is exactly what has happened.

Rust is a success for a lot of reasons,
from thousands of contributors,
thousands of tiny miracles coalescing into one coherent whole.

But tiny miracles don't become big miracles by accident.
