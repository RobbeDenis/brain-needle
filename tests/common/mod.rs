
use brain_needle::*;
use bnctx::*;
use bnemit::BNEmitterFactory;
use bndest::BNDest;
use bndest::BNDestFactory;


pub struct TBNEmitterFactory
{
    pub was_create_dest_called: bool
}

impl BNEmitterFactory for TBNEmitterFactory
{
    fn create_dest(&mut self, target_ctx: &TargetContext) -> Box<dyn BNDest>
    {
        self.was_create_dest_called = true;
        return BNDestFactory::create(&target_ctx);
    }
}