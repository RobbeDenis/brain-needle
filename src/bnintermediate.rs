
use std::{io::Read};

use crate::bnerror::BNError;

////////////////////////////
//////// GENERATE IR ///////
////////////////////////////

pub fn generate_intermediate_representation<R: Read>(reader: R) -> Result<Vec<BNNode>, BNError>
{
    let mut bytes = reader.bytes().peekable();
    let mut builder = BNIRBuilder::new();

    while let Some(byte) = bytes.next() {
        let token = BNTokenMatcher::match_token(byte?);
        builder.enter_token(token)?;
    }
    
    return Ok(builder.finalize()?);
}

//////////////////////////
/////// IR BUILDER ///////
//////////////////////////

type NodeValue = u16;
#[derive(Debug, PartialEq, Clone)]
pub enum BNNode {
    Add(NodeValue),
    Sub(NodeValue),
    Right(NodeValue),
    Left(NodeValue),
    Loop(NodeValue),
    EndLoop(NodeValue),
    In,
    Out,
    Sentinel
}

impl BNNode {
    #[inline]
    pub fn increment(&mut self) {
        match self {
            BNNode::Add(value) |
            BNNode::Sub(value) |
            BNNode::Right(value) |
            BNNode::Left(value) => *value += 1,
            _ => { }
        }
    }
}

pub trait IRBuilderTrait {
    fn new() -> Self;
    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError>;
    fn finalize(self) -> Result<Vec<BNNode>, BNError>;
}

pub struct BNIRBuilder {
    nodes: Vec<BNNode>,
    loop_stack: Vec<NodeValue>
}

impl BNIRBuilder 
{
    #[inline]
    fn mutate_or_push(&mut self, temp: BNNode) {
        let last_idx = self.nodes.len() - 1; 
        if std::mem::discriminant(&self.nodes[last_idx]) == std::mem::discriminant(&temp) {
            self.nodes[last_idx].increment();
        } else {
            self.nodes.push(temp);
        }
    }
}

impl IRBuilderTrait for BNIRBuilder
{
    fn new() -> Self 
    {
        let mut nodes = Vec::with_capacity(4096);
        nodes.push(BNNode::Sentinel);
        return Self { 
            nodes: nodes,
            loop_stack: Vec::new()
        };
    }

    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError>
    {
        match token {
            BNToken::Add => self.mutate_or_push(BNNode::Add(1)),
            BNToken::Sub => self.mutate_or_push(BNNode::Sub(1)),
            BNToken::Right => self.mutate_or_push(BNNode::Right(1)),
            BNToken::Left => self.mutate_or_push(BNNode::Left(1)),
            BNToken::In => self.nodes.push(BNNode::In),
            BNToken::Out => self.nodes.push(BNNode::Out),
            BNToken::Loop => {
                self.loop_stack.push(self.nodes.len() as NodeValue);
                self.nodes.push(BNNode::Loop(0));
                return Ok(());
            },
            BNToken::EndLoop => {
                let index = self.loop_stack.pop().ok_or(BNError::EndLoopTokenMismatch)?;
                // Decrement index because of sentinel node
                self.nodes[index as usize] = BNNode::Loop(self.nodes.len() as NodeValue - 1);
                self.nodes.push(BNNode::EndLoop(index as NodeValue - 1));
                return Ok(());
            },
            BNToken::None => {
                return Ok(())
            }
        }
        return Ok(());
    }

    fn finalize(self) -> Result<Vec<BNNode>, BNError>
    {
        if !self.loop_stack.is_empty() {
            return Err(BNError::LoopTokenMismatch);
        }
        // Skip sentinel node
        return Ok(self.nodes.into_iter().skip(1).collect());
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
    #[inline]
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
        bn_assert_eq!(BNNode::Add(1), intrep[0]);
        bn_assert_eq!(BNNode::Sub(1), intrep[1]);
        bn_assert_eq!(BNNode::Right(1), intrep[2]);
        bn_assert_eq!(BNNode::Left(1), intrep[3]);
        bn_assert_eq!(BNNode::Loop(5), intrep[4]);
        bn_assert_eq!(BNNode::EndLoop(4), intrep[5]);
        bn_assert_eq!(BNNode::In, intrep[6]);
        bn_assert_eq!(BNNode::Out, intrep[7]);
        bn_assert_eq!(8, intrep.len(), "len");
    }

    #[test]
    fn collapsed_tokens()
    {
        let mut builder = BNIRBuilder::new();
        let amount = 10;
        let tokens = [
            BNToken::None, BNToken::Add,
            BNToken::Sub, BNToken::Right,
            BNToken::Left
        ];

        for token in tokens { 
            for _ in 0..amount {
                bn_unwrap!(builder.enter_token(token.clone()));
        }}

        let intrep = bn_unwrap!(builder.finalize());
        bn_assert_eq!(BNNode::Add(amount), intrep[0]);
        bn_assert_eq!(BNNode::Sub(amount), intrep[1]);
        bn_assert_eq!(BNNode::Right(amount), intrep[2]);
        bn_assert_eq!(BNNode::Left(amount), intrep[3]);
        bn_assert_eq!(4, intrep.len(), "len");
    }

    #[test]
    fn collapsed_none_interrupted_tokens()
    {
        let mut builder = BNIRBuilder::new();
        let amount = 10;
        let half_amount = amount / 2;
        let tokens = [
            BNToken::Add, BNToken::Sub,
            BNToken::Right, BNToken::Left
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
        bn_assert_eq!(BNNode::Add(amount), intrep[0]);
        bn_assert_eq!(BNNode::Sub(amount), intrep[1]);
        bn_assert_eq!(BNNode::Right(amount), intrep[2]);
        bn_assert_eq!(BNNode::Left(amount), intrep[3]);
        bn_assert_eq!(4, intrep.len(), "len");
    }

    #[test]
    fn collapsed_interrupted_tokens()
    {
        let mut builder = BNIRBuilder::new();
        let amount = 10;
        let half_amount = amount / 2;
        let tokens = [
            BNToken::Add, BNToken::Sub,
            BNToken::Right, BNToken::Left
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
        bn_assert_eq!(BNNode::Add(half_amount), intrep[0]);
        bn_assert_eq!(BNNode::Add(half_amount), intrep[2]);
        bn_assert_eq!(BNNode::Sub(half_amount), intrep[4]);
        bn_assert_eq!(BNNode::Sub(half_amount), intrep[6]);
        bn_assert_eq!(BNNode::Right(half_amount), intrep[8]);
        bn_assert_eq!(BNNode::Right(half_amount), intrep[10]);
        bn_assert_eq!(BNNode::Left(half_amount), intrep[12]);
        bn_assert_eq!(BNNode::Left(half_amount), intrep[14]);
        bn_assert_eq!(16, intrep.len(), "len");
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