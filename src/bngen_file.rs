
use std::{fs::File, io::Write};

use crate::bngen_dest::BNEmitDest;

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

impl BNEmitDest for FileDest
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