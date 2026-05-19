
use std::{env, path::PathBuf};
use crate::bnctx::*;

#[derive(Debug, Default)]
pub struct Args
{
    pub caller: String,
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub dest_type: String
}

impl Args
{
    pub fn parse_from_env() -> Args
    {
        return Self::parse_from_args(env::args().collect());
    }

    pub fn parse_from_args(args: Vec<String>) -> Args
    {
        if args.is_empty() {
            eprintln!("Argument parser error: the argument list is empty");
            std::process::exit(1);
        }
        else if args.len() == 1 {
            eprintln!("Argument parser error: missing first argument\nUsage: [INPUT_FILE | -h | --help] [FLAGS]");
            std::process::exit(1);
        }

        let mut args_iter = args.into_iter();

        let mut parsed_args = Args {
            caller: args_iter.next().unwrap_or("unkown".into()),
            ..Args::default()
        };

        if let Some(second) = args_iter.next() {
            if second == "-h" || second == "--help" {
                print_help();
                std::process::exit(0);
            }
            parsed_args.input_file = PathBuf::from(second);
        }

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-d" | "--dest" => {
                    match args_iter.next() {
                        Some(value) => parsed_args.dest_type = value.into(),
                        _ => {
                            eprintln!("Argument parser error: The flag '{}' requires destination type", arg);
                            std::process::exit(1);
            }}}
                "-o" | "--out" => {
                    match args_iter.next() {
                        Some(value) => parsed_args.output_file = value.into(),
                        _ => {
                            eprintln!("Argument parser error: The flag '{}' requires output file path", arg);
                            std::process::exit(1);
            }}}
                _ => { 
                    eprintln!("Argument parser error: Unknown flag: {}", arg);
                    std::process::exit(1);
        }}}

        return parsed_args;
    }
}

fn print_help() 
{
    println!("Usage: [INPUT_FILE | -h | --help] [FLAGS]");
    println!("\nFLAGS:");

    println!("  -o, --out <OUTPUT>\t\tSpecify output file path");

    println!("  -d, --dest <DEST> \t\tSpecify destination type");
    println!("             file   \t\tWrites the output to a file");
    println!("             stdout \t\tWrites the output using stdout");
}

#[derive(Debug)]
pub struct Config
{
    pub file_path: PathBuf,
    pub context: TargetContext
}

impl Config
{
    pub fn build(args: &Args) -> Result<Config, &'static str>
    {
        let path = args.input_file.clone();
        let mut ctx = TargetContext::default();

        match args.dest_type.as_str() {
            "stdout" => ctx.dest = OutputDest::Stdout,
            "file" => ctx.dest = OutputDest::File(Some("output".into())),
            _ => ctx.dest = OutputDest::Stdout
        }

        return Ok(Config { file_path: path, context: ctx });
    }
}

//////////////////////////
/////// PARSE TEST ///////
//////////////////////////

#[cfg(test)]
mod parse_tests
{
    use super::*;

    #[test]
    fn parse_short_flags()
    {
        let args:Vec<String>= vec!["program", "input.bf", "-o", "output", "-d", "stdout"]
            .into_iter()
            .map(String::from)
            .collect();

        let args = Args::parse_from_args(args);
        assert!(args.caller == "program");
        assert!(args.input_file == PathBuf::from("input.bf"));
        assert!(args.dest_type == "stdout");
    }

    #[test]
    fn parse_long_flags()
    {
        let args:Vec<String>= vec!["program", "input.bf", "--out", "output", "--dest", "stdout"]
            .into_iter()
            .map(String::from)
            .collect();

        let args = Args::parse_from_args(args);
        assert!(args.caller == "program");
        assert!(args.input_file == PathBuf::from("input.bf"));
        assert!(args.dest_type == "stdout");
    }
}

//////////////////////////
/////// CONFIG TEST //////
//////////////////////////

#[cfg(test)]
mod config_tests
{
    use super::*;

    #[test]
    fn build_config()
    {
        let args = Args { 
            caller: "program".into(), 
            input_file: PathBuf::from("input.bf"), 
            dest_type: "stdout".into(), 
            output_file: "output".into()
        };

        let config = Config::build(&args).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert!(config.file_path == args.input_file);
        assert!(config.context.dest == OutputDest::Stdout);
    }
}