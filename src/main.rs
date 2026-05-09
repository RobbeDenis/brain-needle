
use std::env;
use std::path;
use std::process;

mod bnlex;
mod bnparse;

mod bngen;
mod bngen_ctx;

mod bngen_emit;
mod bngen_x86_64_linux;

mod bngen_dest;
mod bngen_stdout;

mod testhelp;

use crate::bngen::generate_output;
use crate::bngen_ctx::*;


fn main() 
{
    let args: Vec<String> = env::args().collect();

        let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let tokens = bnlex::tokenize_file_from_path(config.file).unwrap_or_else(|err| {
        println!("Lexer error: {err}");
        process::exit(1);
    });

    // println!("\n[Tokens]");
    // for token in &tokens {
    //     print!("{:?} ", token);
    // }

    let flat_instr_tree = bnparse::create_flat_instr_tree_from_tokens(tokens).unwrap_or_else(|err| {
        println!("Parsing error: {err}");
        process::exit(1);
    });

    // println!("\n\n[Flat Instruction Tree]");
    // for node in &flat_instr_tree {
    //     print!("{:?} ", node);
    // }


    let target_ctx = TargetContext { 
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        out:    OutputFormat::Assembly(AsmFlavor::NASM),
        dest:   OutputDest::File(None)
    };

    generate_output(flat_instr_tree, target_ctx);
}

struct Config 
{
    file: path::PathBuf
}

impl Config
{
    fn build(args: &[String]) -> Result<Config, &'static str> 
    {
        let mut file = path::PathBuf::from("src_bf\\hello.bf");

        if args.len() > 1 {
            file = path::PathBuf::from("src_bf");
            file.push(&args[1]);
        }
    
    
        return Ok(Config{ file });
    }
}