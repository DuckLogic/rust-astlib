astlib
=======
A libary for generating (and manipulating) abstract syntax trees (AST) based on ASDL descriptions.

This is in contrast to most parser generators (Antlr, Pest, etc), which almost exclusively generate *parse trees*.

## AST vs Parse Tree

| Parse Tree                                         |  AST                                        |
|----------------------------------------------------|---------------------------------------------|
| ![parse tree](https://i.stack.imgur.com/SyonV.png) | ![AST](https://i.stack.imgur.com/dhd3v.png) |

As you can see, the AST excludes things . Things like paranthesized are also dropped, so grouping is inferred  ()

See [this stackoverflow answer](https://stackoverflow.com/a/9864571/5713037) for more details.

## What is ASDL?
ASDL is a way to specify the layout of your ASTs automatically.

It is most notably used by CPython, to [specify the official python ast](https://github.com/python/cpython/blob/v3.10.0/Parser/Python.asdl).

See this [blog post for more details on what ASDL is](https://www.oilshell.org/blog/2016/12/11.html) (and why it's still useful). It is written by the [Oil Shell](https://www.oilshell.org/) people.

## Features
This library provides two things:

1. A way to automatically generate an AST from an ASDL descripiton (`astlib-meta`)
   - Unfrotunately, this currently requires `python` (see issue #1)
   - Seperate from the main crate (and can be run independently)
2. `astlib` is the runtime, seperate from the codgen. It incldues several utilites to analyse and manipulate ASTs, including:
   - Printing to lisp-style s-expressions
   - Basic traits

### Runtime dependencies
The generated ASTs only have two required dependencies:
1. The `astlib` crate
2. The `derivative` crate (for excluding `Span`s from generated `Eq` implementations)

The library lalso has a number of optional features, some of which are on by default
1. `lisp` for printing lisp trees (on by default)
2. `builtins` to enable a simple builtin `Constant` and `Ident`
   - *TODO*: This is currently a required feature.
3. `codemap` for better (and faster) `Spans` (off by default)
   - The alternative (and default impl) is essentially `Span { start: usize, end: usize}` 
4. `serde` support for generated ASTs
   - By default spans are excluded from the serialized representation

## Build Dependencies: Python
Right now, the actual code generation is based on [the scripts that CPython uses](https://github.com/python/cpython/blob/v3.10.0/Parser/asdl.py).

As such, actually converting ASDL -> AST requires Python.

To be clear, this is only a build dependency and it can be run ahead of time. It is sperated into its own crate `astlib-meta`.

The only dependency of the python scripts is `click`.

In the future, I plan to move the implementation into Rust (See github [issue #1](https://github.com/DuckLogic/rust-astlib/issues/1)).

Ideally we can be self-hosting :)
