
#[allow(dead_code)]
pub(crate) enum AsmFlavor
{
    NASM,
    GAS
}

#[allow(dead_code)]
pub(crate) enum Language
{
    C,
    Cpp
}

#[allow(dead_code)]
pub(crate) enum TargetOS
{
    Windows,
    Linux,
    MacOS
}

#[allow(dead_code)]
pub(crate) enum Architecture
{
    X86_64,
    AArch64
}

#[allow(dead_code)]
pub(crate) enum Output
{
    Assembly(AsmFlavor),
    Binary,
    Transpiled(Language)
}

#[allow(dead_code)]
pub(crate) struct TargetContext
{
    pub os: TargetOS,
    pub arch: Architecture,
    pub out: Output
}