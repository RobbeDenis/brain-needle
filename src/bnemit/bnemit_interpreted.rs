
// using
use crate::bncore::MAX_PROGRAM_BYTES;
use crate::bnemit::BNEmitter;
use crate::bndest::BNDest;
use crate::bnparse::Node;

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
        self.cells.reserve_exact(MAX_PROGRAM_BYTES);
    }

    fn emit_arithmetic(&mut self, node: &Node)
    {
        match node {
            Node::Add(value) => {
                self.cells[self.ptr_idx] = self.cells[self.ptr_idx].wrapping_add(*value as u8);
            },
            Node::Sub(value) => {
                self.cells[self.ptr_idx] = self.cells[self.ptr_idx].wrapping_sub(*value as u8);
            },
            _ => panic!("Called emit_arithmetic when given node was not arithmetic")
        }
    }

    fn emit_shift(&mut self, node: &Node)
    {
        match node {
            Node::Right(value) => {
                self.ptr_idx += *value as usize;
                if self.ptr_idx >= MAX_PROGRAM_BYTES { self.ptr_idx = self.ptr_idx % MAX_PROGRAM_BYTES }
            },
            Node::Left(value) => {
                self.ptr_idx = (self.ptr_idx + MAX_PROGRAM_BYTES - (*value as usize % MAX_PROGRAM_BYTES)) % MAX_PROGRAM_BYTES;
            },
            _ => panic!("Called emit_arithmetic when given node was not arithmetic")
        }
    }

    fn emit_jump(&mut self, node: &Node)
    {

    }

    fn emit_out(&mut self)
    {
        let c = (self.cells[self.ptr_idx] as char).to_string();
        self.dest.push(&c);
    }

    fn emit_in(&mut self)
    {
        self.dest.push("[input]");
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