
// emitters
use crate::istream::emit_interpreted::InterpretedEmitter;

use crate::bndest::BNDestFactory;
use crate::bndest::BNDest;
use crate::bnctx::{OutputFormat, TargetContext};

pub trait EmitterTrait
{
    fn emit_setup(&mut self);
    fn emit_add(&mut self, value: u16);
    fn emit_sub(&mut self, value: u16);
    fn emit_right(&mut self, value: u16);
    fn emit_left(&mut self, value: u16);
    fn emit_loop(&mut self, value: u16) -> Option<usize>;
    fn emit_end_loop(&mut self, value: u16) -> Option<usize>;
    fn emit_out(&mut self);
    fn emit_in(&mut self);
    fn emit_exit(&mut self);
    fn finalize(&mut self);
}

pub trait EmitterFactory
{
    fn create(&mut self, target_ctx: &TargetContext) -> Box<dyn EmitterTrait>
    {
        return match &target_ctx.format {
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
pub struct BCEmitterFactoryDefault;
impl EmitterFactory for BCEmitterFactoryDefault
{
}