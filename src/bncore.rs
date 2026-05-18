

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
pub const DEFAULT_BF_DIR: &str = "src-bf";
#[allow(unused)]
pub const DEFAULT_BF_FILE: &str = "hello.bf";
#[allow(unused)]
pub const DEFAULT_BF_PATH: &str = "src-bf\\hello.bf";
#[allow(unused)]
pub const DEFAULT_OUT_DIR: &str = "output";
#[allow(unused)]
pub const DEFAULT_OUT_FILE: &str = "hello.asm";

pub struct Config 
{
    pub file_path: std::path::PathBuf
}

impl Config
{
    pub fn build(args: &[String]) -> Result<Config, &'static str> 
    {
        let mut path = std::path::PathBuf::from(DEFAULT_BF_DIR);

        if args.len() > 1 {
            path.push(args[1].trim());
        } else {
            path.push(DEFAULT_BF_FILE);
        }

        return Ok(Config { file_path: path });
    }
}