
// using
use crate::bndest::BNDest;
use std::io::Write;

pub struct StdoutDest
{
    // output: String
}

impl StdoutDest
{
    pub const fn new() -> StdoutDest
    {
        return StdoutDest{ };
        // return StdoutDest{ output: String::new() };
    }
}

impl BNDest for StdoutDest
{
    fn push(&mut self, data: &[u8])
    {
        let mut stdout = std::io::stdout().lock();
        stdout.write_all(data).unwrap();
        stdout.flush().unwrap();
    }

    fn finalize(&self)
    {
        // let stdout = std::io::stdout();
        // let mut handle = stdout.lock();
        // handle.write_all(self.output.as_bytes()).unwrap();
    }
}