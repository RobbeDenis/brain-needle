
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
    let mut loop_start_queue: Vec<usize> = Vec::new();

    if tokens.len() > u16::MAX as usize {
        return Err("program too large for 16-bit addressing".into());
    }

    for (i, token) in tokens.iter().enumerate() {
        match *token {
            Token::Add(value) => instr_tree.push(InstrNode::Add(value)),
            Token::Sub(value) => instr_tree.push(InstrNode::Sub(value)),
            Token::Right(value) => instr_tree.push(InstrNode::Right(value)),
            Token::Left(value) => instr_tree.push(InstrNode::Left(value)),
            Token::Out => instr_tree.push(InstrNode::Out),
            Token::In => instr_tree.push(InstrNode::In),
            Token::Loop => {
                instr_tree.push(InstrNode::JumpIfZero(0));
                loop_start_queue.push(i);
            },
            Token::Back => {
                let start_idx = loop_start_queue.pop().ok_or(
                    format!("loop token mismatch: found end \']\' without a matching start \'[\' at index {}", i)
                )?;

                instr_tree.push(InstrNode::JumpIfNotZero(start_idx as u16));
                instr_tree[start_idx] = InstrNode::JumpIfZero(i as u16);
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

    // to enable print pass --nocapture
    #[test]
    fn print()
    {
        use Token::*;

        let tokens: Vec<Token> = vec![                                                       // index
                Add(10),    Loop,       Right(1),   Add(1),     Right(1),   // 4
                Add(3),     Right(1),   Add(7),     Right(1),   Add(10),    // 9
                Left(4),    Sub(1),     Back,       Right(3),   Add(2),     // 14
                Out,        Right(1),   Add(1),     Out,        Add(7),     // 19
                Out,        Out,        Add(3),     Out,        Left(2),    // 24
                Add(2),     Out,        Right(1),   Add(15),    Out,        // 29
                Right(1),   Out,        Add(3),     Out,        Sub(6),     // 34
                Out,        Sub(8),     Out                                 // 37
            ];

        let instr_tree = create_flat_instr_tree_from_tokens(tokens).unwrap();

        assert!(!instr_tree.is_empty(), "[Print Failed]\nInstruction tree should not be empty");

        println!("\n[Print Instr Tree]");
        for node in &instr_tree {
            print!("{:?} ", node);
        }
        println!();
    }

    #[test]
    fn hello_world_from_file()
    {
        use Token::*;

        let tokens: Vec<Token> = vec![                                  // index
            Add(10),    Loop,       Right(1),   Add(1),     Right(1),   // 4
            Add(3),     Right(1),   Add(7),     Right(1),   Add(10),    // 9
            Left(4),    Sub(1),     Back,       Right(3),   Add(2),     // 14
            Out,        Right(1),   Add(1),     Out,        Add(7),     // 19
            Out,        Out,        Add(3),     Out,        Left(2),    // 24
            Add(2),     Out,        Right(1),   Add(15),    Out,        // 29
            Right(1),   Out,        Add(3),     Out,        Sub(6),     // 34
            Out,        Sub(8),     Out                                 // 37
        ];
        
        let tokens_clone = tokens.clone();

        let instr_tree = create_flat_instr_tree_from_tokens(tokens).unwrap();

        assert!(
            tokens_clone.len() == instr_tree.len(),
            "[Compare Length Failed]\nLength mismatch, instr tree length should be same as tokens\nTokens: {:?}\nInstr Tree:   {:?}", 
            tokens_clone.len(),
            instr_tree.len()
        );

        {   // scope for to prevent clash between InstrNode and Token (should just change the names)
            use InstrNode::*;
            let expected_instr_tree: Vec<InstrNode> = vec![                             // index
                Add(10),    JumpIfZero(12), Right(1),           Add(1),     Right(1),   // 4
                Add(3),     Right(1),       Add(7),             Right(1),   Add(10),    // 9
                Left(4),    Sub(1),         JumpIfNotZero(1),   Right(3),   Add(2),     // 14
                Out,        Right(1),       Add(1),             Out,        Add(7),     // 19
                Out,        Out,            Add(3),             Out,        Left(2),    // 24
                Add(2),     Out,            Right(1),           Add(15),    Out,        // 29
                Right(1),   Out,            Add(3),             Out,        Sub(6),     // 34
                Out,        Sub(8),         Out                                         // 37
            ];

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
}