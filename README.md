# brain-needle
Brain-needle is a brainfuck interpreter and compiler for x86-64 assembly.
The reason for making this project is to learn some rust and the basics of creating a modular compiler.

## Implemented
- IR generation [`bnintermediate.rs`](src/bnintermediate.rs)
- Code generation [`bngen.rs`](src/bngen.rs)
    - [x86-64 linux](src/bnemit/bnemit_x86_64_linux.rs)
    - [interpreted](src/bnemit/bnemit_interpreted.rs)
- Output destination [`bndest.rs`](src/bndest.rs)
    - [stdout](src/bndest/bndest_stdout.rs)
    - [file](src/bndest/bndest_file.rs)
- Unit testing for validating generated output [`tests/`](tests/)
    - Only interpreter output has validation tests, assembly tests will be added after the first optimization passes have been implemented.
- Argument parser and config builder [`bnconfig.rs`](src/bnconfig.rs)

## Currently working on
Implementing optimization passes for the assembly generation.

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
