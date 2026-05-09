
// using
use crate::bndest::BNDest;
use std::io::Write;

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

impl BNDest for StdoutDest
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