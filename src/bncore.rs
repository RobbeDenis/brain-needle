

#[allow(unused)]
pub const MAX_PROGRAM_BYTES: usize =  1 << 15;
#[allow(unused)]
pub const BYTE_CEIL_WRAP_U8: u8 = 128;
#[allow(unused)]
pub const BYTE_CEIL_WRAP: u16 = 256;

// Maximum index allowed
pub type UIdx = u16;
// Maximum amount of identical tokens in sequence
pub type USeq = UIdx;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token
{
    Add     (USeq),
    Sub     (USeq),
    Right   (USeq),
    Left    (USeq),
    Loop,
    Back,
    Out,
    In
}

#[derive(Debug, PartialEq)]
pub enum Node
{
    Add             (USeq),
    Sub             (USeq),
    Right           (USeq),
    Left            (USeq),
    JumpIfZero      (UIdx),
    JumpIfNotZero   (UIdx),
    Out,
    In
}

///////////////////////
// App Configuration //
///////////////////////

#[allow(unused)]
pub const DEFAULT_BF_DIR: &str = "bf";
#[allow(unused)]
pub const DEFAULT_BF_FILE: &str = "hello.bf";
#[allow(unused)]
pub const DEFAULT_BF_PATH: &str = "bf\\hello.bf";
#[allow(unused)]
pub const DEFAULT_OUT_DIR: &str = "output";
#[allow(unused)]
pub const DEFAULT_OUT_FILE: &str = "hello.asm";

#[inline]
#[must_use = "The reader must be consumed"]
pub fn create_bnreader<P: AsRef<std::path::Path>>(path: P) -> std::io::BufReader<std::fs::File> {
    let file = std::fs::File::open(path).unwrap();
    std::io::BufReader::new(file)
}// TODO create as a type???