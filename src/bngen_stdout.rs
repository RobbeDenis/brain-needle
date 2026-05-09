
use std::io::Write;

use crate::bngen_dest::BNEmitDest;

pub struct StdoutDest
{
    output: String
}

impl StdoutDest
{
    pub const fn new() -> StdoutDest
    {
        return StdoutDest{ output: String::new() };
    }
}

impl BNEmitDest for StdoutDest
{
    fn push(&mut self, data: &str)
    {
        self.output += data;
    }

    fn finalize(&self)
    {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(self.output.as_bytes()).unwrap();
    }
}