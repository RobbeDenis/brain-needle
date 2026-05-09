
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
        if target_ctx.dest == OutputDest::File(None) {
            return Box::new(FileDest::new());
        } else {
            return Box::new(StdoutDest::new());
        }
    }
}