# Rum

## About

Rum is an experiment. Its goal is to become an HTML5 game framework using
[Rust](http://rust-lang.org) via [Emscripten](http://emscripten.org). At the
moment, there's just a sample Rust program that is compiled to JavaScript and
run with node.js.

All of this is pretty raw at the moment. It has only been tested on Ubuntu 13.10
64-bit.


## Prerequisites

You need a working [node.js](http://nodejs.org). I've tested with v0.10.26, but
I don't think this is very much version-specific.

You'll also need some basics to build Rust and LLVM: make, a C/C++ compiler,
that kind of stuff. If you run into any problems, feel free to add your
solutions to this README and send me a pull request!

This repository includes Rust, Emscripten and LLVM, so you're covered on that
front.


## Usage

Before using this the first time, you need to initialize the repository:

> $ ./scripts/init

After that, you can compile and run the Rust program (source/main.rs) with the
following command:

> $ ./scripts/run


## Limitations

As you can see, currently the Rust program is very basic, kind of a
proto-"Hello, World". The reasons for this is that currently, only programs that
don't require any kind of Rust library (including std) will run. Trying to write
an actual "Hello, World" (i.e. using a string) already requires the std.

As soon as I have some time to invest in this again, I will try to overcome this
limitation and support "real" Rust programs.
