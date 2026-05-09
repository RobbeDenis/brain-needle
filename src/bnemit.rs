
// public modules
pub mod bnemit_x86_64_linux;

// using
use bnemit_x86_64_linux::X86X64LinuxEmitter;
use crate::bndest::BNDestFactory;
use crate::bnparse::Node;
use crate::bnctx::TargetContext;

pub trait BNEmitter
{
    fn emit_setup(&mut self);
    fn emit_arithmetic(&mut self, node: &Node);
    fn emit_shift(&mut self, node: &Node);
    fn emit_jump(&mut self, node: &Node);
    fn emit_out(&mut self);
    fn emit_in(&mut self);
    fn emit_exit(&mut self);
    fn finalize(&mut self);
}

pub struct BNEmitterFactory;

// maybe consider adding factory rules
// that way adding new emitters can be fully modular
// otherwise the factory will have te be changed for every new emitter
impl BNEmitterFactory
{
    pub fn create(target_ctx: &TargetContext) -> Box<dyn BNEmitter>
    {
        return Box::new(X86X64LinuxEmitter::new(BNDestFactory::create(&target_ctx)));
    }
}