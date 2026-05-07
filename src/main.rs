
use std::env;
use std::path;
use std::process;

mod bnlex;
mod bnparse;
mod testhelp;

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

    println!("\n[Tokens]");
    for token in &tokens {
        print!("{:?} ", token);
    }

    let flat_instr_tree = bnparse::create_flat_instr_tree_from_tokens(tokens).unwrap_or_else(|err| {
        println!("Parsing error: {err}");
        process::exit(1);
    });

    println!("\n\n[Flat Instruction Tree]");
    for node in &flat_instr_tree {
        print!("{:?} ", node);
    }
}

struct Config 
{
    file: path::PathBuf
}

impl Config
{
    fn build(args: &[String]) -> Result<Config, &'static str> 
    {
        if args.len() < 2 {
            return Err("to few arguments, file name is needed");
        }
    
        let file = {
            let mut path = path::PathBuf::from("brainfuck_src");
            path.push(&args[1]);
            path
        };
    
        return Ok(Config{ file });
    }
}