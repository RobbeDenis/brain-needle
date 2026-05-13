
use std::path::PathBuf;

#[allow(dead_code)]
pub enum AsmFlavor
{
    NASM,
    GAS
}

#[allow(dead_code)]
pub enum Language
{
    C,
    Cpp
}

#[allow(dead_code)]
pub enum TargetOS
{
    Windows,
    Linux,
    MacOS
}

#[allow(dead_code)]
pub enum Architecture
{
    X86_64,
    AArch64
}

#[allow(dead_code)]
pub enum OutputFormat
{
    Assembly(AsmFlavor),
    Binary,
    Transpiled(Language),
    Interpreted
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum OutputDest
{
    File(Option<PathBuf>),
    Stdout,
    Custom
}

#[allow(dead_code)]
pub struct TargetContext
{
    pub dest: OutputDest,
    pub format: OutputFormat,
    pub os: TargetOS,
    pub arch: Architecture
}