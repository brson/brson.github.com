---
layout: post
title: "Rust in China"
tags: [rust]
---

TODO hook

So this is kind of a field report "from the ground" in the Chinese Rust
community. It also seems to be something of a missive on the nature of open
source community, and specifically corporate open source community, from a lost
exile.

TODO: missive?

- It's all about me
- The foundation of community
- Architecting community
- The pursuit of perfect inclusion {TODO title}
- Embracing the unexpected {TODO title}
- Big communities and their little babies
- The failure of the Rust team in China
- Rust and red carpets
- The first Chinese production user
- The good stuff in China
- Challenges for Rust in China
- The Rust job market
- Community building in China vs. the West
- Being a software engineer in China
- Embracing the other
- That God damned GFW
- We're all alone together
- The Rust toolchain doesn't fucking work in China!
- The Chinese companies who love Rust and what they are up to
- The Rust magick
- Thankses




## It's all about me

Rust in China is a different thing. It's almost unimaginable.

I can't imagine, but I've witnessed a little. And on the basis of that I have
written this massive, winding, deeply personal screed that you may or may not
read.

TODO: who am I?

This text contains my observations, interpretations and opinions alone (not
those of my employers, duh). There are few firm truths within &mdash; these are
just words that I wrote and am inviting you to read. I embrace
overgeneralizations, undergeneralizations {TODO word}, hyperbole, assumptions,
uncited and unprovable assertions and accusations, TODO gossip, and lies with
intent. There are feels within. Brace yourself, and I will do the same.

If you find yourself bouncing off one section in anger, consider picking and
choosing the parts that interest you most.




## The shape of a global community

Rust is a community. Rust's community is global. Rust's community originates in
the US. Rust has a strong Western, English-speaking community. Cities like San
Francisco, Berlin, London (TODO more diversity) are major hubs of Rust community
and economic activity.

Rust is many communities. Rust has non-English-speaking communities. Rust has a
strong presence in Central and South America. Rust has long had a community in
Japan. Korea. France. Italy. Et cetera.

_The [previous Rust website][prev] was written in 13 languages._ All but one of
those were entirely written by unpaid community members. There have been Rust
conferences in TODO cities. There are at least TODO active Rust meetups.

[prev]: https://prev.rust-lang.org/en-US/

These communities are all tied together to the global Rust community to greater
or lesser extents. The language that ties them together is English. This isn't
unexpected. The language of business is English. The language of tech is
English. Rust originates from English-speaking developers.

_TODO: of course Rust communities are not just defined by language and
geography, there are independent Rust communities of TODO._

As a long-time Rust community member, and an American, I have a Western-centric
view of Rust. And because the Rust project itself originates in the
English-speaking West, and is developed in English, I can have great visibility
into the project.

So there's a global Rust community, but it is anchored in the English-language,
Western-centric Rust community. Different sub-communities (not just based on
language) have more or less involvement in the greater community.

As a very early Rust advocate I have some familiarity with the evolution of
Rust's community. And I've had a fascination with how each independent Rust
community "cell" is born from nothing, and develops in its own direction,
whether it operates completely independently, whether they reach out to Rust
team members, what different kind of support and resources they need, how
different communities {TODO}




## The foundation of community

Fairly often people ask me how Mozilla created such a powerful community, with
independent communities all over the world TODO. I observe and encounter many
beautiful people with great intent, creating open source products, armed with
developer relations staff, marketing staff, tweeters, bloggers, whatever, all
trying to figure out the secret to creating community.

And they are doing it backwards. Community isn't built by fiat post-facto. It
isn't bought. Community grows from the ground, on its own, if that ground is
fertile.

{TODO: should the above be less negative?}

I'm not one for metaphore, but Rust's initial seed was
{TODO word}; Rust's ground was {TODO}.

Rust began with [one person], who came with right thinking and right actions,
whatever those were. He inspired a small group of people to join his project, and
his thinking influenced theirs. They inspired other small groups of people.
Those people made mistakes, and the Rust ethos adapted and matured. But always
with a shared, directed and intentional cultural-community vision.

Rust's community always was.

TODO: How close can I draw the religious comparison without actually saying it?




## Architecting community

From the very first moment Mozilla began developing Rust it was carefully
designed to become a community-driven project. Not an open source project &mdash
_a community-driven project_. There are many reasons for this, but the most
important is strategic. Successful programming languages are extremely expensive
to maintain, and they do not make money*. The most successful programming*
languages have a corporate sponsor with their own motivation for "owning" a
programming language. Mozilla's direct competitors are the biggest companies in
the world: Google, Apple, Microsoft. Yes, we saw Go being developed in parallel
and knew we could not compete. The only possible way for Rust to succeed was
through a grassroots upswell of contribution far bigger than Mozilla.

> _*with some exceptions of course. Not worth discussing here._

In my experience, this is not how the vast majority of corporate open source
projects are designed. Typically, their creators sense that there is some
advantage to being open source, work toward productization, then some distance
down the roadmap, they start figuring out that community stuff.

This often involves hiring somebody specifically for that purpose, maybe their
title is called something like "developer relations". Let the engineers deal
with the engineering, somebody else will figure out how to create a community.
This person is a firewall between the people who want to be part of the project,
and the people who are _actually_ part of the project.




## The pursuit of perfect inclusion

People want to be included. People feel bad when they percieve that they are not
included. In Rust we learned this the hard way. Repeatedly. To the greatest
extent reasonable, everything open, all the time, all the issues, all the
planning, all the meetings, everything communicated, every engineer active in
the community, despite how incredibly difficult it is. Company project engineers
and the project's developer community. There is no line between the two*.

> _*again, the fine points can be debated. That's not the purpose here._

That is a difficult path. I know this because my current employer [PingCAP] is
on that path. I know this because most every company I talk to as a Rust
consultant is on that path. Everybody wants to know what that Rust majick is.

The later a project, company, organization, starts to think hard about the
culture they want to build, the less likely they will ever get there. If you
want a community, employee #1 must believe it in their bones, be the community
they want to become, and demonstrate that ethos to every single person who comes
to them.




## Embracing the unexpected

Most of the greatest things that ever happened in Rust came from somewhere
unexpected. When that happened we embraced it with the greatest enthusiasm. Just
off the top of my I remember {TODO maybe rustup.rs?}

Compared to the scope of Rust's community success, Mozilla did little. The
[first meetup][frm] wasn't even organized by anybody from Mozilla, it was
organized by Erick Tryzelaar.

Others started doing the same without prompting, some of them Mozillians at
Mozilla offices, acting on their own, then icreasingly not. Less than a year
after the first Rust meetup in Mountain View 28 groups across the world
[celebrated Rust 1.0][lm].

[frm]: https://www.meetup.com/Rust-Bay-Area/events/143328072/
[lm]: https://users.rust-lang.org/t/a-list-of-rust-1-0-launch-meetups/1171

Mozilla co-sponsors some Rust conferences. It has never run any.

Today the Bay Area Rust meetup isn't even a Mozilla event. Sometimes it is held
at Mozilla and Mozilla buys the pizza, but just as often it is other companies
offering their space and their pizza to be part of this great thing. The main
reason these events happen is because some motivated member of the community
with access to corporate resources steps up, finds speakers, convinces their
manager to offer space and pizza, and makes it happen. It's thankless. It's not
their job. They love the Rust community and want to make it better.

Every Rust meetup around the world happens because an individual enthusiast with
their own independent motivation makes things happen.

What the Rust team did, and does, again, was unambigously encourage and support
the growth of communities around the world in whatever way they could. Usually
not with the direct help of any company, very often not even with Mozilla's
help.

We (again, the engineers, them people creating Rust, while we were creating
Rust), loved to hear about people using Rust, and loved to meet them. We would
come to them at any opportunity. And people who love Rust love to meet the
people behind the names they see all over the Rust project. An encouraging word
from [Niko] can easily spark a great, ever-branching, chain of success for Rust.
Rust team members spend so much time not coding, but just encouraging others to
do amazing things. It's impressive that they have the time to hack.

_So, ok, all that wasn't really what I set out to write about here_.




## Big communities and their little babies (TODO think about this)

Again, as a long time Rust community member I know a number of Rust community
organizers around the world, have some sense of how their community works and
relates to the global Rust community. But there are so many places I haven't
been, cultures I don't understand, organizers I haven't meant. It's humbling.

The European Rust communities are some of the strongest. Berlin in particular is
a hotspot of Rust activity (much thanks to the amazing work of [Florian
Gilcher]). But European cultures are well aligned with North American cultures,
and have many English speakers to help bridge the gaps. Many European
Rust hackers are prominent contributors and community members.

Many of the Latin-American communities are well integrated into the larger Rust
community. For example, early on TODO helped organize the community in Mexico
City while also acting as a sort liason to the greater Rust community. Many
Latin American hackers are prominent contributors and community members.

Many Indian hackers are prominent contributors and community members.

TODO fact check the above, examples
TODO give props

I don't really need to go on with more examples to get to the point. So many
more people that I don't even know deserve their props for independently
bringing the word of Rust to their peers.

Cheers to the dreamers. {TODO}

TODO: for the next section, need to re-emphasize that the greater
rust community embraces the lesser community.
TODO: establish the terms "greater Rust community" and "lesser Rust community".
TODO: find better word than "lesser"




## The failure of the Rust team in China

_The Chinese Rust community is an opaque mystery to the greater Rust community._

And that is tragic (I'm literally crying now).

And it is not just tragic for the human reasons. Those are important, and I have
great feels about that. But it is tragic because it is a major strategic failure
of the Rust team.

The Rust community in China is filled with amazing, talented people. There are
companies building their business on Rust in China.

Rust is not a success in China.

... todo

I didn't think about Rust in China until 2017. That's 7 years after I began
thinking about how to make Rust a success. That's 3 years after the team Rust
released the version of Rust it hoped people would adopt and companies would
build their products with.




## Rust and red carpets

Like I said, we loved to hear from users. Especially production users. If you've
been in the Rust community long enough, you probably remember the period around
1.0 where the Rust team was [supremely enthusiastic][fora] to hear from and help
production users.

[fora]: https://users.rust-lang.org/t/showcasing-rust-in-production-are-you-using-rust-in-production/5617

I think it's obvious why. This was a natural extension of our community-oriented
strategy, but we needed production adoption sooner than later in order to
establish legitimacy and create momentum. Once one big company adopts your
product, that's the go signal for the industry &mdash; this tech can be put into
production.

So, just as we did with contributors, we treated production users like royalty.
You think Rust is nifty, you little startup, welcome, let us please you. One at
a time, collecting production users treasures, and hanging them in [the trophy
case][for] as prominently as possible, like every tech startup (even non-profit
open source programming language startups).

[for]: https://prev.rust-lang.org/en-US/friends.html

_That was a pretty sardonic paragraph, but don't take it to mean I think
anything bad about the strategy. It's how startups work. Gotta get those
showcase users._

Rust has a lot of production users now, so it's impossible to [roll out the red
carpet] for all of them, though I'm sure the team tries their best.

I might (will) point out that it takes [one], [two] clicks of heavily
de-emphasized links to get to the master list of production users today, where
[previously the pathway to that list was very clear][p]. At least it still
exists. Do companies still want to be on that list today? What happens in 40
years when Rust has displaced C++ (yeah it's gonna happen non-believers). Will
there be a million logos on that page? There aren't so many loud calls to hear
from companies these days. Now that Rust has its showcase users and is
moderately successful, it's not a priority to keep building that trophy case
&mdash; there's no glamour in being the one-thoustand-and-first Rust user
(unless you're a Facebook type).

_Again, that might sound brutal, but it's just true. Your early production users
are your close partners. You are bound together in your success or failure._

Uh, again I seem to have fallen into a big digression (I am becoming
increasingly drunk as I rush to put this burst of inspiration to text). Anyway,
about Rust in China.

[one]: https://www.rust-lang.org/production
[two]: https://www.rust-lang.org/production/users
[p]: https://prev.rust-lang.org/en-US/




## The first Chinese production user

The first time I heard about a production Rust user in China (probably any
Chinese user) was {TODO date}, when TODO.

PingCAP creates a [NewSQL] database called [TiDB]. NewSQL is a buzzword that
means "SQL but horizontally scalable". (Strategically, PingCAP would love to
"own" the NewSQL buzzword &mdash; it's in your brain now. You can't forget. You
can't forget. `NewSQL == PingCAP`). That database is backed by a distributed
key/value database, [TiKV], and _that_ they decided to write in Rust.

_Hey, listen[!][hl] Now is a good time recall that I work at PingCAP at the
time of writing._

[hl]: https://www.youtube.com/watch?v=seKaU-qQuts

PingCAP's executive team are all engineers, and the CEO [Max] suspected Rust was
a good choice for TiKV, a system that needs to have infinity speed, infinity
uptime and negative infinity data corruption. CTO [Ed] confirmed that by writing
some encode/decode benchmark comparing Go to Rust. Rust won. Of course. Rust is
faster than Go. I'm confident saying that. Go is fine language though (I'm
required to say that as a matter of professional courtesy. No, really Go is just
fine, and sometimes better than fine. Awesome job on the compile times and the
feature-creep restraint. I'm in awe 🙇).

It was a big bet at the time. The "Friends of Rust" trophy case I mentioned
before was small. But the domain sounded real good for Rust. Anything that needs
to be crazy-fast and crazy-reliable… You want Rust.

To reiterate: _You want Rust_.

Yeah, I'm a bit drunk now. What was I saying?




## My history with the Chinese Rust community

PingCAP is headquartered in Beijing, and [Alex] and I happened to be travelling
to Beijing to give a keynote about Rust at QCon (It was called "The End of
Unsafety", and [this appears to be a poor-quality video of it][eou], but I will
never, ever watch it, and please don't send me feedback about my performance. I
cannot face the embarassment. Now I'm probably going to get trolled to death.
RIP me).

[eou]: https://www.youtube.com/watch?v=YAsaoYZDM1Q

As an aside (yeah, another), as soon as the two of us stepped onto the keynote
stage, _the entire room fucking emptied_. So that indicates a bit about the
challenges Rust faces in China (or maybe they just dislike Alex. Not me. I'm
pretty sure they 💖 me). One sweet lady I had chatted with beforehand, and who's
name I have sadly forgotten, stayed, took some pictures, and said nice things to
us. But the Middle Kingdom as a whole made it clear what they thought of Rust in
2017.

TODO: insert pictures

But in the meantime, the two of us agreed to make a detour and hang out at the
PingCAP office.

I actually don't remember anything about that meeting, but it was epochal in
some way. And somehow we reassured them they were making the right choice.

TODO more detail about who we met, what we did and why




## The good stuff in China

I have a tendency to amplify the negative, and forget the positive. I pretty
much wrote this whole thing (there's a lot yet to read below) without saying
anything positive about what's going on with Rust in China, then circled back
and started writing this sentence that you are reading now.

So what's the great stuff going on in China with Rust?

Today the Chinese Rust community is small and filled with enthusiasts. It's
small enough that you can know most everybody if you want to. In that way it's
kind of like the early Western Rust community... when it was more fun. I guess
that's an advantage of communities within communities: the greater community is
an overwhelming crush of humanity; the lesser {TODO lesser?} community is a
little family. Like real life. Rust is like real life. Maybe that's why it's so
hard.

Like every lesser Rust community, the Chinese Rust community created itself from
nothing, creating their own special version of what they saw happening
elsewhere. Again, like other Rust communities, I've seen motivated individuals
create spaces and opportunities for others to come be part of the Rust tribe.
Many of these people are young and brilliant. Some are sage and experienced.

I so wish I could credit them all, but I'm still basically an outsider and
am not familiar with the history of their community.

I can credit the people I know, for the things I know.

The Chinese Rust community organizes itself in the "Rust China Community"
WeChat group. TODO created it in TODO.

People like [TODO / Alex (Chaos)] patiently answer newbie questions. People
are friendly to each other.




## Challenges for Rust in China

I mean, it was mostly the right choice for TiKV. Rust isn't perfect.

I guess this is the point where we actually get to the point. Or points. I'll
probably make a few.

That one visit to give a keynote to an empty room and pay our respects to Max,
Ed, and crew is approximately the only outreach Mozilla or the Rust team has
granted to any company in China*.

> *once again, like, I'm generalizing. I'm sure you can provide a counterpoint,
  genius. I'm not interested in fighting. Just drinking and loving.

To reiterate: _the Rust team has made approximately no effort to encourage
adoption in the second largest market in the world._

This is the point in the narrative where I want to provide some economic and
demographic statistics about how huge China is, and how much bigger it is going
to get, because I want to set the stage for the argument about the importance of
Rust adoption in China. I'm not going to do that reserch though, because you
know China is huge. And I'm lazy. And drunk. And I want to finish this blog post
before I get bored, put it on the shelf with 1000 other drafts, and go pout on
my couch about how I never finish anything.

_There is little awareness of Rust in China_. Programmers are not thinking about
Rust. Companies are not considering writing software in Rust. Adoption momentum
is even lower than the West where adoption is slow and steady, there is a
growing job market, companies hiring for Rust.




## The Rust job market

Having made my career trying to make Rust a thing, I find the Rust job market
extremely interesting right now. Most hackers in tune with the programming
landscape in the West probbaly know that Rust has a strong niche, has a notably
positive community, and is [well-loved]. They may also know that you can't get a
job in Rust. At least, I read that with some frequency on the nets. But it's
not true at all.

_The market is __desperate__ for strong Rust talent._

The market is just small. For now.

Rust programmers can't be trained fast enough to fill demand. Most of my friends
from the early Rust days are extremely happily employed (and not just by
Mozilla). Many of the young students that grew up contributing Rust are
extremely happily employed. Professional Rust programmers are a happy bunch.

Rust experts make big bank right now. They are effectively impossible to hire.
Brilliant Rust novices get yanked out of the hiring pool instantly.

I can't provide evidence of this of course. If I made the effort I could, like,
graph the number of recruiter inquiries in my inbox over time. That would be
evidence of something. But I'm not going to do that because lazy.

I strongly suspect it goes up and to the right.

Sometimes people make the counterpoint that there aren't many public Rust job
listings. Like I said, it's a small market still, and a tight-knit community.
Spray-and-pray recruiting techniques don't work too well in the Rust job market.

{TODO: can i define spray-and-pray in my head?}

All this is to say that companies in the West are having a hard time hiring Rust
talent.

So, recalling what I said before about the dearth of Rust awareness in China,
you might be able to guess: _companies in China that love Rust are in agonizing
pain trying to hire Rust programmers_.

PingCAP, being the largest Rust employer in China (just like 15 {TODO} people
specifically writing Rust every day), is so desperate that one of the things
they are doing is creating their own pipeline of Rust-trained students,
collaborating with universities to promote and teach Rust. I myself am currently
creating one [training course][tc] of several, intended to teach the real-world
Rust skills applicable to the kind of distributed software PingCAP writes.

[tr]: https://github.com/pingcap/talent-plan/tree/master/rust

(That course is totally in an alpha state, and I have never written a training
course before, and it's a lot of work, and I have no idea if it's good yet, and
I'm rushing to get it done in between bouts of sheer despair. If you give it a
look and provide useful feedback that would be great. If you give it a look and
troll me hard I will probably be irreparably broken).




### Community building in China vs. the West

The Rust job market and the Rust community seem pretty directly related. In
the West, there has been a steadily growing community for nine years. The
number of people in that community is uncountable.

The number of people in the Chinese community is exactly 500.

Know how I know that? Chinese people communicate almost entirely on WeChat.
WeChat groups have a cap of 500 people.

Want to be part of the Rust China Community (that's the name of the Rust WeChat
group)? You can't.

I'm throwing a chair right now. It made a loud noise.

TODO: re the new Rust group.




## Being a software engineer in China

Remember when I was ranting so man thousands of words ago about Rust's
fundamental strategy of being community-driven? You know open source in general?
You know the much maligned silicon-valley startup culture? This is kinda new
stuff in China. China is dominated by massive companies with strict {TODO word},
ruthless old-school business methods, that treat their employees like cogs, and
drive them with the whip until they burn out and die.

The idea of starting from a position of building a healthy, equananimous, open
community, letting the "rising tide raise all ships" {TODO correct quote},
encouraging competition to create a diverse and healthy market (now I'm just
bullshitting &mdash; I'm a mediocre engineer, not a mediocre MBA){TODO elipsis}
that stuff doesn't really exist in China. It's a brutal business environment.

I know this probably sounds like the same soulless corporate treatment that you
and I and everybody experiences, but it's not. In all likelihood, your
counterpart in China is worked much harder than you, and treated with less basic
humanity.

TODO: 996, code farmers, etc.




## Embracing the other

Um, where was I?

Right, open source communities in China are not like open source communities in
the West. This was a tough thing for me to accept when I joined PingCAP.
Everywhere I looked I'm thinking, "we did this so much better in Rust. Can't we
just do the same here?"

PingCAP (I'm told) is quite a progressive company in China. I believe it (I'm an
expert &mdash; I've stepped into China thrice). They model their business after
western-type, open-source, silicon valley startups*. They are trying to do things
"right", to be good open source citizens, to treat their employees with dignity.

> _*oh, you probably have opinions about this. Share amongst yourselves. My point
  lies elsewhere._

They are pioneers in open source community building in China, and they are
struggling. Even in the West there's no roadmap for "doing open source right",
and China is a completely different environment.

One of the many reasons they hired me, and [Ana], and [Nick], is because we know
the magick that made the Rust community so amazing. We don't. We know what has
happened in the Rust community, what appeared to worked there. We know the
disasters that happened there. We have opinions.

_There is no magick recipe for open source success._

Just to continue with the WeChat thing, one of the first things all of us
westerners thought when we joined PingCAP was &mdash; your communication
platforms are not suitable for creating a welcoming community.

I'm not going to claim that Rust has their communication down perfectly forever,
but it's working pretty well, and it's pretty freaking simple. We are satisfied
to have one real-time "chat" platform, and one asynchronous "forum" platform. In
the modern world, we want these to be accessible to everybody on every platform,
to everybody of every skill level, without invitation, to accomodate an
arbitrary number of users, and to have moderation tools.

Rust today uses [Discord] for chat, and [Discourse] as a forum (yes, the extreme
similarity of the names is extremely unfortunate and a constant source of
confusion). I think most people in the Rust community are pretty happy with them
(though most everybody who is not a gamer laments certain aspect sof Discord).

In China, everybody again uses WeChat for everything. It's for interpersonal
communication, group communication, social media, corporate advertising,
payments (along with AliPay {TODO sp}). It's like their home. They live there.
They do everything there. They are used to it, they know it. Trying to do
anything else is a difficult argument to make.

Add to that that the Internet is utterly broken in China.




### That God damned GFW

Just writing GFW probably means nobody in China will ever see this web page.

That may or may not be hyperbole (ed: it totally is).

Do I even dare expand the acronym? The great firewall makes being a developer
in China a nightmare. It changes from day to day, from region to region. Nobody
knows what it's doing. As a result, developers in China habitually use VPNs to
get around it. It's ridiculous.

But it also means that you can't count on any given internet service or website
hosted outside of China to work at all. You and me might think, "let's use what
we know works". Let's just set up Discord and Discourse and be done with it.

We (me, Ana, Nick) haven't been able to make that argument at PingCAP.

I don't know offhand (I know I say this a lot), but I'd bet money that one of
Discord or Discourse doesn't work for some group of people in China right now.

weChat has the blessing of the GFW. So that's where the community meets.

TODO: The Rust Community 2




### We're all alone together

The China Rust community isn't part of the Rust community and I don't know what
to do about it.

I didn't even mention the language barrier yet. My experience is mostly with the
Chinese community, but there are probably similar problems in every
non-English-language Rust community.

In China, everybody is required to take English in school, which is great. Some
live lives that allow them to retain that knowledge, some don't. Most Chinese
developers need to be able to moderately understand written English. Some
Chinese developers can read English but not speak it. Many Chinese developers
are embarassed about their English skills (I'm not embarassed about my Chinese
skills, though most everything I say comes across as nonsense).

_I think most developers are scared of participating in the English-speaking
Rust community._ You know, the one where the vast bulk of Rust knowledge is.
And we're not doing anything at all to help them.

I already told you that China's platforms for community-building suck. It's hard
to initiate change. Generally. And we (the Western Rust community) are doing
nothing to help them organize better.

_There are no official non-English-language Rust communication channels_.

There's not even a discoverable way to find the non-official non-English
communication channels via the Rust website.

I don't understand how this happened. I mean, I do. But I don't like it.

There's definitely an argument that it's best to leave the communications
infrastructure to each individual foreign-language community, since we don't
understand their culture like they do. This may even be especially true in
China. And as I've stated many times, each individual community is self
organizing. But the greater Rust community supports the lesser communities. And
some of them need more.

In the case of the Chinese community, this all means that they are stuck in a
500 person group channel on WeChat, while the Rest of us ignore them. TODO nuance.

I'm getting tired. Who knows if I'm making coherent points now.




## The Rust toolchain doesn't fucking work in China!

Now we get to the most stupid, basic, ridiculous part of the Rust-in-China
debacle.

TODO

I am ashamed of Mozilla, myself included, for not solving this problem for the
Rust community.




## The Chinese companies who love Rust and what they are up to

It's props time. These are the companies in China betting on Rust. They deserve
the love of the Rust community. They are the torch-bearers of our mission to NXX
of the world's population.

I have relationships with people in some of these companies. Some I know little
about. There may be more that I don't know of, but probably not a great number.


**[PingCAP]**. They make the open source [NewSQL] database, [TiDB]. TiDB is a
MySQL-compatible database that scales horizontally to huge datasets. It is
cleverly designed to serve both [operational] workloads, and [analytical]
workloads, use-cases typically served by different databases. It is in
production with over TODO users in China, including many major companies like
the Bank of China. It is in production with 0 users in the West. TiDB is built
on a storage engine written in Rust, [TiKV]. As the first and largest Rust
production user in China they are widely reconized for their advocacy of Rust,
and they employ at least TODO Rust programmers, the most of any company in
China. [Ana Hobden], [Nick Cameron], and [Myself] work for PingCAP.


**[Cryptape]**.


**[Baidu]**


**[Bilibili]**


**[ByteBeat]**



## The Rust magick

I didn't set out to write some kind of dissertation on how the Rust community
built itself. And that's not what this is. This is a limited narrative about
open source community building based on my experiences. There's so much
more that made and continues to make Rust what it is.

I will say though that the Rust language itself is special. Its mission,
whatever one interprets it to be, inspires developers. And Rust has one unique
advantage that is inaccessable to many projects. Many _types_ of projects want
to attract contributors. TiDB for example, a database. Programmers though, love
programming languages, and Rust is a programming language, and Rust is written
in Rust. Rust's users are also Rust's developers.

But I already spilled the secret earlier:

_There is no magick recipe for open source success._




## Thankses




<!--

## Notes


- strategic
  - gdp
  - biggest companies

- image: Aaron, Me, Tennix
- caption: Aaron and Myself as Mozilla envoys to PingCAP, 2017. Those were happy times. These are also happy times. Some times are sad.

- image: Max and Me
- caption: Max, PingCAP; Me, Rust svengali

- image: RustCon Asia
- caption: RustCon Asia 2019. Not sponsored by Mozilla.

Hot gossip: despite discussing the conference name with the Rust core team well
in advance, they told us a week before the event not to use the name "RustCon"
because it was too close to "RustConf". We told them to fuck themselves. {TODO confirm}

Who deserves credit?

- Some PingCAP / Cryptape / ByteBeat devs
- kennytm, upstream involvement
- dcjanus

- rust.cc - created by Mike Tang, Alex helps
- Rust daily news - Alex
- cargo registry - DCjanus and Mike
- new rust website - aimee, mike
- rustcon asia - 

- Alex - rust book, rust training, community organizing
- Mike Tang
- Companies
  - PingCAP
  - Cryptape
  - ByteBeat
  - Baidu
  - Bilibili
- rust.cc - by Mike Tang
- Rust China Community
- Rust Daily News - Alex
- The Dao of Rust - Alex
- DCjanus makes the cargo registry
- https://github.com/rustcc/lernaean-deploy/
- https://github.com/rustcc/lernaean

- other users
  - https://github.com/KiChjang
  - https://github.com/crlf0710
  - https://github.com/losfair is good
- https://www.reddit.com/r/rust/comments/ax86y1/introducing_the_book_the_tao_of_rust/
- https://www.amazon.cn/gp/product/B07NW95M76
- https://www.amazon.cn/dp/B07GSRFG28/ - first rust book F001
- http://github.com/huangjj27     This guy is also more active for wasm
https://github.com/aaranxu and https://github.com/OlingCat  

The two guys organized the translation of the official rust book.
https://github.com/rust-lang-cn

TODO: "greater" rust community or "global" rust community
- TODO: greater vs lesser
TODO: figure out how to integrate rusty-dash story https://internals.rust-lang.org/t/the-rust-project-needs-much-better-visibility-into-important-metrics/3367
TODO: give links to places where other-language organization takes place
  - talk about lack of official support
- pingcap is the flag bearer of rust in china

- fact checkers
  - keith young
  - alex
  - breesewish
- editors
  - aimee
  - clevelnd (wtf)

- 码农 ("code" "farmers")
- 996

- thank Keith and others for helping

Here's another perspective from a China topic writer in Quora: https://www.quora.com/What-does-Silicon-Valley-fundamentally-misunderstand-about-Chinese-tech-companies/answer/Paul-Denlinger

- TODO: move footnotes to footnotes. Make them thematic.

- TODO: architecting community -> adapting
- TODO: is there an opportunity to mention compile times?
- TODO: magick vs secret

- TOdo: get output of rustup and cargo behind the GFW
- TODO: need to say more about my background with PingCAP and china

- Rust channel #2: Rust 语言学习交流1群
- There are several QQ groups
- TODO: note that it's not up to me to decide how the China community organizes itself
- Rust Community Channel first organized by Liigo
  - He wrote an article in 2014 「 why did I give up the go language」
  - 庄晓立(Liigo) / XiaoLi  Zhuang
  - He contributed to Rust. ?Sized maybe
  - https://blog.csdn.net/liigo/article/details/86535831https://blog.csdn.net/liigo/article/details/86535831

- The number of people in the community is about 5,000.
- But the active people are estimated to be less than a thousand.
- Rust daily news subscribers are about a thousand people
- Alex:
rust.cc forum   pv 2000～4000  uv 600～800 

Rust.cc is implemented by mike's web framework sapper
- The member number of qq groups is about 5,000.
- github.com/rustforce/sapper
- Telegram

- TODO: note about incomplete openness of Rust
- TODO: background on what Rust is
- discord is unstable through GFW per Mike Tang
- TODO: add anecdote about rust chinese flag

- take pictures of chinese rust books
- no thesis
- "insinuation"
- add anecdote about early rust
- note about western audience and complex language


-->
