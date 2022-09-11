# NanoGL

NanoGL is a pure-Rust, _extremely_ extremely lightweight OpenGL library for computer graphics.

It internally uses a `c2rust` port of the [Tigr](https://github.com/erkkah/tigr) library to Rust, with ongoing work to rustify its API.

## Example

See [demo.rs](./examples/demo.rs)

## Use case

Please don't use this library if you are learning OpenGL or working on a substantial project! NanoGL is my experiment at writing the absolute most barebones OpenGL library possible in Rust. It is very easy to break and cause black holes to open in your computer!

## Features

- **Zero dependencies:** actually no dependencies whatsoever! Not even libc!
- **Low-level access:** being minimalist has advantages! It's easy to have fine-grained control over the entire rendering pipeline.
- **Cross-platform:** works on essentially all modern operating systems.
- **Public domain:** the most open and permissive license you can imagine. Do whatever you want with it!

## Planned features

- **Safe wrapper API:** NanoGL should avoid undefined behavior whenever possible and provide a safe, Rust-style API
- **Zero-cost abstractions:** whenever possible, use abstractions to ease graphics programming without compromising performance
