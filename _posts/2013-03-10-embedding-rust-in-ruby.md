---
layout: post
title: "Embedding Rust in Ruby"
tags: [rust]
---

One of the most requested features of Rust is the ability to call Rust functions from other languages.
So far this has been completely impossible because Rust code depends on a runtime,
and the Rust runtime is not embeddable (yet).
As part of my work to reimplement Rust's scheduler I am trying also to make Rust more embeddable,
and we've reached the point now on the 'incoming' branch where one can usefully call into Rust.

As an example,
I want to build a web application in Ruby that applies a gaussian blur to a canvas.
To start with, here's a Ruby function that applies a blur to a grayscale image
(preemptive apologies for the quality of my Ruby and DSP code):

    def blur_ruby(width, height, data)

      filter = [[0.011, 0.084, 0.011],
                [0.084, 0.619, 0.084],
                [0.011, 0.084, 0.011]]

      newdata = []

      # Iterate through the pixels of the image
      (0...height).each do |y|
        (0...width).each do |x|
          new_value = 0
          # Iterate through the values in the filter
          (0...filter.length).each do |yy|
            (0...filter.length).each do |xx|
              x_sample = x - (filter.length - 1) / 2 + xx
              y_sample = y - (filter.length - 1) / 2 + yy
              sample_value = data[width * (y_sample % height) + (x_sample % width)]
              weight = filter[yy][xx]
              new_value += sample_value * weight
            end
          end
          newdata[width * y + x] = new_value
        end
      end

      newdata
    end

This takes an array of `width * height` grayscale values from 0-255 and applies a blur,
producing a new array with the same length.
It's not that fast, so it would rather be written in Rust.
An equivalent function in Rust might be

    fn blur_rust(width: uint, height: uint, data: &[u8]) -> ~[u8] {

        let filter = [[0.011, 0.084, 0.011],
                      [0.084, 0.619, 0.084],
                      [0.011, 0.084, 0.011]];

        let mut newdata = ~[];

        for uint::range(0, height) |y| {
            for uint::range(0, width) |x| {
                let mut new_value = 0.0;
                for uint::range(0, filter.len()) |yy| {
                    for uint::range(0, filter.len()) |xx| {
                        let x_sample = x - (filter.len() - 1) / 2 + xx;
                        let y_sample = y - (filter.len() - 1) / 2 + yy;
                        let sample_value = data[width * (y_sample % height) + (x_sample % width)];
                        let sample_value = sample_value as float;
                        let weight = filter[yy][xx];
                        new_value += sample_value * weight;
                    }
                }
                newdata.push(new_value as u8);
            }
        }

        return newdata;
    }

A direct translation of the Ruby to Rust is pretty trivial.
What we really care about is how to call that function from Ruby, and the way we will
do so is by producing a public wrapper function, written in Rust,
that follows the *C calling conventions* (or ABI) instead of the Rust calling conventions.
To create a foreign-ABI function we use the `extern` keyword.

    use core::libc::c_uint;

    pub extern fn blur(width: c_uint, height: c_uint, data: *mut u8) {
        ...
    }

By default foreign functions use the C calling convention,
but you could also use others with the `abi` attribute, as in the `#[abi = "stdcall"]`
attribute for interoperating with the Win32 API.
*Note that the FFI is undergoing some [significant changes](https://github.com/mozilla/rust/issues/3678)
and while the syntax discussed here will likely remain valid
some of the details are still being ironed out.*

In our foreign functions we largely want to stick to -- or at least start with -- C types and unsafe pointers.
Rust has a [comprehensive set of bindings](https://github.com/mozilla/rust/blob/master/src/libcore/libc.rs)
to C in `core::libc` and when interfacing to foreign
code you'll often employ types like `c_char`, `c_int`, `size_t`, `uintptr_t`.
It's possible to pass Rust types like `&[u8]` through foreign functions but it can by tricky,
requires understanding the type's representation and how it will be treated by the foreign calling convention.
I usually avoid it.

For simplicity, we'll use the same `data` buffer for both input and output,
so we've declared the `data` pointer mutable, `*mut u8`, to indicate we'll be writing to it.
Dealing with mutability qualifiers across the language boundary can get imprecise
since C has very weak notions about immutability,
but you should generally try to make unsafe pointers have the proper mutability
from Rust's point of view and avoid casting them around.

OK, so we've decided on the proper function signature to present to foreign code.
Now we've got to adapt this signature consisting of foreign and unsafe types
to call the Rust-typed blur function.
We're entering the realm of unsafe code and while Rust has lots of potential
for writing low-level code, the libraries for such are not exactly cohesive nor complete.
You'll want to get familiar with the [`core::vec::raw`][vec_raw], [`core::str::raw`][str_raw] and
[`core::ptr`][ptr] modules when doing this sort of interop,
but you may find that utility functions you really, really want don't actually exist yet.
The code in these modules tends to be added as needed,
and I even had to make changes to `core` to make this project work nicely.
I hope we can clean up a lot of this before Rust 1.0.

[vec_raw]: http://static.rust-lang.org/doc/0.5/core/vec_raw.html
[str_raw]: http://static.rust-lang.org/doc/0.5/core/str_raw.html
[ptr]: http://static.rust-lang.org/doc/0.5/core/ptr.html

So many digressions. Anyway, here's how to write the adapter:

    pub extern fn blur(width: c_uint, height: c_uint, data: *mut u8) {
        let width = width as uint;
        let height = height as uint;
        unsafe {
            do vec::raw::mut_buf_as_slice(data, width * height) |data| {
                let out_data = blur_rust(width, height, data);
                vec::raw::copy_memory(data, out_data, width * height);
            }
        }
    }

`mut_buf_as_slice` takes a `*mut T` and temporarily casts it as an `&mut [T]`.
Almost any time I'm dealing with an unsafe pointer to a buffer
the first thing I want to do is convert it to a Rust slice.
Once we've got a slice of our buffer we've got all the types needed to call
the Rust blur function, which returns a new `~[u8]` of the modified image.
Finally `copy_memory` does a simple blit from the new buffer back to the
slice of the input/output buffer argument.
`copy_memory` is just a `memmove` equivalent and is only safe
because we're dealing with a buffer of POD (plain-old-data) types.

If we were to compile this to a Rust library we would still run into one final problem.
Symbols output by Rust are *mangled* to contain their module
path as well as versioning information.
As written, rustc will produce a library in which the `blur` symbol is
named something inscrutable like `_ZN4blur16_f15a16294e229a23_00E`.
While we *could* tell our Ruby code to find the function using that name,
it will be easier to maintain if we don't have to deal with the mangling.
To prevent mangling we can apply the `#[no_mangle]` attribute, after
which the function's symbol will simply be called `blur`.

    #[no_mangle]
    pub extern fn blur(width: c_uint, height: c_uint, data: *mut u8) {
        ...


Finally we need to call this from Ruby using their FFI.
Start by requiring the `ffi` library and defining a module to represent
our Rust blur library.

    require 'ffi'

    module RustBlur
      extend FFI::Library
      ffi_lib 'libblur-68a2c114141ca-0.0'

      attach_function :blur, :blur, [ :uint, :uint, :pointer ], :void
    end

This binds the RustBlur module to our library and defines a method called `blur`.
Note that `ffi_lib` has to specify the full name of the library, including
the version hash.
I can imagine it being a hassle to keep that version
hash in sync during development.

Finally, we need a little glue function to convert the Ruby data array
to a buffer of C chars. In Ruby you use strings to represent binary
data so this involves packing and unpacking to and from a string.

    def blur_rust(width, height, data)
      packed_data = data.pack("C*")
      raw_data = FFI::MemoryPointer.from_string(packed_data)
      RustBlur.blur(width, height, raw_data)
  
      raw_data.get_bytes(0, width * height).unpack("C*")
    end

That's it! Now we've got all the peices to connect Ruby with Rust. What a lovely combination.
As you would hope, the optimized Rust version is more than 20x faster than the Ruby,
so that's promising for people wanting to give their dynamic language codebases a kick with Rust.
If you want to build and run the full application, the [code is on GitHub](https://github.com/brson/rubyrustdemo).
It's also [running on EC2 temporarily](http://rubyrustdemo-1226433825.us-west-2.elb.amazonaws.com/index.html).

Now for the caveats.

This is very preliminary and there's a lot that doesn't work.
When calling Rust code you will not be executing in a Rust task and will not have access to any
runtime services that require task-local resources.
Currently this means you can't use the local heap,
nor can you spawn or communicate with tasks, nor call `fail!()` to unwind the stack.
I/O doesn't work because `core::io` (unfortunately, and incorrectly) uses `@`-boxes.
Even logging does not work.
Calling any code that tries to access the task context will cause the process to abort.
Because code is not executing in a task, it does not grow the stack, and instead runs on
whatever stack the foreign caller was executing on.
Recurse too deep and you will scribble on random memory.

Most or all of those limitations will evaporate over time.
