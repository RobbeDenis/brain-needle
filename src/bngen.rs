
// using
use crate::bnparse::Node;
use crate::bnctx::TargetContext;
use crate::bnemit::*;

pub fn generate_output<TFactory: BNEmitterFactory + Default>(instr_tree: Vec<Node>, target_ctx: TargetContext)
{
    let mut factory = TFactory::default();
    let mut codegen = factory.create(&target_ctx);
    
    codegen.emit_setup();
    
    for node in &instr_tree {
        match  *node {
            Node::Add(__) | 
            Node::Sub(__) => codegen.emit_arithmetic(node),
            Node::Right(__) | 
            Node::Left(__) => codegen.emit_shift(node),
            Node::JumpIfZero(__) | 
            Node::JumpIfNotZero(__) => codegen.emit_jump(node),
            Node::Out => codegen.emit_out(),
            Node::In => codegen.emit_in()
        }
    }

    codegen.emit_exit();
    codegen.finalize();
}