
use std::error::Error;
use crate::bnlex::Token;

#[derive(Debug, PartialEq)]
pub enum InstrNode
{
    Add             (u8),   // value [0-255]
    Sub             (u8),
    Right           (u8),
    Left            (u8),
    JumpIfZero      (u16),  // index [0-30'000]
    JumpIfNotZero   (u16),
    Out,
    In
}

pub fn create_flat_instr_tree_from_tokens(tokens: Vec<Token>) -> Result<Vec<InstrNode>, Box<dyn Error>>
{
    let mut instr_tree: Vec<InstrNode> = Vec::new();

    for token in tokens {
        match token {
            Token::Add(value) => instr_tree.push(InstrNode::Add(value)),
            Token::Sub(value) => instr_tree.push(InstrNode::Sub(value)),
            Token::Right(value) => instr_tree.push(InstrNode::Right(value)),
            Token::Left(value) => instr_tree.push(InstrNode::Left(value)),
            Token::Loop => instr_tree.push(InstrNode::Out), // temp
            Token::Back => instr_tree.push(InstrNode::Out), // temp
            Token::Out => instr_tree.push(InstrNode::Out),
            Token::In => instr_tree.push(InstrNode::In)
        }
    }

    return Ok(instr_tree);
}