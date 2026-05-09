
use crate::bngen_ctx::*;
use crate::bngen_dest::*;
use crate::bngen_x86_64_linux::X86X64LinuxEmitter;
use crate::bnparse::InstrNode;

pub trait BNEmitter
{
    fn emit_setup(&mut self);
    fn emit_arithmetic(&mut self, node: &InstrNode);
    fn emit_shift(&mut self, node: &InstrNode);
    fn emit_jump(&mut self, node: &InstrNode);
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
        return Box::new(X86X64LinuxEmitter::new(BNEmitDestFactory::create(&target_ctx)));
    }
}