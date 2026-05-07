
use std::env;
use std::path;
use std::process;

mod bnlex;

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

    for token in &tokens {
        print!("{:?} ", token);
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