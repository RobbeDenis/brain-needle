
use std::{env, path::PathBuf};
use crate::bnctx::*;

#[derive(Debug)]
pub struct Args
{
    pub caller: String,
    pub input_path: PathBuf,
    pub dest_flag: String
}

impl Args
{
    pub fn parse_from_env() -> Result<Args, &'static str>
    {
        return Self::parse_from_args(env::args().collect());
    }

    pub fn parse_from_args(mut args: Vec<String>) -> Result<Args, &'static str>
    {
        if args.is_empty() {
            return Err("Arguments does not contain a caller???");
        }
        let caller = args.remove(0);

        if args.is_empty() {
            return Err("Usage: bn <input_file>");
        }

        let input_path = PathBuf::from(args.remove(0).trim());
        let dest_flag = args.pop().unwrap_or_else(|| "default".to_string());

        return Ok(Args { 
            caller: caller,
            input_path: input_path, 
            dest_flag: dest_flag 
        });
    }
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
        let path = args.input_path.clone();
        let mut ctx = TargetContext::default();

        match args.dest_flag.as_str() {
            "stdout" => ctx.dest = OutputDest::Stdout,
            "file" => ctx.dest = OutputDest::File(Some("output".into())),
            _ => ctx.dest = OutputDest::Custom
        }

        return Ok(Config { file_path: path, context: ctx });
    }
}

#[cfg(test)]
mod config_tests
{
    use super::*;

    #[test]
    fn parse_arguments()
    {
        let args:Vec<String>= vec!["program", "input.bf", "stdout"]
            .into_iter()
            .map(String::from)
            .collect();

        let args = Args::parse_from_args(args).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert!(args.caller == "program");
        assert!(args.input_path == PathBuf::from("input.bf"));
        assert!(args.dest_flag == "stdout");
    }

        #[test]
    fn build_config()
    {
        let args = Args { 
            caller: "program".into(), 
            input_path: PathBuf::from("input.bf"), 
            dest_flag: "stdout".into() 
        };

        let config = Config::build(&args).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert!(config.file_path == args.input_path);
        assert!(config.context.dest == OutputDest::Stdout);
    }
}