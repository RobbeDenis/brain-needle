
use crate::bncore::UIdx;
use crate::bncore::Node;
use crate::bncore::Token;
use std::error::Error;

pub fn create_flat_instr_tree_from_tokens(tokens: Vec<Token>) -> Result<Vec<Node>, Box<dyn Error>>
{
    let mut instr_tree: Vec<Node> = Vec::new();
    let mut loop_start_queue: Vec<usize> = Vec::new();

    if tokens.len() > UIdx::MAX as usize {
        return Err(format!("program too large for {}-bit addressing", UIdx::BITS).into());
    }

    for (i, token) in tokens.iter().enumerate() {
        match *token {
            Token::Add(value) => instr_tree.push(Node::Add(value)),
            Token::Sub(value) => instr_tree.push(Node::Sub(value)),
            Token::Right(value) => {
                instr_tree.push(Node::Right(value))
            },
            Token::Left(value) => {
                instr_tree.push(Node::Left(value))
            },
            Token::Out => instr_tree.push(Node::Out),
            Token::In => instr_tree.push(Node::In),
            Token::Loop => {
                instr_tree.push(Node::JumpIfZero(0));
                loop_start_queue.push(i);
            },
            Token::Back => {
                let start_idx = loop_start_queue.pop().ok_or(
                    format!("loop token mismatch: found end \']\' without a matching start \'[\' at index {}", i)
                )?;

                instr_tree.push(Node::JumpIfNotZero(start_idx as UIdx));
                instr_tree[start_idx] = Node::JumpIfZero(i as UIdx);
            }
        }
    }

    if !loop_start_queue.is_empty() {
        return Err(format!("loop token mismatch: {} start \'[\' without a matching end \']\'", loop_start_queue.len()).into());
    }

    return Ok(instr_tree);
}




/////////////////////////////////////////////////////////




#[cfg(test)]
mod parser_tests 
{
    use super::*;
    use crate::hello_world_tokens;
    use crate::hello_world_instr_tree;

    #[test]
    fn print()
    {
        let tokens: Vec<Token> = hello_world_tokens!();

        let instr_tree = create_flat_instr_tree_from_tokens(tokens).unwrap();

        assert!(!instr_tree.is_empty(), "[Print Failed]\nInstruction tree should not be empty");

        println!("\n[Print Instr Tree]");
        for node in &instr_tree {
            print!("{:?} ", node);
        }
        println!();
    }

    #[test]
    fn hello_world_from_tokens()
    {
        let tokens: Vec<Token> = hello_world_tokens!();
        
        let tokens_clone = tokens.clone();

        let instr_tree = create_flat_instr_tree_from_tokens(tokens).unwrap();

        assert!(
            tokens_clone.len() == instr_tree.len(),
            "[Compare Length Failed]\nLength mismatch, instr tree length should be same as tokens\nTokens: {:?}\nInstr Tree:   {:?}", 
            tokens_clone.len(),
            instr_tree.len()
        );

        let expected_instr_tree: Vec<Node> = hello_world_instr_tree!();

        for (i, node) in instr_tree.iter().enumerate() {
            assert!(
                node == &expected_instr_tree[i],
                "[Compare Node Failed]\nNode mismatch at index {}\nExpected node: {:?}\nActual node:   {:?}", 
                i,
                &expected_instr_tree[i],
                node
            );
        }
    }
}