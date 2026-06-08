use crate::bnctx::TargetContext;
use crate::bstream::emit::EmitterFactory;
use crate::bstream::emit::EmitterTrait;
use crate::bstream::*;

#[inline(never)]
pub fn generate_output<TFactory: EmitterFactory + Default>(intermediate: Vec<u8>, target_ctx: TargetContext)
{
    // let mut factory = TFactory::default();
    // let mut codegen = factory.create(&target_ctx);
    let mut codegen = crate::bstream::emit_interpreted::InterpretedEmitter::new(crate::bndest::BNDestFactory::create(&target_ctx));
    
    codegen.emit_setup();
    
    let mut i: usize = 0;
    while i < intermediate.len() {
        let id = intermediate[i];
        match id {
            IAdd::ID => { codegen.emit_add(IAdd::read(&intermediate, i)); i += IAdd::SIZE; }
            ISub::ID => { codegen.emit_sub(ISub::read(&intermediate, i)); i += ISub::SIZE; }
            IRight::ID => { codegen.emit_right(IRight::read(&intermediate, i)); i += IRight::SIZE; }
            ILeft::ID => { codegen.emit_left(ILeft::read(&intermediate, i)); i += ILeft::SIZE; }
            ILoop::ID => {
                if let Some(target) = codegen.emit_loop(ILoop::read(&intermediate, i)) {
                    i = target;
                }
                i += ILoop::SIZE;
            }
            IEndLoop::ID => {
                if let Some(target) = codegen.emit_end_loop(IEndLoop::read(&intermediate, i)) {
                    i = target;
                }
                i += IEndLoop::SIZE;
            }
            IIn::ID => { codegen.emit_in(); i += IIn::SIZE; }
            IOut::ID => { codegen.emit_out(); i += IOut::SIZE; }
            // SAFETY: JUST A TEST
            // compiler hint so we can messuare the speed when match translates to a dispatch table
            _ => unsafe { std::hint::unreachable_unchecked() }
        }
    }

    codegen.emit_exit();
    codegen.finalize();
}