
// using
use crate::bndest::BNDest;
use std::fs::File;
use std::io::Write;

pub struct FileDest
{
    output: String
}

impl FileDest
{
    pub const fn new() -> FileDest
    {
        return FileDest{ output: String::new() };
    }
}

impl BNDest for FileDest
{
    fn push(&mut self, data: &str)
    {
        self.output += data;
    }

    fn finalize(&self)
    {
        let mut file = File::create("output\\hello.asm").unwrap();
        file.write_all(self.output.as_bytes()).unwrap();
    }
}