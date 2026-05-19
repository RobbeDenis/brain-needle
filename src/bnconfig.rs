
use std::{env, path::PathBuf};
use crate::bnctx::*;

#[derive(Debug, Default)]
pub struct Args
{
    pub caller: String,
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub output_format: String,
    pub dest_type: String,
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
                            eprintln!("Argument parser error: The flag '{}' requires a destination type", arg);
                            std::process::exit(1);
            }}}
                "-o" | "--out" => {
                    match args_iter.next() {
                        Some(value) => parsed_args.output_file = value.into(),
                        _ => {
                            eprintln!("Argument parser error: The flag '{}' requires an output file path", arg);
                            std::process::exit(1);
            }}}
                "-f" | "--fmt" => {
                    match args_iter.next() {
                        Some(value) => parsed_args.output_format = value.into(),
                        _ => {
                            eprintln!("Argument parser error: The flag '{}' requires an output format", arg);
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
    println!("\nUsage: [INPUT_FILE | -h | --help] [FLAGS]");
    println!("\nFLAGS:");

    println!("  -o, --out  <OUTPUT>\t\t  Specify output file path");

    println!("  -d, --dest <DEST> \t\t  Specify destination type");
    println!("              file   \t\t  Writes the output to a file");
    println!("              stdout \t\t  Writes the output using stdout");

    println!("  -f, --fmt  <FORMAT>\t\t  Specify output format");
    println!("              nasm    \t\t  Compiles brainfuck to NASM");
    println!("              interpret\t\t  Directly interprets brainfuck and writes to the output");

    println!("");
}

#[derive(Debug)]
pub struct Config
{
    pub file_path: PathBuf,
    pub context: TargetContext
}

impl Config
{
    pub fn build(args: &Args) -> Config
    {
        let path = args.input_file.clone();
        let mut ctx = TargetContext::default();

        match args.output_format.as_str() {
            "interpret" => ctx.format = OutputFormat::Interpreted,
            "nasm" => ctx.format = OutputFormat::Assembly(AsmFlavor::NASM),
            _ => ctx.format = OutputFormat::Interpreted
        }

        match args.dest_type.as_str() {
            "stdout" => ctx.dest = OutputDest::Stdout,
            "file" => {
                if args.output_file.as_os_str().is_empty() {
                    ctx.dest = OutputDest::File(Some("output".into()))
                } else {
                    ctx.dest = OutputDest::File(Some(args.output_file.clone()))
            }},
            _ => ctx.dest = OutputDest::Stdout
        }

        return Config { file_path: path, context: ctx };
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
        let args:Vec<String>= vec!["program", "input.bf", "-o", "output", "-d", "stdout", "-f", "nasm"]
            .into_iter()
            .map(String::from)
            .collect();

        let args = Args::parse_from_args(args);
        assert!(args.caller == "program");
        assert!(args.input_file == PathBuf::from("input.bf"));
        assert!(args.dest_type == "stdout");
        assert!(args.output_format == "nasm");
    }
    
    #[test]
    fn parse_long_flags()
    {
        let args:Vec<String>= vec!["program", "input.bf", "--out", "output", "--dest", "stdout", "--fmt", "nasm"]
        .into_iter()
        .map(String::from)
        .collect();
    
        let args = Args::parse_from_args(args);
        assert!(args.caller == "program");
        assert!(args.input_file == PathBuf::from("input.bf"));
        assert!(args.dest_type == "stdout");
        assert!(args.output_format == "nasm");
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
    fn build_default_config()
    {
        let mut args = Args {
            caller: "caller".into(),
            input_file: PathBuf::from("input.bf"),
            ..Args::default()
        };

        // complete default requires at least caller and input file
        let config = Config::build(&args);
        assert!(config.file_path == args.input_file);
        assert!(config.context.dest == OutputDest::Stdout);
        assert!(config.context.format == OutputFormat::Interpreted);

        // check for default output file name
        args.dest_type = "file".into();
        let config = Config::build(&args);
        assert!(config.file_path == args.input_file);
        assert!(config.context.dest == OutputDest::File(Some(PathBuf::from("output")))); // TODO make default output the name of input with fitting extension
        assert!(config.context.format == OutputFormat::Interpreted);
    }
    
    #[test]
    fn build_config()
    {
        let mut args = Args {
            caller: "caller".into(),
            input_file: PathBuf::from("input.bf"),
            output_file: "test".into(),
            dest_type: "stdout".into(),
            output_format: "interpret".into(),
        };

        let config = Config::build(&args);
        assert!(config.file_path == args.input_file);
        assert!(config.context.dest == OutputDest::Stdout);
        assert!(config.context.format == OutputFormat::Interpreted);
        
        args.dest_type = "file".into();
        args.output_format = "nasm".into();

        let config = Config::build(&args);
        assert!(config.file_path == args.input_file);
        assert!(config.context.dest == OutputDest::File(Some(PathBuf::from("test"))));
        assert!(config.context.format == OutputFormat::Assembly(AsmFlavor::NASM));
    }
}