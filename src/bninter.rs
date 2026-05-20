
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::bnerror::BNError;

////////////////////////////
//////// SYNTAX TREE ///////
////////////////////////////

pub fn generate_intermediate(file_path: &PathBuf) -> Result<Vec<BNNode>, BNError>
{
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file).bytes().peekable();
    
    let mut builder = BNNodeBuilder::new();

    while let Some(byte) = reader.next() {
        let token = BNTokenMatcher::match_token(byte?);
        builder.enter_token(token);
    }
    
    return Ok(builder.finalize());
}

////////////////////////////
/////// NODE BUILDER ///////
////////////////////////////

#[derive(Debug)]
pub enum BNNode {
    Add(usize),
    Sub(usize),
    Right(usize),
    Left(usize),
    Loop(usize),
    LoopEnd(usize),
    In(usize),
    Out(usize)
}

pub trait NodeBuilderTrait {
    fn new() -> Self;
    fn enter_token(&mut self, token: BNToken);
    fn finalize(self) -> Vec<BNNode>;
}

pub struct BNNodeBuilder {
    nodes: Vec<BNNode>,
}

impl BNNodeBuilder 
{
    fn handle_add(&mut self) {

    }
    fn handle_sub(&mut self) {

    }
    fn handle_right(&mut self) {

    }
    fn handle_left(&mut self) {

    }
    fn handle_loop(&mut self) {

    }
    fn handle_loop_end(&mut self) {

    }
    fn handle_in(&mut self) {

    }
    fn handle_out(&mut self) {

    }
}

impl NodeBuilderTrait for BNNodeBuilder
{
    fn new() -> Self {
        return Self{ nodes: Vec::new() };
    }

    fn enter_token(&mut self, token: BNToken) 
    {
        match token {
            BNToken::Add     => self.handle_add(),
            BNToken::Sub     => self.handle_sub(),
            BNToken::Right   => self.handle_right(),
            BNToken::Left    => self.handle_left(),
            BNToken::Loop    => self.handle_loop(),
            BNToken::LoopEnd => self.handle_loop_end(),
            BNToken::In      => self.handle_in(),
            BNToken::Out     => self.handle_out(),
            BNToken::None    => { }
        }
    }

    fn finalize(self) -> Vec<BNNode> {
        return self.nodes;
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
const LOOPEND: u8   = b']';
const OUT: u8       = b'.';
const IN: u8        = b',';

pub enum BNToken {
    Add,
    Sub,
    Right,
    Left,
    Loop,
    LoopEnd,
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
            LOOPEND => BNToken::LoopEnd,
            IN      => BNToken::In,
            OUT     => BNToken::Out,
            _       => BNToken::None
        }
    }
}