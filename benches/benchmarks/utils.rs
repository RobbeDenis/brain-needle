#[allow(unused)]
#[inline]
#[must_use]
pub fn multiply_contents_into_bytes<P: AsRef<std::path::Path>>(path: P, n: usize) -> Vec<u8> {
    let contents = std::fs::read_to_string(path).unwrap();
    return contents.repeat(n).into_bytes();
}

#[allow(unused)]
#[inline]
#[must_use]
pub fn std_lin_int_ctx() -> bn::bnctx::TargetContext
{
    use bn::bnctx::*;
    TargetContext { 
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64,
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Stdout
    }
}