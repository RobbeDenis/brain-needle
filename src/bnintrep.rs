
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::bnerror::BNError;

////////////////////////////
//////// GENERATE IR ///////
////////////////////////////

pub fn generate_intermediate_representaion(file_path: &PathBuf) -> Result<Vec<BNNode>, BNError>
{
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file).bytes().peekable();
    
    let mut builder = BNIRBuilder::new();

    while let Some(byte) = reader.next() {
        let token = BNTokenMatcher::match_token(byte?);
        builder.enter_token(token)?;
    }
    
    return Ok(builder.finalize()?);
}

//////////////////////////
/////// IR BUILDER ///////
//////////////////////////

#[derive(Debug, PartialEq)]
pub enum BNNode {
    Add(usize),
    Sub(usize),
    Right(usize),
    Left(usize),
    Loop(usize),
    EndLoop(usize),
    In(usize),
    Out(usize)
}

pub trait IRBuilderTrait {
    fn new() -> Self;
    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError>;
    fn finalize(self) -> Result<Vec<BNNode>, BNError>;
}

pub struct BNIRBuilder {
    nodes: Vec<BNNode>,
    loop_stack: Vec<usize>,
    token_sequence: usize,
    previous_token: BNToken,
}

impl BNIRBuilder 
{
    fn collapse_previous_token(&mut self)
    {
        match self.previous_token {
            BNToken::Add => self.nodes.push(BNNode::Add(self.token_sequence)),
            BNToken::Sub => self.nodes.push(BNNode::Sub(self.token_sequence)),
            BNToken::Right => self.nodes.push(BNNode::Right(self.token_sequence)),
            BNToken::Left => self.nodes.push(BNNode::Left(self.token_sequence)),
            BNToken::In => self.nodes.push(BNNode::In(self.token_sequence)),
            BNToken::Out => self.nodes.push(BNNode::Out(self.token_sequence)),
            _ => { }
        }
        self.token_sequence = 0;
    }
}

impl IRBuilderTrait for BNIRBuilder
{
    fn new() -> Self 
    {
        return Self{ 
            nodes: Vec::new(),
            loop_stack: Vec::new(),
            previous_token: BNToken::None,
            token_sequence: 0
        };
    }

    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError>
    {
        match token {
            BNToken::Loop => {
                self.collapse_previous_token();
                self.loop_stack.push(self.nodes.len());
                self.nodes.push(BNNode::Loop(0));
            },
            BNToken::EndLoop => {
                self.collapse_previous_token();
                let index = self.loop_stack.pop().ok_or(BNError::EndLoopTokenMismatch)?;
                self.nodes[index] = BNNode::Loop(self.nodes.len());
                self.nodes.push(BNNode::EndLoop(index));
            },
            // Early exit because None should never be saved as previous token
            BNToken::None => return Ok(()),
            // Collapsing tokens: Add, Sub, Right, Left, In, Out
            // any other tokens will be ignored in collapse_previous_token
            _ => {
                if token != self.previous_token {
                    self.collapse_previous_token();
            }}
        }
        
        self.token_sequence += 1;
        self.previous_token = token;

        return Ok(());
    }

    fn finalize(mut self) -> Result<Vec<BNNode>, BNError>
    {
        self.collapse_previous_token();
        if !self.loop_stack.is_empty() {
            return Err(BNError::LoopTokenMismatch);
        }
        return Ok(self.nodes);
    }
}

/////////////////////////////
/////// TOKEN MATCHING //////
/////////////////////////////

const ADD: u8       = b'+';
const SUB: u8       = b'-';
const RIGHT: u8     = b'>';
const LEFT: u8      = b'<';
const LOOP: u8      = b'[';
const ENDLOOP: u8   = b']';
const OUT: u8       = b'.';
const IN: u8        = b',';

#[derive(Debug, PartialEq, Clone)]
pub enum BNToken {
    Add,
    Sub,
    Right,
    Left,
    Loop,
    EndLoop,
    In,
    Out,
    None
}

pub trait TokenMatcherTrait {
    fn match_token(byte: u8) -> BNToken;
}

struct BNTokenMatcher;
impl TokenMatcherTrait for BNTokenMatcher
{
    fn match_token(byte: u8) -> BNToken
    {
        match byte {
            ADD     => BNToken::Add,
            SUB     => BNToken::Sub,
            RIGHT   => BNToken::Right,
            LEFT    => BNToken::Left,
            LOOP    => BNToken::Loop,
            ENDLOOP => BNToken::EndLoop,
            IN      => BNToken::In,
            OUT     => BNToken::Out,
            _       => BNToken::None
        }
    }
}

///////////////////////////////////
/////// TOKEN MATCHING TESTS //////
///////////////////////////////////

#[cfg(test)]
mod token_matcher_tests
{
    use super::*;
    use crate::bn_assert_eq;

    #[test]
    fn base_tokens()
    {
        for byte in 0..255 {
            match byte {
                ADD     => bn_assert_eq!(BNToken::Add, BNTokenMatcher::match_token(byte)),
                SUB     => bn_assert_eq!(BNToken::Sub, BNTokenMatcher::match_token(byte)),
                RIGHT   => bn_assert_eq!(BNToken::Right, BNTokenMatcher::match_token(byte)),
                LEFT    => bn_assert_eq!(BNToken::Left, BNTokenMatcher::match_token(byte)),
                LOOP    => bn_assert_eq!(BNToken::Loop, BNTokenMatcher::match_token(byte)),
                ENDLOOP => bn_assert_eq!(BNToken::EndLoop, BNTokenMatcher::match_token(byte)),
                IN      => bn_assert_eq!(BNToken::In, BNTokenMatcher::match_token(byte)),
                OUT     => bn_assert_eq!(BNToken::Out, BNTokenMatcher::match_token(byte)),
                _       => bn_assert_eq!(BNToken::None, BNTokenMatcher::match_token(byte)),
            }
        }
    }
}

////////////////////////////////
/////// IR BUILDER TESTS ///////
////////////////////////////////

#[cfg(test)]
mod ir_builder_tests
{
    use super::*;
    use crate::bn_expect_error;
    use crate::bn_unwrap;
    use crate::bn_assert_eq;

    #[test]
    fn single_token_to_node()
    {
        let mut builder = BNIRBuilder::new();
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Add));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Sub));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Right));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Left));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::In));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Out));
        bn_unwrap!(builder.enter_token(BNToken::None));

        let intrep = bn_unwrap!(builder.finalize());
        bn_assert_eq!(BNNode::Add(1), intrep[0], 0);
        bn_assert_eq!(BNNode::Sub(1), intrep[1], 1);
        bn_assert_eq!(BNNode::Right(1), intrep[2], 2);
        bn_assert_eq!(BNNode::Left(1), intrep[3], 3);
        bn_assert_eq!(BNNode::Loop(5), intrep[4], 4);
        bn_assert_eq!(BNNode::EndLoop(4), intrep[5], 5);
        bn_assert_eq!(BNNode::In(1), intrep[6], 6);
        bn_assert_eq!(BNNode::Out(1), intrep[7], 7);
        bn_assert_eq!("Length does not match expected", 8, intrep.len());

    }

    #[test]
    fn collapsed_tokens()
    {
        let mut builder = BNIRBuilder::new();
        let amount = 10;
        let tokens = [
            BNToken::None, BNToken::Add,
            BNToken::Sub, BNToken::Right,
            BNToken::Left, BNToken::In,
            BNToken::Out,
        ];

        for token in tokens { 
            for _ in 0..amount {
                bn_unwrap!(builder.enter_token(token.clone()));
        }}

        let intrep = bn_unwrap!(builder.finalize());
        bn_assert_eq!(BNNode::Add(amount), intrep[0], 0);
        bn_assert_eq!(BNNode::Sub(amount), intrep[1], 1);
        bn_assert_eq!(BNNode::Right(amount), intrep[2], 2);
        bn_assert_eq!(BNNode::Left(amount), intrep[3], 3);
        bn_assert_eq!(BNNode::In(amount), intrep[4], 4);
        bn_assert_eq!(BNNode::Out(amount), intrep[5], 5);
        bn_assert_eq!("Length does not match expected", 6, intrep.len());
    }

    #[test]
    fn collapsed_none_interrupted_tokens()
    {
        let mut builder = BNIRBuilder::new();
        let amount = 10;
        let half_amount = amount / 2;
        let tokens = [
            BNToken::Add, BNToken::Sub,
            BNToken::Right, BNToken::Left,
            BNToken::In, BNToken::Out,
        ];

        for token in tokens { 
            for _ in 0..half_amount {
                bn_unwrap!(builder.enter_token(token.clone()));
            } 
            bn_unwrap!(builder.enter_token(BNToken::None));
            for _ in 0..half_amount {
                bn_unwrap!(builder.enter_token(token.clone()));
        }}

        let intrep = bn_unwrap!(builder.finalize());
        bn_assert_eq!(BNNode::Add(amount), intrep[0], 0);
        bn_assert_eq!(BNNode::Sub(amount), intrep[1], 1);
        bn_assert_eq!(BNNode::Right(amount), intrep[2], 2);
        bn_assert_eq!(BNNode::Left(amount), intrep[3], 3);
        bn_assert_eq!(BNNode::In(amount), intrep[4], 4);
        bn_assert_eq!(BNNode::Out(amount), intrep[5], 5);
        bn_assert_eq!("Length does not match expected", 6, intrep.len());
    }

    #[test]
    fn collapsed_interrupted_tokens()
    {
        let mut builder = BNIRBuilder::new();
        let amount = 10;
        let half_amount = amount / 2;
        let tokens = [
            BNToken::Add, BNToken::Sub,
            BNToken::Right, BNToken::Left,
            BNToken::In, BNToken::Out,
        ];

        for token in tokens { 
            for _ in 0..half_amount {
                bn_unwrap!(builder.enter_token(token.clone()));
            } 
            bn_unwrap!(builder.enter_token(BNToken::Loop));
            for _ in 0..half_amount {
                bn_unwrap!(builder.enter_token(token.clone()));
            }
            bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        }

        let intrep = bn_unwrap!(builder.finalize());
        bn_assert_eq!(BNNode::Add(half_amount), intrep[0], 0);
        bn_assert_eq!(BNNode::Add(half_amount), intrep[2], 2);
        bn_assert_eq!(BNNode::Sub(half_amount), intrep[4], 4);
        bn_assert_eq!(BNNode::Sub(half_amount), intrep[6], 6);
        bn_assert_eq!(BNNode::Right(half_amount), intrep[8], 8);
        bn_assert_eq!(BNNode::Right(half_amount), intrep[10], 10);
        bn_assert_eq!(BNNode::Left(half_amount), intrep[12], 12);
        bn_assert_eq!(BNNode::Left(half_amount), intrep[14], 14);
        bn_assert_eq!(BNNode::In(half_amount), intrep[16], 16);
        bn_assert_eq!(BNNode::In(half_amount), intrep[18], 18);
        bn_assert_eq!(BNNode::Out(half_amount), intrep[20], 20);
        bn_assert_eq!(BNNode::Out(half_amount), intrep[22], 22);
        bn_assert_eq!("Length does not match expected", 24, intrep.len());
    }

    #[test]
    fn loop_mismatch_error()
    {
        let mut builder = BNIRBuilder::new();
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_expect_error!(builder.finalize(), BNError::LoopTokenMismatch);
    }

    #[test]
    fn end_loop_mismatch_error()
    {
        let mut builder = BNIRBuilder::new();
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_expect_error!(builder.enter_token(BNToken::EndLoop), BNError::EndLoopTokenMismatch);
    }
}