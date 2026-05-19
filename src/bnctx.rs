
use std::{path::PathBuf};

#[allow(dead_code)]
#[derive(Debug)]
pub enum AsmFlavor
{
    NASM,
    GAS
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Language
{
    C,
    Cpp
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TargetOS
{
    Windows,
    Linux,
    MacOS
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Architecture
{
    X86_64,
    AArch64
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum OutputFormat
{
    Assembly(AsmFlavor),
    Binary,
    Transpiled(Language),
    Interpreted
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum OutputDest
{
    File(Option<PathBuf>),
    Stdout,
    Custom
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TargetContext
{
    pub dest: OutputDest,
    pub format: OutputFormat,
    pub os: TargetOS,
    pub arch: Architecture
}

impl TargetContext
{
    pub const fn default() -> TargetContext
    {
        return TargetContext { 
            dest: OutputDest::File(Some(PathBuf::new())),
            format: OutputFormat::Assembly(AsmFlavor::NASM), 
            os: TargetOS::Linux, 
            arch: Architecture::X86_64 
        };
    }
}