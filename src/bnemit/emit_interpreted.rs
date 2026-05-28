
use std::io::Read;

use crate::bncore::MAX_PROGRAM_BYTES;
use crate::bncore::BYTE_CEIL_WRAP_U8;
use crate::bnemit::BNEmitter;
use crate::bndest::BNDest;
use crate::bnintermediate::BNNode;

pub struct InterpretedEmitter
{
    dest: Box<dyn BNDest>,
    cells: Vec<u8>,
    ptr_idx: usize
}

impl InterpretedEmitter
{
    pub const fn new(dest: Box<dyn BNDest>) -> InterpretedEmitter
    {
        return InterpretedEmitter{ dest: dest, cells: Vec::new(), ptr_idx: 0 };
    }
}

impl BNEmitter for InterpretedEmitter
{
    fn emit_setup(&mut self)
    {
        self.cells.resize(MAX_PROGRAM_BYTES, 0);
    }

    fn emit_arithmetic(&mut self, node: &BNNode)
    {
        match node {
            BNNode::Add(value) => {
                self.cells[self.ptr_idx] = self.cells[self.ptr_idx].wrapping_add(*value as u8) % BYTE_CEIL_WRAP_U8;
            },
            BNNode::Sub(value) => {
                self.cells[self.ptr_idx] = self.cells[self.ptr_idx].wrapping_sub(*value as u8) % BYTE_CEIL_WRAP_U8;
            },
            _ => panic!("Called emit_arithmetic when given node was not arithmetic")
        }
    }

    fn emit_shift(&mut self, node: &BNNode)
    {
        match node {
            BNNode::Right(value) => {
                self.ptr_idx += *value as usize % MAX_PROGRAM_BYTES;
                if self.ptr_idx >= MAX_PROGRAM_BYTES { self.ptr_idx = self.ptr_idx % MAX_PROGRAM_BYTES }
            },
            BNNode::Left(value) => {
                self.ptr_idx = (self.ptr_idx + MAX_PROGRAM_BYTES - (*value as usize % MAX_PROGRAM_BYTES)) % MAX_PROGRAM_BYTES;
            },
            _ => panic!("Called emit_shift when given node was not a shift")
        }
    }

    fn emit_jump(&mut self, node: &BNNode) -> Option<usize>
    {
        match node {
            BNNode::Loop(value) => {
                if self.cells[self.ptr_idx] == 0 {
                    return Some(*value as usize);
                }
            },
            BNNode::EndLoop(value) => {
                if self.cells[self.ptr_idx] != 0 {
                    return Some(*value as usize);
                }
            },
            _ => panic!("Called emit_jump when given node was not a jump")
        }
        
        return None;
    }

    fn emit_out(&mut self)
    {
        self.dest.push(&[self.cells[self.ptr_idx]]);
    }

    fn emit_in(&mut self)
    {
        let stdin = std::io::stdin().lock();
        let byte = stdin.bytes().next().and_then(|b| b.ok()).unwrap();
        self.cells[self.ptr_idx] = byte;
    }

    fn emit_exit(&mut self)
    {
        self.cells = Vec::new();
    }

    fn finalize(&mut self)
    {
        self.dest.finalize();
    }
}