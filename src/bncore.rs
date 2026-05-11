

#[allow(unused)]
pub const MAX_PROGRAM_BYTES: usize = 30000;
#[allow(unused)]
pub const BYTE_CEIL_WRAP: u8 = 255;

// Maximum index allowed
pub type UIdx = u16;
// Maximum amount of identical tokens in sequence
pub type USeq = UIdx;

///////////////////////
// App Configuration //
///////////////////////

#[allow(unused)]
pub const DEFAULT_BF_DIR: &str = "src-bf";
#[allow(unused)]
pub const DEFAULT_BF_FILE: &str = "hello.bf";
#[allow(unused)]
pub const DEFAULT_OUT_DIR: &str = "output";
#[allow(unused)]
pub const DEFAULT_OUT_FILE: &str = "hello.asm";

pub struct Config 
{
    pub file: std::path::PathBuf
}

impl Config
{
    pub fn build(args: &[String]) -> Result<Config, &'static str> 
    {
        let mut file = std::path::PathBuf::from(DEFAULT_BF_DIR);

        if args.len() > 1 {
            file.push(args[1].trim());
        } else {
            file.push(DEFAULT_BF_FILE);
        }

        return Ok(Config { file: file });
    }
}