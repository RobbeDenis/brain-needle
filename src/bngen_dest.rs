
use crate::bngen_ctx::*;
use crate::bngen_file::FileDest;
use crate::bngen_stdout::StdoutDest;

pub trait BNEmitDest
{
    fn push(&mut self, data: &str);
    fn finalize(&self);
}

pub struct BNEmitDestFactory;

impl BNEmitDestFactory
{
    pub fn create(target_ctx: &TargetContext) -> Box<dyn BNEmitDest>
    {
        if target_ctx.dest == OutputDest::File(None) {
            return Box::new(FileDest::new());
        } else {
            return Box::new(StdoutDest::new());
        }
    }
}