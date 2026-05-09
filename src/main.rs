
// modules
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
    // using
    use bnctx::*;
    use std::env;
    use std::process;

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
        out:    OutputFormat::Assembly(AsmFlavor::NASM),
        dest:   OutputDest::File(None)
    };

    bngen::generate_output(flat_instr_tree, target_ctx);
}

struct Config 
{
    file: std::path::PathBuf
}

impl Config
{
    fn build(args: &[String]) -> Result<Config, &'static str> 
    {
        let mut file = std::path::PathBuf::from("src_bf\\hello.bf");

        if args.len() > 1 {
            file = std::path::PathBuf::from("src_bf");
            file.push(&args[1]);
        }
    
        return Ok(Config{ file });
    }
}