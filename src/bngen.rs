
// using
use crate::bncore::Node;
use crate::bnctx::TargetContext;
use crate::bnemit::*;

pub fn generate_output<TFactory: BNEmitterFactory + Default>(instr_tree: Vec<Node>, target_ctx: TargetContext)
{
    let mut factory = TFactory::default();
    let mut codegen = factory.create(&target_ctx);
    
    codegen.emit_setup();
    
    let mut i: usize = 0;
    while i < instr_tree.len() {
    let node = &instr_tree[i];
        match  *node {
            Node::Add(__) | 
            Node::Sub(__) => codegen.emit_arithmetic(node),
            Node::Right(__) | 
            Node::Left(__) => codegen.emit_shift(node),
            Node::JumpIfZero(__) | 
            Node::JumpIfNotZero(__) => {
                if let Some(target) = codegen.emit_jump(node) {
                    i = target;
                }
            },
            Node::Out => codegen.emit_out(),
            Node::In => codegen.emit_in()
        }
        i += 1;
    }

    codegen.emit_exit();
    codegen.finalize();
}