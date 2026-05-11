
// public modules
pub mod bndest_file;
pub mod bndest_stdout;

// using
use bndest_file::FileDest;
use bndest_stdout::StdoutDest;
use crate::bnctx::TargetContext;
use crate::bnctx::OutputDest;

pub trait BNDest
{
    fn push(&mut self, data: &str);
    fn finalize(&self);
}

pub struct BNDestFactory;

impl BNDestFactory
{
    pub fn create(target_ctx: &TargetContext) -> Box<dyn BNDest>
    {
        return match &target_ctx.dest {
            OutputDest::File(option) =>Box::new(FileDest::new(option.clone())),
            OutputDest::Stdout => Box::new(StdoutDest::new())
        }
    }
}