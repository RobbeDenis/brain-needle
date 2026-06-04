
// using
use crate::bndest::BNDest;
use std::io::Write;

pub struct StdoutDest
{
}

impl StdoutDest
{
    pub const fn new() -> StdoutDest
    {
        return StdoutDest{ };
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

    }
}