
// using
use crate::bncore::MAX_PROGRAM_BYTES;
use crate::bnemit::BNEmitter;
use crate::bndest::BNDest;
use crate::bnparse::Node;

pub struct InterpretedEmitter
{
    dest: Box<dyn BNDest>,
    cells: Vec<u8>
}

impl InterpretedEmitter
{
    pub const fn new(dest: Box<dyn BNDest>) -> InterpretedEmitter
    {
        return InterpretedEmitter{ dest: dest, cells: Vec::new() };
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

    }

    fn emit_shift(&mut self, node: &Node)
    {

    }

    fn emit_jump(&mut self, node: &Node)
    {

    }

    fn emit_out(&mut self)
    {

    }

    fn emit_in(&mut self)
    {
        
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