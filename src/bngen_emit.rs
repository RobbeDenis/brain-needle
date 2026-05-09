
use crate::bngen_ctx::*;
use crate::bngen_x86_64_linux::X86X64LinuxEmitter;
use crate::bnparse::InstrNode;

pub trait Emitter
{
    fn emit_setup(&mut self);
    fn emit_arithmetic(&mut self, node: &InstrNode);
    fn emit_shift(&mut self, node: &InstrNode);
    fn emit_jump(&mut self, node: &InstrNode);
    fn emit_out(&mut self);
    fn emit_in(&mut self);
    fn emit_exit(&mut self);
    fn clone(&self) -> String;
}

pub struct EmitterFactory;

impl EmitterFactory
{
    pub fn create(_target_ctx: &TargetContext) -> Box<dyn Emitter>
    {
        return Box::new(X86X64LinuxEmitter::new());
    }
}