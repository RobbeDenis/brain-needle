
// public modules
pub mod bnemit_x86_64_linux;
pub mod bnemit_interpreted;

// emitters
use bnemit_x86_64_linux::X86X64LinuxEmitter;
use bnemit_interpreted::InterpretedEmitter;

use crate::bndest::BNDestFactory;
use crate::bndest::BNDest;
use crate::bnintermediate::BNNode;
use crate::bnctx::{OutputFormat, TargetContext};

pub trait BNEmitter
{
    fn emit_setup(&mut self);
    fn emit_arithmetic(&mut self, node: &BNNode);
    fn emit_shift(&mut self, node: &BNNode);
    fn emit_jump(&mut self, node: &BNNode) -> Option<usize>;
    fn emit_out(&mut self);
    fn emit_in(&mut self);
    fn emit_exit(&mut self);
    fn finalize(&mut self);
}

pub trait BNEmitterFactory
{
    fn create(&mut self, target_ctx: &TargetContext) -> Box<dyn BNEmitter>
    {
        return match &target_ctx.format {
            OutputFormat::Assembly(_value) => Box::new(X86X64LinuxEmitter::new(self.create_dest(&target_ctx))),
            OutputFormat::Interpreted => Box::new(InterpretedEmitter::new(self.create_dest(&target_ctx))),
            _ => Box::new(InterpretedEmitter::new(self.create_dest(&target_ctx)))
        }
    }

    /// INTERNAL USE ONLY: Used by `create` to hook up the destination.
    /// Consider using BNDestFactory instead or implement create_dest to handle custom destinations.
    fn create_dest(&mut self, target_ctx: &TargetContext) -> Box<dyn BNDest>
    {
        return BNDestFactory::create(&target_ctx);
    }
}

#[derive(Default)]
pub struct BNEmitterFactoryDefault;
impl BNEmitterFactory for BNEmitterFactoryDefault
{
}