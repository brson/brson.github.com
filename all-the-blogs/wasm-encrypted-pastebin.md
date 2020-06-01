---
layout: post
title: "My first foray into Rust on WASM"
tags: [rust]
---

I am going to create an encrypted [pastebin] in Rust, using [WASM] on the client
side for the encryption, and [tokio] plus [PostgreSQL] on the server side. An
encrypted pastebin is a web application that stores text that has been encrypted
on the client side. Functionally, it will be like [cryptobin.co], which I use
occassionally to send information to people who don't otherwise know how to
receive encrypted messages.

I like this tool because it is conceptually simple, should be relatively easy to
build and deploy, and is an opportunity to demonstrate several important Rust
libraries.

I intend to take this project all the way to production. It will be the first
dynamic website I have put into production since perhaps the early 2000's, and I
will blog about my mistakes for your edumusement.


## The product

For now I'll call it "Simple Message Encrypter" (SME &mdash; "smee"). I'll think
harder about the name later.

SME is a website where you enter a message and a password, the message is
encrypted client-side, and the ciphertext transmitted to the server for storage.
The cipher text can later be accessed via a unique URL, and client-side
decrypted with the password.

Each message has an associated expiration time after which the server deletes
the message and it can no longer be accessed.

Optionally, the client can automatically generate a strong password.


## Use case

Mom sends me an email saying "I need your bank account number and routing number
to send you your weekly allowance."

This isn't info I want to send over email, but Mom doesn't use [Signal] and I
don't want to teach her how right now.

[Signal]: https://signal.org/

I open the SME website, enter the info she requested, set the message to expire
in 1 day, click "generate password", copy the password to my clipboard, and
click "encrypt message". SME stores the encrypted message, then redirects me to
the URL for the encrypted message.

I respond to my mom's email with "The info you want is here: [URL]. Enter
the password "[password]" to decrypt it. The message expires in 24 hours."


## Security considerations

Let's get this out of the way: this is not an app one should rely on generally.
Even if implemented perfectly it has several security problems. It is though
convenient when used appropriately.

TODO


## Design

As a general rule, I like to do as little upfront design as possible, but here's
a sketch of what this app should look like architecturally.

SME has four components

- the web server (sme-server) &mdash; besides handling the HTTP requests, its
  main duty is to store and load messages from a PostgreSQL database. Any number
  of these nodes can be run to scale the application.
- the web client &mdash; it encrypts messages and submits them to the server as
  POST-ed form data. When loaded from the URL of an existing message it decrypts
  from a user-provided password. The code for this lives inside `sme-server`.
- the message GC job (sme-gc) &mdash; this is a single node that handles message
  expiration as a batch job.

Some principles:

- _No REST_ &mdash; It's tempting to completely separate the client code from
  the server code, have the client be an HTML web application that only accesses
  the server through XMLHttpRequest. I think that would be a modern approach.
  I'm not going to do that though &mdash; I will access the server through
  traditional form submission; and when an encrypted message is loaded, I'll
  have the server pre-populate the returned HTML with the data. There's no
  server API.

  My main reason for doing this is for experiment's sake &mdash; to see whether
  the implementation feels simpler or more complex than one based on an API.


## Technology selection

SME is a Rust + WASM project.

- Rust servers &mdash; TODO
- WASM encryption &mdash; TODO

I intend to use the following Rust libraries:

- 



## Implementation plan

- Create sme-server
- Create client interface
- Implement form submission with in-memory storage
- Add wasm-based encryption
- Add PostgresQL backend


## Setup

I already know 


