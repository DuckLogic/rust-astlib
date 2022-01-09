astlib
=======
A libary for generating (and manipulating) abstract syntax trees (AST) using ASDL descriptions.

Note that an AST is very different from a parse tree, and is independent of any specific parser generator.

See this [stack overflow answer](https://stackoverflow.com/a/9864571/5713037) for details on the difference between AST/parse tree.

See this [blog post for more details on what ASDL is](https://www.oilshell.org/blog/2016/12/11.html) (and why it's still useful). It is written by the [Oil Shell](https://www.oilshell.org/) people.

## Details
Right now, the actual code generation is based on [the scripts that CPython uses](https://github.com/python/cpython/blob/v3.10.0/Parser/asdl.py).

As such, currently using this library requires Python.

In the future, I plan to move the implementation into Rust.

Ideally we can be self-hosting :)
