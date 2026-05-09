
use crate::bngen_ctx::*;
use crate::bngen_emit::*;
use crate::bnparse::InstrNode;

use std::fs;


pub fn generate_output(instr_tree: Vec<InstrNode>, target_ctx: TargetContext)
{
    let mut generator = EmitterFactory::create(&target_ctx);
    
    generator.emit_setup();
    
    for node in &instr_tree {
        match  *node {
            InstrNode::Add(__) | 
            InstrNode::Sub(__) => generator.emit_arithmetic(node),
            InstrNode::Right(__) | 
            InstrNode::Left(__) => generator.emit_shift(node),
            InstrNode::JumpIfZero(__) | 
            InstrNode::JumpIfNotZero(__) => generator.emit_jump(node),
            InstrNode::Out => generator.emit_out(),
            InstrNode::In => generator.emit_in()
        }
    }

    generator.emit_exit();


    fs::write("bn_output.asm", generator.clone()).unwrap();
}