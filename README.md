# brain-needle
Brain-needle is a brainfuck interpreter and compiler for x86-64 assembly.
The reason for making this project is to familiarise myself with rust and learn the basics of creating a modular basic compiler.

## Implemented
- Tokenization [`src/bnlex.rs`](src/bnlex.rs)
- Parsing to Flat AST [`src/bnparse.rs`](src/bnparse.rs)
- Code generation [`src/bngen.rs`](src/bngen.rs)
    - [x86-64 linux](src/bnemit/bnemit_x86_64_linux.rs)
    - [interpreted](src/bnemit/bnemit_interpreted.rs)
- Output destination [`src/bndest.rs`](src/bndest.rs)
    - [stdout](src/bndest/bndest_stdout.rs)
    - [file](src/bndest/bndest_file.rs)
- Unit testing helpers for validating output [`tests/`](tests/)
    - Only interpreter has complete unit tests, NASM tests still have to be implemented
- Argument parser and config builder [`src/bnconfig.rs`](src/bnconfig.rs)

## Usage
```terminal
Usage: [INPUT | -h | --help] [OPTIONS]

Options:
  -o, --out  <OUTPUT>             Specify output file path [default: INPUT.*]
  -d, --dest <DEST>               Specify destination output type [default: stdout]
              stdout              Writes using stdout
              file                Writes to a file
  -f, --fmt  <FORMAT>             Specify output format [default: interpret]
              interpret           Directly interprets and writes brainfuck to the output
              nasm                Compiles brainfuck to NASM
  -a, --arch <ARCH>               Specify target architecture [default: x86_64]
              x86-64              Uses the x86-64 AMD/Intel instruction set
  -t, --os   <OS>                 Specify target operating system [default: linux]
              linux               Uses linux system calls
```