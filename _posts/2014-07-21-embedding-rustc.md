---
layout: post
title: "Embedding and customizing the Rust compiler"
tags: [rust]
---

I recently created a [project][chamber] that uses the Rust compiler as a library.
Although rustc has long been built as a library,
little effort has been put into making the API nice.
Although I've done this a few times,
I was pleasantly surprised that it's really not
hard to embed rustc if you know which functions to wire together.

For simple use cases most of the API you need comes from
the [rustc::driver] module, though any code that touches the compiler
will inevitably also need to refer to the AST, in [syntax::ast].

*Warning: the entire compiler API is experimental and does change regularly.*

## Running rustc from your own Rust code

Rust comes with everything you need to extend rustc;
put this at the top of your crate:

    extern crate syntax; // The Rust parser.
    extern crate rustc;  // The Rust compiler.

All that it takes to run the compiler is to call `rustc::driver::driver::compile_input`. Unfortunately, it takes some effort to wrangle all the correct arguments.

Here's what my new tool, Chamber, does:

    // This is our own application configuration. We're going to
    // translate it to rustc's (moderately complex) configuration.
    let ref config = config;

    // Build the `Options` struct from our own configuration.
    // `Options` provides the configuration for constructing `Session`,
    // which is the context to run the compiler pipeline one time.
    let sopts: Options = build_session_options(config);

    // Create the "diagnostics registry". This is what
    // maintains error codes and extended error documentation.
    let registry: Registry = Registry::new(rustc::DIAGNOSTICS);

    // Create the `Session` from the `Options`.
    // The name of the source file provided here is only to inform
    // debuginfo generation AFAICT.
    let source = config.input_file.clone();
    let sess: Session = build_session(sopts, Some(source), registry);

    // Builds the set of `#[cfg(...)]` idents in effect, combining
    // defaults with those derived from `Session` options.
    let cfg: ast::CrateConfig = build_configuration(&sess);

    // This source code comes from a file (`FileInput`),
    // not in memory (`StrInput`).
    let ref input_file = FileInput(config.input_file.clone());

    // Some standard rustc options.
    let ref out_dir = config.out_dir;
    let ref out_file = config.out_file;

    // Our custom plugins that we want to run.
    let plugins: Plugins = get_chamber_plugins(config);

    compile_input(sess, cfg, input_file, out_dir, out_file, Some(plugins));

Hopefully, the comments explain well enough what's happening.
For the curious about compiler innards, `compile_input`
is probably [worth reading][compile_input],
as it defines the entire compiler pipeline.

The most important step there is creating the `Options` value
that configures the compiler. `Options` is quite a robust
data type, but rustc provides a prototype via the `basic_options`
function, which can be used as defaults via FRU
(functional record update):

Here is Chamber's `build_session_options`:

    /// Converts our app-specific options to a rustc `Options`.
    fn build_session_options(config: &Config) -> Options {

        use rustc::back::link::OutputTypeExe;
        use rustc::driver::config::basic_options;
        use std::cell::RefCell;

        // Convert from Vec<T> to HashSet<T> like magic.
        let search_paths = config.search_paths.clone();
        let search_paths = search_paths.move_iter().collect();

        Options {
            // If this is empty rustc will just pick a crate type.
            crate_types: config.crate_types.clone(),

            // -L paths.
            addl_lib_search_paths: RefCell::new(search_paths),

            // The "sysroot" a directory rustc uses as a reference point
            // for various operations, including discovering crates. It is
            // often "/usr/local". *By default rustc infers it to be the
            // directory above the directory containing the running
            // executeable.* In our case that executable is probably
            // called `chamber` and is not located anywhere near the
            // sysroot.
            maybe_sysroot: config.sysroot.clone(),

            // Output a final binary. rustc will output nothing by default.
            output_types: vec!(OutputTypeExe),

            // The name of the library we'll be using as 'std', the 'chamber'.
            alt_std_name: Some(config.chamber_name.clone()),

            // Don't try to fill out all of `Options` by hand.
            // Use this prototype!
            .. basic_options()
        }
    }

## Plugins

OK, now go back and look again at the `compile_input` invocation:

    // Our custom plugins that we want to run.
    let plugins: Plugins = get_chamber_plugins(config);

    compile_input(sess, cfg, input_file, out_dir, out_file, Some(plugins));

Chamber customizes rustc by plugging in [its own lint passes][lints].
These passes are contained in a separate `chamber_plugins` crate,
which is seamlessly rebuilt by [Cargo] as necessary.

Although I don't want to go into detail,
creating lints is nearly as easy as writing the following.

    pub fn plugin_registrar(reg: &mut Registry) {
        reg.register_lint_pass(box FeatureGatePass);
    }

    /// Forbids using the `#[feature(...)]` attribute
    struct FeatureGatePass;

    declare_lint!(CH_FEATURE_GATE, Forbid,
                  "enabling experimental features")

    impl LintPass for FeatureGatePass {
        fn get_lints(&self) -> LintArray {
            lint_array!(CH_FEATURE_GATE)
        }

        fn check_attribute(&mut self, ctx: &Context, attr: &ast::Attribute) {

            use syntax::attr;

            if attr::contains_name(&[attr.node.value], "feature") {
                ctx.span_lint(CH_FEATURE_GATE, attr.span, "chamber: feature gate");
            }
        }
    }

## Handling errors

rustc was designed in another era: it's error handling mechanism
involves logging to somewhere and then eventually calling `fail!`.
You wouldn't design it like that now. On top of that, since rustc is
also still somewhat prone to ICEing (hitting a logic error and calling
`fail!`), you really must put it into it's own task to run it reliably
(score 1 for task isolation and recovery).

The compiler provides a `monitor` function that runs the compiler in
another task. `monitor` [does ugly stuff that you don't want to do][ugly],
like configure rustc's surprisingly complex error handling, doing
something useful with ICE's, and setting the proper stack size.

## Rust on

OK, now you have a vague idea about how to use rustc as a library.
Check out the [source to the tool][chamber] for
more details. It's very simple and extensively commented.

To run Chamber (requires very recent Rust nightly, like from tomorrow probably):

    $ git clone git://github.com/brson/rust-chamber
    $ cd rust-chamber
    $ cargo build
    $ target/chamber example.rs

[chamber]: https://github.com/brson/rust-chamber
[rustc::driver]: http://doc.rust-lang.org/rustc/driver
[syntax::ast]: http://doc.rust-lang.org/syntax/ast
[Cargo]: http://crates.io
[lints]: https://github.com/brson/rust-chamber/tree/master/src/chamber_plugin/lib.rs
[compile_input]: https://github.com/rust-lang/rust/tree/master/src/librustc/driver/driver.rs
[ugly]: https://github.com/rust-lang/rust/blob/97ca98f5ccda65589049397723662e634ada04a4/src/librustc/driver/mod.rs#L421