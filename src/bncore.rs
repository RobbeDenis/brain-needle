

pub const MAX_PROGRAM_BYTES: usize = 30000;
pub const BYTE_CEIL_WRAP: u8 = 255;

// Maximum index allowed
pub type UIdx = u16;

// Maximum amount of identical tokens in sequence
pub type USeq = UIdx;

// Configuration
pub struct Config 
{
    pub file: std::path::PathBuf
}

impl Config
{
    pub fn build(args: &[String]) -> Result<Config, &'static str> 
    {
        let mut file = std::path::PathBuf::from("src_bf\\hello.bf");

        if args.len() > 1 {
            file = std::path::PathBuf::from("src_bf");
            file.push(&args[1]);
        }
    
        return Ok(Config{ file });
    }
}