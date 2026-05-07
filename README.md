# brain-needle
Brain-needle is a work in progress brainfuck compiler, the goal is to compile to x86-64 assembly.
The reason for making this project is to familiarise myself with rust.

## Implemented
- Tokenization [`src/bnlex.rs`](src/bnlex.rs)
    - tokenize_file_from_path
        - returns tokens as `Vec<Token>`
    - get_amount_inseq_consume
        - collapses sequence of inc, dec and shifts into one token
- Parsing to Flat AST [`src/bnparse.rs`](src/bnparse.rs)
    - create_flat_instr_tree_from_tokens
        - returns tree as `Vec<InstrNode>`
        - uses a queue for jump instructions to ensure single pass parsing
