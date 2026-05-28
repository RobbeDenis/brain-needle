
// public modules
pub mod dest_file;
pub mod dest_stdout;

// using
use dest_file::FileDest;
use dest_stdout::StdoutDest;
use crate::bnctx::TargetContext;
use crate::bnctx::OutputDest;

pub trait BNDest
{
    fn push(&mut self, data: &[u8]);
    fn finalize(&self);
}

pub struct BNDestFactory;

impl BNDestFactory
{
    pub fn create(target_ctx: &TargetContext) -> Box<dyn BNDest>
    {
        return match &target_ctx.dest {
            OutputDest::File(option) => Box::new(FileDest::new(option.clone())),
            OutputDest::Stdout => Box::new(StdoutDest::new()),
            OutputDest::Custom => panic!("Custom destinaiton case is required to be handled by implementing BNEmitterFactory::create_dest")
        }
    }
}