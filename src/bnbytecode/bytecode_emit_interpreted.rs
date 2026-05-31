
use std::io::Read;

use crate::bncore::MAX_PROGRAM_BYTES;
use crate::bncore::BYTE_CEIL_WRAP_U8;
use crate::bnbytecode::bytecode_emit::BCEmitter;
use crate::bndest::BNDest;

pub struct BCInterpretedEmitter
{
    dest: Box<dyn BNDest>,
    cells: Vec<u8>,
    ptr_idx: usize
}

impl BCInterpretedEmitter
{
    pub const fn new(dest: Box<dyn BNDest>) -> BCInterpretedEmitter
    {
        return BCInterpretedEmitter{ dest: dest, cells: Vec::new(), ptr_idx: 0 };
    }
}

impl BCEmitter for BCInterpretedEmitter
{
    fn emit_setup(&mut self)
    {
        self.cells.resize(MAX_PROGRAM_BYTES, 0);
    }

    fn emit_add(&mut self, value: u16)
    {
        self.cells[self.ptr_idx] = self.cells[self.ptr_idx].wrapping_add(value as u8) % BYTE_CEIL_WRAP_U8;
    }

    fn emit_sub(&mut self, value: u16)
    {
        self.cells[self.ptr_idx] = self.cells[self.ptr_idx].wrapping_sub(value as u8) % BYTE_CEIL_WRAP_U8;
    }

    fn emit_right(&mut self, value: u16)
    {
        self.ptr_idx += value as usize % MAX_PROGRAM_BYTES;
        if self.ptr_idx >= MAX_PROGRAM_BYTES { 
            self.ptr_idx = self.ptr_idx % MAX_PROGRAM_BYTES 
        }
    }

    fn emit_left(&mut self, value: u16)
    {
        self.ptr_idx = (self.ptr_idx + MAX_PROGRAM_BYTES - (value as usize % MAX_PROGRAM_BYTES)) % MAX_PROGRAM_BYTES;
    }

    fn emit_loop(&mut self,  value: u16) -> Option<usize>
    {
        if self.cells[self.ptr_idx] == 0 {
            return Some(value as usize);
        }
        return None;
    }

    fn emit_end_loop(&mut self,  value: u16) -> Option<usize>
    {
        if self.cells[self.ptr_idx] != 0 {
            return Some(value as usize);
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