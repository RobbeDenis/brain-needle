use std::path::PathBuf;


// modules
mod bncore;
mod bnctx;
mod bndest;
mod bnemit;
mod bngen;
mod bnlex;
mod bnparse;

// public crate modules
pub(crate) mod bn_test_helper;


fn main() 
{
    use bnctx::*;
    use std::env;
    use std::process;
    use bncore::Config;

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let tokens = bnlex::tokenize_file_from_path(config.file).unwrap_or_else(|err| {
        println!("Lexer error: {err}");
        process::exit(1);
    });

    let flat_instr_tree = bnparse::create_flat_instr_tree_from_tokens(tokens).unwrap_or_else(|err| {
        println!("Parsing error: {err}");
        process::exit(1);
    });

    let target_ctx = TargetContext { 
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::File(Some(PathBuf::from("output\\dump.txt")))
    };

    bngen::generate_output(flat_instr_tree, target_ctx);
}