
use crate::bngen_ctx::*;
use crate::bngen_emit::*;
use crate::bnparse::InstrNode;

pub fn generate_output(instr_tree: Vec<InstrNode>, target_ctx: TargetContext)
{
    let mut codegen = BNEmitterFactory::create(&target_ctx);
    
    codegen.emit_setup();
    
    for node in &instr_tree {
        match  *node {
            InstrNode::Add(__) | 
            InstrNode::Sub(__) => codegen.emit_arithmetic(node),
            InstrNode::Right(__) | 
            InstrNode::Left(__) => codegen.emit_shift(node),
            InstrNode::JumpIfZero(__) | 
            InstrNode::JumpIfNotZero(__) => codegen.emit_jump(node),
            InstrNode::Out => codegen.emit_out(),
            InstrNode::In => codegen.emit_in()
        }
    }

    codegen.emit_exit();
    codegen.finalize();
}