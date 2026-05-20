
use std::{env, path::PathBuf};
use crate::bnctx::*;

//////////////////////////
/////// Arguments ////////
//////////////////////////

#[derive(Debug, Default)]
pub struct Args
{
    pub caller: String,
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub output_format: String,
    pub dest_type: String,
    pub target_arch: String,
    pub target_os: String,
}

impl Args
{
    // Argument parsing is divided into parse_from_env and parse_from_args
    // parse_from_env is the main method of parsing the given arguments,
    // while parse_from_args is ment for specific test cases
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
            eprintln!("Argument parser error: missing first argument\nUsage: [INPUT | -h | --help] [OPTIONS]");
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
                            eprintln!("Argument parser error: The option '{}' requires a destination type", arg);
                            std::process::exit(1);
            }}}
                "-o" | "--out" => {
                    match args_iter.next() {
                        Some(value) => parsed_args.output_file = value.into(),
                        _ => {
                            eprintln!("Argument parser error: The option '{}' requires an output file path", arg);
                            std::process::exit(1);
            }}}
                "-f" | "--fmt" => {
                    match args_iter.next() {
                        Some(value) => parsed_args.output_format = value.into(),
                        _ => {
                            eprintln!("Argument parser error: The option '{}' requires an output format", arg);
                            std::process::exit(1);
            }}}
                "-a" | "--arch" => {
                    match args_iter.next() {
                        Some(value) => parsed_args.target_arch = value.into(),
                        _ => {
                            eprintln!("Argument parser error: The option '{}' requires a specified architecture", arg);
                            std::process::exit(1);
            }}}
                "-t" | "--os" => {
                    match args_iter.next() {
                        Some(value) => parsed_args.target_os = value.into(),
                        _ => {
                            eprintln!("Argument parser error: The option '{}' requires a specified operating system", arg);
                            std::process::exit(1);
            }}}
                _ => { 
                    eprintln!("Argument parser error: Unknown option: {}", arg);
                    std::process::exit(1);
        }}}

        return parsed_args;
    }
}

fn print_help() 
{
    println!("\nUsage: [INPUT | -h | --help] [OPTIONS]");
    println!("\nOptions:");

    println!("  -o, --out  <OUTPUT>\t\t  Specify output file path [default: INPUT.*]");

    println!("  -d, --dest <DEST> \t\t  Specify destination output type [default: stdout]");
    println!("              stdout \t\t  Writes using stdout");
    println!("              file   \t\t  Writes to a file");

    println!("  -f, --fmt  <FORMAT>\t\t  Specify output format [default: interpret]");
    println!("              interpret\t\t  Directly interprets and writes brainfuck to the output");
    println!("              nasm    \t\t  Compiles brainfuck to NASM");

    println!("  -a, --arch <ARCH>\t\t  Specify target architecture [default: x86_64]");
    println!("              x86-64\t\t  Uses the x86-64 AMD/Intel instruction set");

    println!("  -t, --os   <OS>\t\t  Specify target operating system [default: linux]");
    println!("              linux\t\t  Uses linux system calls");

    println!("");
}

//////////////////////////
///////// Config /////////
//////////////////////////

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
                    ctx.dest = OutputDest::File(Some(path.with_extension(create_extension(&ctx.format))));
                } else {
                    ctx.dest = OutputDest::File(Some(args.output_file.clone()))
            }},
            _ => ctx.dest = OutputDest::Stdout
        }

        match args.target_arch.as_str() {
            "x86-64" => ctx.arch = Architecture::X86_64,
            _ => ctx.arch = Architecture::X86_64
        }

        match args.target_os.as_str() {
            "linux" => ctx.os = TargetOS::Linux,
            _ => ctx.os = TargetOS::Linux
        }

        return Config { file_path: path, context: ctx };
    }
}

/// create extension based on [`OutputFormat`]
fn create_extension(format: &OutputFormat) -> &str
{
    match format {
        OutputFormat::Interpreted => "txt",
        OutputFormat::Assembly(AsmFlavor::NASM) => "asm",
        _ => ""
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
    fn parse_short_options()
    {
        let args:Vec<String>= vec![
            "caller", "input.bf", "-o", "output", "-d", "stdout", "-f", 
            "nasm", "-a", "x86-64", "-t", "linux"]
            .into_iter()
            .map(String::from)
            .collect();

        let args = Args::parse_from_args(args);
        assert!(args.caller == "caller");
        assert!(args.input_file == PathBuf::from("input.bf"));
        assert!(args.dest_type == "stdout");
        assert!(args.output_format == "nasm");
        assert!(args.target_arch == "x86-64");
        assert!(args.target_os == "linux");
    }
    
    #[test]
    fn parse_long_options()
    {
        let args:Vec<String>= vec![
            "caller", "input.bf", "--out", "output", "--dest", "stdout", 
            "--fmt", "nasm", "--arch", "x86-64", "--os", "linux"]
        .into_iter()
        .map(String::from)
        .collect();
    
        let args = Args::parse_from_args(args);
        assert!(args.caller == "caller");
        assert!(args.input_file == PathBuf::from("input.bf"));
        assert!(args.dest_type == "stdout");
        assert!(args.output_format == "nasm");
        assert!(args.target_arch == "x86-64");
        assert!(args.target_os == "linux");
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
        assert!(config.context.arch == Architecture::X86_64);
        assert!(config.context.os == TargetOS::Linux);

        // check for default output file name
        args.dest_type = "file".into();
        let config = Config::build(&args);
        assert!(config.file_path == args.input_file);
        let expected = OutputDest::File(Some(PathBuf::from("input.txt")));
        assert!(config.context.dest == expected, "OutputDest mismatch\nExpected: {:?}\nActual:   {:?}\n", expected, config.context.dest);
        assert!(config.context.format == OutputFormat::Interpreted);
        assert!(config.context.arch == Architecture::X86_64);
        assert!(config.context.os == TargetOS::Linux);
    }
    
    #[test]
    fn build_config()
    {
        let mut args = Args {
            caller: "caller".into(),
            input_file: PathBuf::from("input.bf"),
            output_file: "test".into(),
            output_format: "interpret".into(),
            dest_type: "stdout".into(),
            target_arch: "x86-64".into(),
            target_os: "linux".into(),
        };

        let config = Config::build(&args);
        assert!(config.file_path == args.input_file);
        assert!(config.context.dest == OutputDest::Stdout);
        assert!(config.context.format == OutputFormat::Interpreted);
        assert!(config.context.arch == Architecture::X86_64);
        assert!(config.context.os == TargetOS::Linux);
        
        args.dest_type = "file".into();
        args.output_format = "nasm".into();

        let config = Config::build(&args);
        assert!(config.file_path == args.input_file);
        assert!(config.context.dest == OutputDest::File(Some(PathBuf::from("test"))));
        assert!(config.context.format == OutputFormat::Assembly(AsmFlavor::NASM));
        assert!(config.context.arch == Architecture::X86_64);
        assert!(config.context.os == TargetOS::Linux);
    }
}