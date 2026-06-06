use crate::bnctx::TargetContext;
use crate::bnbytecode::bytecode_emit::BCEmitterFactory;
use crate::bnbytecode::core::*;

pub fn generate_output<TFactory: BCEmitterFactory + Default>(intermediate: Vec<u8>, target_ctx: TargetContext)
{
    let mut factory = TFactory::default();
    let mut codegen = factory.create(&target_ctx);
    
    codegen.emit_setup();
    
    let mut i: usize = 0;
    while i < intermediate.len() {
        match intermediate[i] {
            0 => { codegen.emit_add(read_u16_at(&intermediate, i)); i += size_of::<IValue>() + 1; }
            1 => { codegen.emit_sub(read_u16_at(&intermediate, i)); i += size_of::<IValue>() + 1; }
            2 => { codegen.emit_right(read_u16_at(&intermediate, i)); i += size_of::<IValue>() + 1; }
            3 => { codegen.emit_left(read_u16_at(&intermediate, i)); i += size_of::<IValue>() + 1; }
            4 => {
                i = codegen.emit_loop(read_u16_at(&intermediate, i)).unwrap_or(i + size_of::<IValue>() + 1);
            }
            5 => {
                i = codegen.emit_end_loop(read_u16_at(&intermediate, i)).unwrap_or(i + size_of::<IValue>() + 1);
            }
            6 => { codegen.emit_in(); i += 1; }
            7 => { codegen.emit_out(); i += 1; }
            // SAFETY: JUST A TEST
            // compiler hint so we can messuare the speed when match translates to a dispatch table
            _ => unsafe { std::hint::unreachable_unchecked() }
        }
    }

    codegen.emit_exit();
    codegen.finalize();
}