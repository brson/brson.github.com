---
layout: post
title: Writing an LLVM backend for the Move language in Rust
tags: [rust, move, solana, blockchain]
---

Since December I and others have been working on a port of
[the Move programming language](https://github.com/move-language/move)
to LLVM.
This work is sponsored by [Solana](https://github.com/solana-labs/),
with the goal of running Move on Solana and its [`rbpf` VM](https://github.com/solana-labs/rbpf);
though ultimately the work should be portable to other targets
supported by LLVM.

This work is firmly in the prototyping phase,
but I've learned a lot
and want share what I know so far about writing LLVM compiler backends.

At this point we have created the scoffolding for the LLVM backend,
and a test harness that compiles Move bytecode,
links multiple Move modules with a "native" Move runtime,
and runs them inside an `rbpf` VM test harness.

This blog post will describe Move,
and some of the features of the Move compiler that make it amenable to compiling to LLVM,
what I learned about writing a simple LLVM backend,
and a description of the native Move runtime.

While this post is about the Move compiler specifically,
it also has tidbits relevant to starting an LLVM-based compiler backend generally.

Note that I am not a compiler expert.
This is a learning project for me,
and some of the things I have done here will turn out to be mistakes.

The code for the Move LLVM backend is not ready for anybody to do anything with yet,
but it lives here:

- <https://github.com/solana-labs/move/tree/llvm-sys/language/tools/move-mv-llvm-compiler>
- <https://github.com/solana-labs/move/tree/llvm-sys/language/move-native>

The first link is the Move bytecode to LLVM compiler,
and the second is the native runtime.


## Table of contents

- [About Move](#about-move)
- [Writing an LLVM backend with Rust](#writing-an-llvm-backend-with-rust)
- [The basics of an LLVM backend](#the-basics-of-an-llvm-backend)
- [Challenges of porting Move](#challenges-of-porting-move)
- [The Move Model and Move stackless bytecode](#the-move-model-and-move-stackless-bytecode)
- [An outline of lowering Move to LLVM](#an-outline-of-lowering-move-to-llvm)
- [Isolating the unsafe LLVM C API](#isolating-the-unsafe-llvm-c-api)
- [Dealing with missing LLVM APIs](#dealing-with-missing-llvm-apis)
- [A "native" Move runtime](#a-native-move-runtime)
- [Testing and how everything works together](#testing-and-how-everything-works-together)
- [What's next](#whats-next)


## About Move

Move is a strongly-statically-typed programming language originally developed at Facebook for
writing programs on the now-defunct Libra/Diem blockchain platform.
Although Libra is no more,
the Move language (as well as other Libra tech) is being used
by several projects, most notably
[Aptos](https://github.com/aptos-labs/aptos-core)
and
[Sui](https://github.com/MystenLabs/sui),
both started by former Libra devs.

The tech that came out of Libra is
exceptional in both its design and implementation quality.
In addition to Move,
the
[Narwhal / Tusk](https://arxiv.org/abs/2105.11827)
and [Bullshark](https://arxiv.org/abs/2201.05677)
consensus algorithms
that derive from Libra are some of the most elegant and performant (in theory at least) in the industry.

Move is written in Rust and is heavily influenced by Rust:
it is based on ownership types,
and is strongly focused on simplicity, correctness, and analyzability.
As part of this focus,
Move notably _does not have dynamic dispatch_.
This makes Move exceptionally analyzable,
but imposes extreme limitations on what can be expressed between any two programs.
It likely will force developers to discover and adopt coding patterns unique to Move.

An example of Move from [the Move book](https://move-language.github.io/move/):

```move
module 0x42::test {
    struct Example has copy, drop { i: u64 }

    use std::debug;
    friend 0x42::another_test;

    const ONE: u64 = 1;

    public fun print(x: u64) {
        let sum = x + ONE;
        let example = Example { i: sum };
        debug::print(&sum)
    }
}
```

Move source is compiled to Move bytecode,
which runs in the Move VM.
Bytecode validation is critical to the correctness and security
of interacting Move programs,
though I do not yet know in detail how Move bytecode is validated.

Also, move has a built-in theorem prover leveraging [z3](https://github.com/Z3Prover/z3).
You can write proofs inline with Move modules:

```move
public fun char(byte: u8): Char {
    assert!(is_valid_char(byte), EINVALID_ASCII_CHARACTER);
    Char { byte }
}

spec char {
    aborts_if !is_valid_char(byte) with EINVALID_ASCII_CHARACTER;
}
```

Outside of toys and test cases I have not written much Move myself.

Resources about Move:

- The paper [Resources: A Safe Language Abstraction for Money](https://arxiv.org/pdf/2004.05106.pdf)
- [The Official Move Book](https://move-language.github.io/move/)
- [The Unofficial Move Book](https://move-book.com/)

Some useful technical blog posts about Move:

- [Smart contract development: Move vs. Rust](https://medium.com/@kklas/smart-contract-development-move-vs-rust-4d8f84754a8f)
- [Securing Move](https://medium.com/aptoslabs/securing-move-f81099f5e08c)
- My own [First impressions of the Move programming language](https://brson.github.io/2022/09/21/move-impressions)


## Writing an LLVM backend with Rust

Depending on perspective,
when you using LLVM to emit machine code for your language you are either:
writing a _backend_ to your language's compiler,
or writing a _frontend_ to LLVM.
I tend to think of the LLVM component being the backend of the compiler,
as that's how the Rust compiler calls it,
though when reading LLVM docs you might see compilers like Clang called LLVM frontends.

In my experience, writing one's first LLVM backend is daunting:
LLVM is huge, with a sprawling number of instructions, and additional metadata that can be associated with those instructions;
[the docs](https://llvm.org/docs/)
are inadequate for newcomers,
with especially the [API documentation](https://llvm.org/doxygen/)
being stunningly difficult to navigate.
The [LLVM language reference](https://llvm.org/doxygen/)
is the most useful LLVM documentation.

Non-C++ languages access LLVM primarily through the official [LLVM-C](https://llvm.org/doxygen/group__LLVMC.html)
bindings,
though it is important to understand that not all features of LLVM are exposed
through the C bindings,
and they are basically undocumented:
one will often need to read the source through to the underlying C++ APIs to figure out what the C APIs do.
In my experience compilers not written in C++ that uses LLVM as a backend end up writing some C++ glue code to expose
extra C APIs for their own use.

In the Rust world there are a few options for interfacing to LLVM:

- [llvm-sys](https://docs.rs/llvm-sys/latest/llvm_sys/) - raw unsafe bindings to the LLVM C API
- [inkwell](https://thedan64.github.io/inkwell/inkwell/index.html) - idiomatic Rust bindings to `llvm-sys`
- [llvm-ir](https://docs.rs/llvm-ir/latest/llvm_ir/) - another high-level Rust binding

As a Rust programmer I have found the easiest way to discover and understand
APIs is:

- read the LLVM language reference to understand what LLVM features I need
- search the llvm-sys API docs and source to find the LLVM-C functions I need
- search the inkwell API docs and source to understand how to use the LLVM-C functions

Inwkwell uses the Rust type system to make explicit
the ownership relationship of objects in the LLVM API.
The Rust bindings to LLVM are much easier to navigate and search than the official C++ API docs.

The Rust compiler itself has its own crate of LLVM bindings,
and while one _could_ use it directly,
I wouldn't recommend it,
as it depends on Rust's own LLVM fork,
and is filled with Rust-specific glue code.
Most projects will not need all the esoteric LLVM patches that `rustc` carries around.

The [`rustc` LLVM bindings](https://github.com/rust-lang/rust/tree/master/compiler/rustc_llvm/llvm-wrapper)
though are a great reference for
how to access LLVM features that are not exposed through the LLVM C APIs.

The Move LLVM backend uses the `llvm-sys` crate to access LLVM,
wrapping the unsafe calls in its own [lightweight wrapper types](https://github.com/solana-labs/move/tree/0746853e012b2c98eb7feb28f4b6df739212a6d7/language/tools/move-mv-llvm-compiler/src/stackless/llvm.rs)
that provide some amount of memory safety as well as higher-level semantic
operations appropriate to the Move language.


## The basics of an LLVM backend

While LLVM is intimidating in its breadth and complexity,
one can get started with only a few simple principles.

First, the basics:

- The unit of compilation in LLVM is a _module_.
  This might correspond to e.g. a C source file,
  or a Rust crate.
- Modules contain local function definitions,
  and declarations of functions that exist elsewhere
  (to be linked later).
- LLVM provides typical integer and float (scalar) types.
- LLVM provides arrays of types.
- LLVM has structs, anonymous or named, and for our purposes we will give them
  all a name within the module.
- Pointers are untyped (like `void*`): their type must be
  named whenever they are used.

So the basic task of getting a compiler started
is deciding where the module boundaries are,
declaring all the local and external functions,
declaring all the struct types,
and then defining the functions.

The LLVM model consists mainly of pointers to memory and registers.
Some of those pointers are to stack variables that LLVM calls
[_allocas_](https://llvm.org/docs/LangRef.html#alloca-instruction).

LLVM functions are composed of a list of _basic blocks_
which contain instructions. Basic blocks are the targets
of branching instructions.

In reading about compilers and code generation,
one will quickly come across the concept of _single static assignment_
(SSA) form. This is a common model wherein the compiler
operates on virtual _registers_ that are assigned exactly once.
Compilers love this form because it is easy to analyze.

You don't need to care about SSA, or the confusing notion of
[phi nodes](https://llvm.org/docs/LangRef.html#phi-instruction)
that link registers across blocks.

Instead, in defining a function your job is to

- declare allocas for all the local variables,
- then for every operation in your language:
- load those variables into a register, perform an operation on it,
  and store it back to an alloca.

In a real machine this would be inefficient,
loading from memory to registers and back for every operation.
LLVM though has an optimization pass called
[mem2reg](https://llvm.org/docs/Passes.html#mem2reg-promote-memory-to-register)
that transforms allocas into SSA registers.

You will also need to terminate every block with a branch to another block or a function return.

That's enough to get started,
and that's about as much as we've accomplished with the Move LLVM backend.
I'm sure I'll learn about all the mistakes I've made as we progress further.




## Challenges of porting Move

There is reason to be skeptical of porting Move to platforms other than the Move VM:
the Move language and the Move VM are tightly integrated,
and Move bytecode must be validated before running it to be correct.
This suggests that any platftorm not interpreting bytecode will have some big challenges maintaining Move's correctness guarantees.

I don't actually know what all those guarantees are,
nor what validations are performed on the bytecode,
but I have a vague guess of why bytecode validation is so important:

Any two blockchain programs cannot trust each other with the other's data,
otherwise one program would simply steal from the other.
On most blockchains there is a strong process boundary between any two programs
to enforce they do not interfere directly with each other's operation.
I suspect that Move relies on its strong type system to let mutually-distrusting
programs operate within the same address/data space;
as long as the programs follow the ownershp-based type system,
they cannot access any non-owned data;
and there are rules that the bytecode must follow in order
for the type system to be sound.

Assuming the above is vaguely in the ballpark,
I only have three ideas for how to maintain safety in a non-Move VM:

1. Run every Move module in its own process and emit code at the boundaries that dynamically
   verifies certain invariants; while also marshalling arbitrary Move values, including reference types, across those boundaries.
   This strategy seems extremely speculative to me.
   If this is possible it will be only with exceptional effort.
2. Have a trusted party compile the Move bytecode to LLVM. Just as the Move VM
   validates the bytecode, this trusted party does as well. This trusted party could be, e.g.
   the Solana validators themselves or a secondary consensus network dedicated
   just to generating native-compiled Move code that can be trusted.
   This is the most-obviously workable solution.
3. Run the translated programs off-chain, through a zero-knowledge-proof execution environment,
   like [RISC-0](https://github.com/risc0/risc0), to generate proofs of correct execution,
   and verify the proofs with an on-chain program. This would still require some method
   to guarantee that the individual RISC-0 modules a complete program is linked from
   correspond to particular Move bytecode; but that _seems_ to me an eaiser problem
   than verifying that arbitrary machine code corresponds to Move bytecode. This is also extremely speculative,
   as RISC-0 is very new, and I don't have any experience.

I note that Move already includes [an EVM backend](https://github.com/move-language/move/tree/041592e9486573d7458d808ff53b656622bb3951/language/evm)
that targets [YUL](https://docs.soliditylang.org/en/v0.8.19/yul.html),
Solidity's intermediate language. I have not read that code yet,
but it presumably contains opinions about this problem.

From my perspective, this is still a distant problem,
though admittedly perhaps insurmountable.
At present we are still just getting basic compiler-stuff working.

Another technical challenge is that the Move VM is a [stack machine](https://en.wikipedia.org/wiki/Stack_machine),
and LLVM is a register machine,
so how to translate one to the other is not immediately obvious.



## The Move Model and Move stackless bytecode

Move already has a solution for register-based backends:
[Move stackless bytecode](https://github.com/move-language/move/blob/041592e9486573d7458d808ff53b656622bb3951/language/move-prover/bytecode/src/stackless_bytecode.rs).

Stackless bytecode is a transformation of the Move bytecode
to a register machine model. It is used by the Move theorem
prover, as well as the port of Move to YUL.

The Move stackless bytecode is refreshingly simple,
a great foundation for doing codegen &mdash;
as soon as I looked at it it was obvious how to lower it to LLVM.

The bytecode itself is represented as an enum:

```rust
pub enum Bytecode {
    Assign(AttrId, TempIndex, TempIndex, AssignKind),

    Call(
        AttrId,
        Vec<TempIndex>,
        Operation,
        Vec<TempIndex>,
        Option<AbortAction>,
    ),
    Ret(AttrId, Vec<TempIndex>),

    Load(AttrId, TempIndex, Constant),
    Branch(AttrId, Label, Label, TempIndex),
    Jump(AttrId, Label),
    Label(AttrId, Label),
    Abort(AttrId, TempIndex),
    Nop(AttrId),

    SaveMem(AttrId, MemoryLabel, QualifiedInstId<StructId>),
    SaveSpecVar(AttrId, MemoryLabel, QualifiedInstId<SpecVarId>),
    Prop(AttrId, PropKind, Exp),
}
```

These `TempIndex` types are indexes into a table of local variables,
and all of the stackless bytecodes operate over them.
Depending on your perspective one might think of these `TempIndex`es as either
registers or stack slots.
The `Label` types represent branch targets; a compiler
might think of them as the start of basic blocks.

That `Bytecode::Call` variant does most of the work,
representing most Move operations as function-like calls
via the `Operation` enum:

```rust
pub enum Operation {
    // User function
    Function(ModuleId, FunId, Vec<Type>),

    // Markers for beginning and end of transformed
    // opaque function calls (the function call is replaced
    // by assumes/asserts/gotos, but it is necessary to
    // add more assumes/asserts later in the pipeline.
    OpaqueCallBegin(ModuleId, FunId, Vec<Type>),
    OpaqueCallEnd(ModuleId, FunId, Vec<Type>),

    // Pack/Unpack
    Pack(ModuleId, StructId, Vec<Type>),
    Unpack(ModuleId, StructId, Vec<Type>),

    // Resources
    MoveTo(ModuleId, StructId, Vec<Type>),
    MoveFrom(ModuleId, StructId, Vec<Type>),
    Exists(ModuleId, StructId, Vec<Type>),

    // Borrow
    BorrowLoc,
    BorrowField(ModuleId, StructId, Vec<Type>, usize),
    BorrowGlobal(ModuleId, StructId, Vec<Type>),

    // Get
    GetField(ModuleId, StructId, Vec<Type>, usize),
    GetGlobal(ModuleId, StructId, Vec<Type>),

    // Builtins
    Uninit,
    Destroy,
    ReadRef,
    WriteRef,
    FreezeRef,
    Havoc(HavocKind),
    Stop,

    // Memory model
    IsParent(BorrowNode, BorrowEdge),
    WriteBack(BorrowNode, BorrowEdge),
    UnpackRef,
    PackRef,
    UnpackRefDeep,
    PackRefDeep,

    // Unary
    CastU8,
    CastU16,
    CastU32,
    CastU64,
    CastU128,
    Not,

    // Binary
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitOr,
    BitAnd,
    Xor,
    Shl,
    Shr,
    Lt,
    Gt,
    Le,
    Ge,
    Or,
    And,
    Eq,
    Neq,
    CastU256,

    // Debugging
    TraceLocal(TempIndex),
    TraceReturn(usize),
    TraceAbort,
    TraceExp(TraceKind, NodeId),
    TraceGlobalMem(QualifiedInstId<StructId>),

    // Event
    EmitEvent,
    EventStoreDiverge,
}
```

Move also provides the "Move model",
a whole-program analysis of a Move program and its dependencies
that provides most or all of the meta-information a compiler
needs to lower stackless bytecode to machine code.
The Move model can answer questions like "what functions does this function call?",
and many more.

Crucially, while the Move model can be created from Move source code,
it can _also be constructed from nothing but Move bytecode_
(plus some extra knowledge about how dependencies are ordered).
This provides hope that platforms not running the Move VM
can still translate bytecode via the Move model and not
ever need to see users' source code.

Links:

- [The Move model](https://github.com/move-language/move/tree/main/language/move-model)
- [Move stackless bytecode](https://github.com/move-language/move/blob/main/language/move-prover/bytecode/src/stackless_bytecode.rs)
- [This Github issue](https://github.com/move-language/move/issues/817)
contains resources for hacking on Move stackless bytecode.


## An outline of lowering Move to LLVM

The Move model provides a `GlobalEnv` that understands
the entire world of a program, and which provides access
to `ModuleEnv`s, which provide access to `FunctionEnv`s.

These pleasantly map to LLVM's `Context`, `Module`,
and `Function`.

Our own translation will have context objects that
mirror this heirarchy and contain both the Move and LLVM contexts.

Here's how our context types look in the compiler today:

```rust
pub struct GlobalContext<'up> {
    env: &'up mm::GlobalEnv,
    llvm_cx: llvm::Context,
    target: Target,
}

pub struct ModuleContext<'mm, 'up> {
    env: mm::ModuleEnv<'mm>,
    llvm_cx: &'up llvm::Context,
    llvm_module: llvm::Module,
    llvm_builder: llvm::Builder,
    /// A map of move function id's to llvm function ids
    ///
    /// All non-generic functions that might be called are declared prior to function translation.
    /// This includes local functions and dependencies.
    fn_decls: BTreeMap<mm::QualifiedId<mm::FunId>, llvm::Function>,
}

struct FunctionContext<'mm, 'up> {
    env: mm::FunctionEnv<'mm>,
    llvm_cx: &'up llvm::Context,
    llvm_module: &'up llvm::Module,
    llvm_builder: &'up llvm::Builder,
    /// A function to get llvm types from move types.
    ///
    /// The implementation lives on ModuleContext, and this
    /// ugly declaration exists to avoid passing the entire module
    /// context to the function context. It may end up not worth
    /// the effort.
    llvm_type: Box<dyn (Fn(&mty::Type) -> llvm::Type) + 'up>,
    fn_decls: &'up BTreeMap<mm::QualifiedId<mm::FunId>, llvm::Function>,
    label_blocks: BTreeMap<sbc::Label, llvm::BasicBlock>,
    /// Corresponds to FunctionData:local_types
    locals: Vec<Local>,
}
```

I am a bit embarrassed about this sloppy code,
but it's illustrative of how compilers organize
their code generation,
and this compiler is still simple enough to
take in the whole thing at once.
Another year of development and it'll surely be much more difficult to understand.
Take particular notice of all the caches for cross referencing between the Move and LLVM worlds.

Keeping in mind that the Move model has done a _ton_ of the
upfront analysis work needed for the job,
our basic algorithm for translating a whole Move program is:

- For each module:
  - Declare every function that it might call,
    both local functions, foreign Move functions,
    and foreign native functions. It's easiest if
    one can declare everything up front because it
    allows for compiler data structures to be immutable,
    though it is also possible to declare functions
    "just-in-time" as their call-sites are encountered.
  - Declare every struct the module will reference.
  - Translate the functions.

To translate the functions,
recall that the Move stackless bytecode operates
on `TempIndex`es, and branches to `Labels`:
with basic knowledge of LLVM this suggests
the obvious translation strategy:

A `TempIndex` will correspond directly to an
[LLVM alloca](https://llvm.org/docs/LangRef.html#alloca-instruction),
which we can think of as a pointer to a stack slot;
and most operations will involve loading allocas into SSA registers,
maniplating them, and storing SSA registers to allocas.
The LLVM [mem2reg](https://llvm.org/docs/Passes.html#mem2reg-promote-memory-to-register)
pass will then optimize the heck out of all those loads and stores.

Whenever we see a `Label` instruction we will open
a new LLVM [basic block](https://en.wikipedia.org/wiki/Basic_block),
and they are the target of branch instructions.


## Isolating the unsafe LLVM C API

The LLVM C API has a big surface area, and in Rust it's all unsafe.
We don't want to be intermixing FFI code with our bytecode translation,
so we create a module just for encapsulating LLVM calls
in "safe" Rust code.
I put "safe" in scare-quotes because LLVM is a vast un-memory-safe system,
and making it mostly memory-safe is very hard,
and making it completely memory-safe is probably not possible.

For our purposes we'll just accept that we can't be perfectly memory-safe,
and pretend our wrapper APIs are truly memory-safe.
It's a convenient trade-off.
For those that want stronger guarantees,
the Inkwell crate has done the hard work of making LLVM as memory safe is reasonable.

Our compiler takes an approach of building
LLVM abstractions that bundle multiple LLVM operations
into single high-level load-transform-store operations
with the appropriate semantics for Move stackless bytecode.

Here's an illustrative example of translating `Operation::Eq`:

```rust
Operation::Eq => {
    assert_eq!(dst.len(), 1);
    assert_eq!(src.len(), 2);
    // These are the Move TempIndexes of all the operands,
    // which allso correspond to the indexes of the LLVM allocas.
    let dst_idx = dst[0];
    let src0_idx = src[0];
    let src1_idx = src[1];
    // Get the LLVM values out of our cache of allocas.
    let dst_llval = self.locals[dst_idx].llval;
    let src0_llval = self.locals[src0_idx].llval;
    let src1_llval = self.locals[src1_idx].llval;
    // Get the Move and LLVM types out of the cache.
    let mty = &self.locals[src0_idx].mty;
    let llty = self.locals[src0_idx].llty;
    match mty {
        mty::Type::Primitive(mty::PrimitiveType::U8) => {
            self.llvm_builder.load_icmp_store(
                llty,
                src0_llval,
                src1_llval,
                dst_llval,
                llvm::LLVMIntPredicate::LLVMIntEQ,
            );
        }
        _ => todo!(),
    }
}
```

The `src0_llval`, `src1_llval`, and `dst_llval`,
are all `LLVMValue`s that represent allocas.
We call the `load_icmp_store` method which performs
multiple LLVM operations. It looks like

```rust
pub fn load_icmp_store(
    &self,
    ty: Type,
    src0: Alloca,
    src1: Alloca,
    dst: Alloca,
    pred: LLVMIntPredicate,
) {
    unsafe {
        let src0_reg = LLVMBuildLoad2(self.0, ty.0, src0.0, "icmp_src_0".cstr());
        let src1_reg = LLVMBuildLoad2(self.0, ty.0, src1.0, "icmp_src_1".cstr());
        let dst_reg = LLVMBuildICmp(self.0, pred, src0_reg, src1_reg, "icmp_dst".cstr());
        LLVMBuildStore(self.0, dst_reg, dst.0);
    }
}
```

Most of our LLVM methods will look like this:
load everything into registers, perform register ops,
store back to allocas.


## Dealing with missing LLVM APIs

While the LLVM C APIs provide _most_ of the features
one needs to write a compiler backend,
in my experience compilers (that aren't written in C++) usually end
up with their own little library of custom glue code
to expose needed features from the C++ API.

See for example Rust's [big collection of LLVM C++ glue](https://github.com/rust-lang/rust/tree/master/compiler/rustc_llvm/llvm-wrapper).

Already for this project I found we needed our own glue
to get access to the `NoReturn` function attribute
(maybe this is exposed in the C API but I couldn't find it).

I pillaged the glue for this directly from Rust
(both projects share the same license,
and I have indicated the copying in the code).


The pattern that I used here was to create
[a crate to hold the extra bindings](https://github.com/solana-labs/move/tree/0746853e012b2c98eb7feb28f4b6df739212a6d7/language/tools/move-mv-llvm-compiler/llvm-extra-sys).
I called this one `llvm-extra-sys`.

This crate contains a [build script](https://github.com/solana-labs/move/blob/0746853e012b2c98eb7feb28f4b6df739212a6d7/language/tools/move-mv-llvm-compiler/llvm-extra-sys/build.rs),
that:

- gets the path to the `llvm-config` tool from the `llvm-sys` crate
- calls `llvm-config` to get various compiler flags
- uses the [`cc`] crate to build our C++ glue, and
  instruct cargo how to link it

[`cc`]: https://docs.rs/cc/latest/cc/

The ability for one crate, in this case `llvm-sys`,
to pass metadata to its direct dependent crates
through environment variables,
via cargo,
is something I've only learned of recently.

I still don't quite understand how it works,
but here's how the build script of `llvm-extra-sys`
retreives the path to `llvm-config`
by reading environment variables from the build script
of `llvm-sys`:

```rust
let llvm_config_path = std::env::var("DEP_LLVM_15_CONFIG_PATH")
    .context("DEP_LLVM_15_CONFIG_PATH not set")
    .context("this probably means the llvm-sys build failed")?;
```

That `DEP_LLVM_15_CONFIG_PATH` env var is set by cargo,
at the direction of the `llvm-sys` build script.

To complete the glue,
`llvm-extra-sys`'s `lib.rs` file contains Rust bindings to the C-ABI
exports from the C++ glue code.


## A "native" Move runtime

When compiled to LLVM, Move code doesn't have access to services
provided by the Move VM.
The most obvious of these services I've run into so far
are "native" function calls,
calls from Move into functions not defined in Move but provided by the runtime.
There are a handful of these in the Move standard library,
e.g. for operating on vectors or calculating hashes.

Here's a fragment of the `vector` module in Move `std`:

```move
module std::vector {
    /// Add element `e` to the end of the vector `v`.
    native public fun push_back<Element>(v: &mut vector<Element>, e: Element);

    /// Destroy the vector `v`.
    /// Aborts if `v` is not empty.
    native public fun destroy_empty<Element>(v: vector<Element>);

    /// Pushes all of the elements of the `other` vector into the `lhs` vector.
    public fun append<Element>(lhs: &mut vector<Element>, other: vector<Element>) {
        reverse(&mut other);
        while (!is_empty(&other)) push_back(lhs, pop_back(&mut other));
        destroy_empty(other);
    }
}
```

I have implemented many of the native function calls
needed by `std` in the [`move-native`](https://github.com/solana-labs/move/tree/llvm-sys/language/move-native) crate,
the almost none of it is yet supported by the LLVM backend's codegen.

The layout of e.g. the Move `vector<T>` type needs to be known to both
the compiler and the runtime, and the runtime defines it as

```rust
#[repr(C)]
#[derive(Debug)]
pub struct MoveUntypedVector {
    pub ptr: *mut u8,  // Safety: must be correctly aligned per type
    pub capacity: u64, // in typed elements, not u8
    pub length: u64,   // in typed elements, not u8
}
```

This is similar to Rust's own `Vec` type,
and for the most part, the runtime calls implement Move `vector`
operations by converting the Move `vector` to a Rust `Vec`,
calling methods on Rust `Vec`,
then converting back to the Move `vector`.

The `push_back` function for example looks like:

```rust
#[export_name = "move_native_vector_push_back"]
pub unsafe extern "C" fn push_back(
    type_ve: &MoveType,
    v: &mut MoveUntypedVector,
    e: *mut AnyValue
) {
    let mut rust_vec = borrow_typed_move_vec_as_rust_vec_mut(type_ve, v);

    match rust_vec {
        TypedMoveBorrowedRustVecMut::Bool(mut v) => v.push(ptr::read(e as *const bool)),
        TypedMoveBorrowedRustVecMut::U8(mut v) => v.push(ptr::read(e as *const u8)),
        TypedMoveBorrowedRustVecMut::U64(mut v) => v.push(ptr::read(e as *const u64)),
        TypedMoveBorrowedRustVecMut::U128(mut v) => v.push(ptr::read(e as *const u128)),
        TypedMoveBorrowedRustVecMut::Address(mut v) => v.push(ptr::read(e as *const MoveAddress)),
        TypedMoveBorrowedRustVecMut::Signer(mut v) => v.push(ptr::read(e as *const MoveSigner)),
        TypedMoveBorrowedRustVecMut::Vector(_t, mut v) => v.push(ptr::read(e as *const MoveUntypedVector)),
        TypedMoveBorrowedRustVecMut::Struct(mut s) => s.push(e),
        TypedMoveBorrowedRustVecMut::Reference(_t, mut v) => v.push(ptr::read(e as *const MoveUntypedReference)),
    }
}
```

Whether this strategy for implementing generic Move vectors works out
in the end is yet to be determined, but it's an easy way to get something working.

There will also need to be a class of runtime calls:
functions implemented by the runtime,
and calls to which are emitted by the compiler to implement various
features that would be unwieldy to emit directly from LLVM.

The first of these I've encountered is for the `abort` Move expression.
One might expect an abort to simply emit a machine-specific abort instruction,
and the compiler could do that, but:

1) The abort mechanism might be platform-specific
2) Move `abort` carries with it an _abort code_, and handling that
   is definitely platform-specific.

So when the compiler sees `abort 5`,
it emits a call to `move_rt_abort(5)`,
and lets the runtime figure it out.

On Solana, this `abort` function is currently implemented like

```rust
pub fn abort(code: u64) -> ! {
    unsafe {
        syscalls::sol_log_64_(
            code, code, code, code, code,
        );
        syscalls::abort()
    }
}
```

The abort code is logged in a Solana-specific way,
and the program terminated in a Solana-specific way.
(There's no great reason `code` needs to be passed to `sol_log_64_`
_five_ times &mdash; but it creates an obvious and eye-catching pattern in the logs
that is hard to mistake for some other use of `sol_log_64_`).


## Testing and how everything works together

Up until now my primary goal has been to make the Move LLVM backend testable,
and that has driven all decisions about what features to implement.
It has been a great source of motivation.

Like most compilers I've seen,
our primary test cases are Move source files,
and we have multiple test harnesses that
interpret these in particular ways.
These source files contain special comments
that direct the test harness about how the source should be interpreted.

Our two most important test harnesses are:

- [`move-ir-tests`](https://github.com/solana-labs/move/tree/llvm-sys/language/tools/move-mv-llvm-compiler/tests/move-ir-tests.rs).
These convert `.move` files to multiple LLVM IR text files,
and compare the output to expected versions of the LLVM IR.
- [`rbpf-tests`](https://github.com/solana-labs/move/tree/llvm-sys/language/tools/move-mv-llvm-compiler/tests/rbpf-tests.rs).
These compile `.move` files to multiple `.o` object files,
then link them together into a `.so` dynamic library in ELF format,
and load and execute them in the [rbpf VM](https://github.com/solana-labs/rbpf).

Like the rest of the Move repo we use the [`datatest_stable`](https://docs.rs/datatest-stable/latest/datatest_stable/)
crate to drive our test harness. This crate offers a macro, `datatest_stable::harness!`,
that finds files on disk and calls a user-specified function to do whatever they
with those files.

In our `rbpf-tests` this looks like:

```rust
const TEST_DIR: &str = "tests/rbpf-tests";

datatest_stable::harness!(run_test, TEST_DIR, r".*\.move$");

fn run_test(test_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(run_test_inner(test_path)?)
}

fn run_test_inner(test_path: &Path) -> anyhow::Result<()> {
    let sbf_tools = get_sbf_tools()?;
    let runtime = get_runtime(&sbf_tools)?;

    let harness_paths = tc::get_harness_paths()?;
    let test_plan = tc::get_test_plan(test_path)?;

    if test_plan.should_ignore() {
        eprintln!("ignoring {}", test_plan.name);
        return Ok(());
    }

    tc::run_move_build(&harness_paths, &test_plan)?;

    let compilation_units = tc::find_compilation_units(&test_plan)?;
    compile_all_bytecode_to_object_files(&harness_paths, &compilation_units)?;
    let exe = link_object_files(&test_plan, &sbf_tools, &compilation_units, &runtime)?;
    run_rbpf(&test_plan, &exe)?;

    Ok(())
}
```

As an example of how everything in a language, compiler, runtime, and VM ties together I'll describe
how the Move `abort` expression works.
Getting `abort` working was a big milestone
because calling `abort` allows test cases to make assertions
that the test harness can intercept,
and implementing `abort` requires linking the runtime
and calling it from LLVM code.

In Move `abort` comes in two forms:

```move
abort 5
assert!(true == false, 5);
```

The second is a built-in macro, like Rust's own `assert!`,
that compiles down to `abort`. In Move stackless bytecode,
the abort appears as `Bytecode::Abort(AttrId, TempIndex)`,
where that `TempIndex` is the index of the stack-allocated
variable holding the abort code (`5` in our example).

In the LLVM backend `Bytecode::Abort` is handled like

```rust
sbc::Bytecode::Abort(_, local) => {
    self.emit_rtcall(RtCall::Abort(*local));
}
```

which calls

```rust
fn emit_rtcall(&self, rtcall: RtCall) {
    match &rtcall {
        RtCall::Abort(local_idx) => {
            let llfn = self.get_runtime_function(&rtcall);
            let local_llval = self.locals[*local_idx].llval;
            let local_llty = self.locals[*local_idx].llty;
            self.llvm_builder
                .load_call(llfn, &[(local_llty, local_llval)]);
            self.llvm_builder.build_unreachable();
        }
    }
}

fn get_runtime_function(&self, rtcall: &RtCall) -> llvm::Function {
    let name = match rtcall {
        RtCall::Abort(..) => "abort",
    };
    let name = format!("move_rt_{name}");
    let llfn = self.llvm_module.get_named_function(&name);
    if let Some(llfn) = llfn {
        llfn
    } else {
        let (llty, attrs) = match rtcall {
            RtCall::Abort(..) => {
                let ret_ty = self.llvm_cx.void_type();
                let param_tys = &[self.llvm_cx.int64_type()];
                let llty = llvm::FunctionType::new(ret_ty, param_tys);
                let attrs = vec![llvm::AttributeKind::NoReturn];
                (llty, attrs)
            }
        };

        let llfn = self
            .llvm_module
            .add_function_with_attrs(&name, llty, &attrs);
        llfn
    }
}
```

Like in previous examples,
the handling of `Bytecode::Abort`
first finds the correct LLVM alloca for the abort code,
then calls a helper function, `load_call`, that loads
allocas and calls a function.

Here though we have a special function for
getting "runtime functions": where normal functions are all
declared prior to function lowering,
runtime functions are only declared on demand.
So our little backend already illustrates both obvious
strategies for declaring functions: upfront and on demand.
If we wanted to declare runtime functions upfront
we would need to do some initial function traversal to discover
all the runtime functions used by a module.

So the backend emits code to call `move_rt_abort`,
expecting that it will be linked in to the final executable.
`move_rt_abort` is defined in our `move-native` runtime,
written in Rust.
Our test harness compiles the `move-native` crate to what
Rust calls a "staticlib",
otherwise known as an archive file, with a `.a` extension.
It contains a bunch of object files that the test harness
links with the Move object files using `lld`,
the LLVM Linker.

Recall that `move_rt_abort` is ultimately implemented as

```rust
pub fn abort(code: u64) -> ! {
    unsafe {
        syscalls::sol_log_64_(
            code, code, code, code, code,
        );
        syscalls::abort()
    }
}
```

The test harness then must set up the `rbpf` VM
to install syscall handlers for `sol_log_64_` and `abort`.
The code for this is verbose,
but in part it looks like

```
let mut loader = BuiltInProgram::new_loader(config);

loader
    .register_function_by_name("abort", SyscallAbort::call)
    .map_err(|e| anyhow!("{e}"))?;

pub struct SyscallAbort;

impl SyscallAbort {
    pub fn call(
        _invoke_context: &mut Context,
        _arg_a: u64,
        _arg_b: u64,
        _arg_c: u64,
        _arg_d: u64,
        _arg_e: u64,
        _memory_mapping: &mut MemoryMapping,
        result: &mut ProgramResult,
    ) {
        *result = ProgramResult::Err(EbpfError::UserError(Box::new(AbortError)));
    }
}
```

When the program running under the VM calls `abort`
(I _think_ eBPF doesn't have an explicit syscall instruction &mdash;
instead the `rbpf` VM just traps when calling particular installed functions)
the VM will call this little stub function that does nothing except
tell the VM that it needs to exit with an abort error.

With all these disparate pieces,
all executing in different phases of the compile-test cycle,
our test harness can finally handle the result of executing the VM
and properly deal with Move assertions:

```rust
let (_instruction_count, result) = vm.execute_program(true);
match result {
    Ok(0) => {}
    Err(EbpfError::UserError(e)) if e.is::<rbpf_setup::AbortError>() => {
        if let Some(expected_code) = test_plan.abort_code() {
            // Dig the abort code out of the log
            let last_event = context_object.events.last();
            match last_event {
                Some(rbpf_setup::Event::LogU64(c1, c2, c3, c4, c5)) => {
                    assert!(
                        [c1, c2, c3, c4, c5].iter().all(|c| *c == c1),
                        "all abort codes same"
                    );
                    if *c1 != expected_code {
                        panic!("unexpected abort code {c1}, expected {expected_code}");
                    }
                }
                _ => {
                    panic!("abort without abort code?!");
                }
            }
        } else {
            panic!("test aborted unexpectedly");
        }
    }
}
```

All that infrastructure to be a be able to run this 7 line Move test:

```move
// abort 10

script {
  fun main() {
    assert!(1 == 2, 10);
  }
}
```


## What's next

What we've accomplished so far is the bare minimum to write useful executable tests.

We can compile and link multiple modules,
and run them in an [rbpf](https://github.com/solana-labs/rbpf)
VM test harness.
These modules support simple functions,
scalar types and operations on them,
conditionals and branching,
referencing and dereferencing,
and aborting.

That's it!

For the near future we'll continue to implement basic language features:
scalars, unops and binops, references, structs, function calls, generics;
and runtime integration: vectors and runtime type descriptors.

Once we've got those basics pretty solid,
then we'll have to step back and take a look at the hard problems:
accessing signers and storage in a chain specific-way,
and safely making cross-module calls.
I have several ideas for different ways to tackle these problems,
but there's a lot I don't know about what the problems even are.

It is probably still too early for others to contribute to this project,
though I am curious to know which other blockchains with
LLVM-targetable VMs are interested in running Move.

