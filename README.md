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
- Unit testing framework for validating output [`tests/`](tests/)
    - NASM tests still have to be implemented
- Partial argument parser [`src/bnconfig.rs`](src/bnconfig.rs)

## Arguments
```terminal
Usage: [INPUT_FILE | -h | --help] [FLAGS]

FLAGS:
  -o, --out  <OUTPUT>             Specify output file path
  -d, --dest <DEST>               Specify destination type
              file                Writes the output to a file
              stdout              Writes the output using stdout
  -f, --fmt  <FORMAT>             Specify output format
              nasm                Compiles brainfuck to NASM
              interpret           Directly interprets brainfuck and writes to the output
```
