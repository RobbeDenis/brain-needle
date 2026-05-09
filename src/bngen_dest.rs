
use crate::bngen_ctx::*;
use crate::bngen_stdout::StdoutDest;

pub trait BNEmitDest
{
    fn push(&mut self, data: &str);
    fn finalize(&self);
}

pub struct BNEmitDestFactory;

impl BNEmitDestFactory
{
    pub fn create(_target_ctx: &TargetContext) -> Box<dyn BNEmitDest>
    {
        return Box::new(StdoutDest::new());
    }
}