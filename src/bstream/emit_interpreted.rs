
use std::io::Read;

use crate::bncore::MAX_PROGRAM_BYTES;
use crate::bncore::BYTE_CEIL_WRAP_U8;
use crate::bstream::emit::EmitterTrait;
use crate::bndest::BNDest;

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

impl EmitterTrait for InterpretedEmitter
{
    #[inline]
    fn emit_setup(&mut self)
    {
        self.cells.resize(MAX_PROGRAM_BYTES, 0);
    }

    #[inline]
    fn emit_add(&mut self, value: u16)
    {
        self.cells[self.ptr_idx] = self.cells[self.ptr_idx].wrapping_add(value as u8) % BYTE_CEIL_WRAP_U8;
    }

    #[inline]
    fn emit_sub(&mut self, value: u16)
    {
        self.cells[self.ptr_idx] = self.cells[self.ptr_idx].wrapping_sub(value as u8) % BYTE_CEIL_WRAP_U8;
    }

    #[inline]
    fn emit_right(&mut self, value: u16)
    {
        self.ptr_idx += value as usize % MAX_PROGRAM_BYTES;
        if self.ptr_idx >= MAX_PROGRAM_BYTES { 
            self.ptr_idx = self.ptr_idx % MAX_PROGRAM_BYTES 
        }
    }

    #[inline]
    fn emit_left(&mut self, value: u16)
    {
        self.ptr_idx = (self.ptr_idx + MAX_PROGRAM_BYTES - (value as usize % MAX_PROGRAM_BYTES)) % MAX_PROGRAM_BYTES;
    }

    #[inline]
    fn emit_loop(&mut self,  value: u16) -> Option<usize>
    {
        if self.cells[self.ptr_idx] == 0 {
            return Some(value as usize);
        }
        return None;
    }

    #[inline]
    fn emit_end_loop(&mut self,  value: u16) -> Option<usize>
    {
        if self.cells[self.ptr_idx] != 0 {
            return Some(value as usize);
        }
        return None;
    }

    #[inline]
    fn emit_out(&mut self)
    {
        self.dest.push(&[self.cells[self.ptr_idx]]);
    }

    #[inline]
    fn emit_in(&mut self)
    {
        let stdin = std::io::stdin().lock();
        let byte = stdin.bytes().next().and_then(|b| b.ok()).unwrap();
        self.cells[self.ptr_idx] = byte;
    }

    #[inline]
    fn emit_exit(&mut self)
    {
        self.cells = Vec::new();
    }

    #[inline]
    fn finalize(&mut self)
    {
        self.dest.finalize();
    }
}