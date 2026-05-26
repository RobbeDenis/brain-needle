# brain-needle
Brain-needle is a brainfuck interpreter and compiler for x86-64 assembly.
The initial goal of this project was to learn rust, by making a simple interpreter.
But it quickly turned into learning about modular compiler design rather then rust.

## Implemented
- IR generation [`bnintermediate.rs`](src/bnintermediate.rs)
- Code generation [`bngen.rs`](src/bngen.rs)
    - [x86-64 linux](src/bnemit/bnemit_x86_64_linux.rs)
    - [interpreted](src/bnemit/bnemit_interpreted.rs)
- Output destination [`bndest.rs`](src/bndest.rs)
    - [stdout](src/bndest/bndest_stdout.rs)
    - [file](src/bndest/bndest_file.rs)
- Validation tests for the generated output [`tests/`](tests/)
    - Only interpreter output has validation tests, concrete tests for assembly output will be added after the golden tests and the first optimization pass has been implemented.
- Argument parser and config builder [`bnconfig.rs`](src/bnconfig.rs)
- Criterion for benchmarks

## Currently working on
Setting up an environment for golden tests.

Implementing optimization passes for the assembly generation. This involves creating a more streamlined way to setup the environment, e.g., bit-width, tape-width, wrapping, EOF and debug print.

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
