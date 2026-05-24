
use crate::bnctx::TargetContext;
use crate::bnemit::*;
use crate::bnintrep::BNNode;

pub fn generate_output<TFactory: BNEmitterFactory + Default>(intermediate: Vec<BNNode>, target_ctx: TargetContext)
{
    let mut factory = TFactory::default();
    let mut codegen = factory.create(&target_ctx);
    
    codegen.emit_setup();
    
    let mut i: usize = 0;
    while i < intermediate.len() {
    let node = &intermediate[i];
        match  *node {
            BNNode::Add(_) | 
            BNNode::Sub(_) => codegen.emit_arithmetic(node),
            BNNode::Right(_) | 
            BNNode::Left(_) => codegen.emit_shift(node),
            BNNode::Loop(_) | 
            BNNode::EndLoop(_) => {
                if let Some(target) = codegen.emit_jump(node) {
                    i = target;
                }
            },
            BNNode::Out(_) => codegen.emit_out(),
            BNNode::In(_) => codegen.emit_in(),
            BNNode::Sentinel(_) => panic!("Sentinel node should not be in final IR")
        }
        i += 1;
    }

    codegen.emit_exit();
    codegen.finalize();
}