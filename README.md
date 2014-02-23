# Rum

## About

Rum is currently an experiment. Its goal is to become an HTML5 game framework
using [Rust](http://rust-lang.org) via [Emscripten](http://emscripten.org).

All of this is pretty raw at the moment. It has only been tested on Ubuntu 13.10
64-bit.


## Prerequisites

### Rust

You need a working Rust installation. At the time of writing, I've been
successful with the following:

> $ rustc --version
> rustc 0.10-pre (d22099c 2014-02-22 01:11:50 -0800)
> host: x86_64-unknown-linux-gnu


### Emscripten & LLVM

Stock emscripten won't work with Rust because LLVM 3.2 (used by emscripten)
can't handle the IR (intermediate representation) produced by the 3.5
pre-release version that Rust uses.

To solve this problem, you need to install a recent version of LLVM (I've had
success with commit 589d6377251d39e5f4d866bcb495bf6e547b4372 of LLVM's Git
mirror). The following steps describe how to set up emscripten with the latest
version.

1. Clone LLVM:
   $ git clone http://llvm.org/git/llvm.git
2. Clone Clang:
   $ cd llvm/tools
   $ git clone http://llvm.org/git/clang.git
3. Build LLVM + Clang:
   $ cd llvm
   $ ./configure --enable-optimized --disable-assertions
   $ make
4. Clone emscripten:
   git clone https://github.com/kripken/emscripten.git
5. Generate emscripten configuration file:
   $ cd emscripten
   $ ./emcc
6. Configure emscripten:
   $ cd llvm/Release/bin
   $ pwd
   Note down that path.
   $ vim ~/.emscripten
   Comment out the "LLVM_ROOT =" line and replace it with the following:
   LLVM_ROOT = '<insert path here>'
7. Add the emscripten directory to your PATH variable.


##  Compile and run the sample program

$ ./run
