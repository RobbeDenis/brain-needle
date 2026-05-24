
use crate::bnemit::BNEmitter;
use crate::bndest::BNDest;
use crate::bnintrep::BNNode;
use crate::bncore::BYTE_CEIL_WRAP_U8;
use crate::bncore::MAX_PROGRAM_BYTES;

pub struct X86X64LinuxEmitter // needs drop???
{
    dest: Box<dyn BNDest>,
    loop_stack: Vec<usize>,
    loop_count: usize
}

impl X86X64LinuxEmitter
{
    pub const fn new(dest: Box<dyn BNDest>) -> X86X64LinuxEmitter
    {
        return X86X64LinuxEmitter{ dest: dest, loop_stack: Vec::new(), loop_count: 0 };
    }

    fn emit_arithmetic_wrap(&mut self)
    {
        self.dest.push(format!("\tand byte [rsi], {}\n", BYTE_CEIL_WRAP_U8 - 1).as_bytes());
    }

    fn emit_shift_wrap(&mut self)
    {
        self.dest.push(format!("\tand rsp, {}\n\tlea rsi, [rbx + rsp]\n", MAX_PROGRAM_BYTES - 1).as_bytes());
    }
}

impl BNEmitter for X86X64LinuxEmitter
{
    fn emit_setup(&mut self)
    {
        self.dest.push(format!("section .data\n\ttape times {0} db 0\n\nsection .text\n\tglobal _start\n\n_start:\n\tlea rbx, [rel tape]\n\txor rsp, rsp\n\tlea rsi, [rbx + rsp]\n\tmov byte [rsi], 0\n\n",
                                        MAX_PROGRAM_BYTES).as_bytes());
    }

    fn emit_arithmetic(&mut self, node: &BNNode)
    {
        match *node {
            BNNode::Add(value) => self.dest.push(format!("\tadd byte [rsi], {}\n", value).as_bytes()),
            BNNode::Sub(value) => self.dest.push(format!("\tsub byte [rsi], {}\n", value).as_bytes()),
            _ => panic!("Node found that was not Add or Sub")
        }
        self.emit_arithmetic_wrap();
    }

    fn emit_shift(&mut self, node: &BNNode)
    {
        match *node {
            BNNode::Right(value) => {
                self.dest.push(format!("\tadd rsp, {}\n", value).as_bytes());
            },
            BNNode::Left(value) => {
                self.dest.push(format!("\tsub rsp, {}\n", value).as_bytes());
            },
            _ => panic!("Node found that was not Right or Left")
        }
        self.emit_shift_wrap();
    }

    fn emit_jump(&mut self, node: &BNNode) -> Option<usize>
    {
        match *node {
            BNNode::Loop(_value) => {
                self.loop_count += 1;
                self.loop_stack.push(self.loop_count);
                self.dest.push(format!("LS_{}:\n", self.loop_count).as_bytes());
                self.dest.push(b"\tcmp byte [rsi], 0\n");
                self.dest.push(format!("\tjz LE_{}\n", self.loop_count).as_bytes());
            },
            BNNode::EndLoop(_value) => {
                let idx = self.loop_stack.pop().unwrap();
                self.dest.push(format!("\tjmp LS_{}\n", idx).as_bytes());
                self.dest.push(format!("LE_{}:\n", idx).as_bytes());
            },
            _ => panic!("Node found that was not Jump")
        }

        return None;
    }
    
    fn emit_out(&mut self)
    {
        // rax << 1 => sys_write
        // rdi << 1 => stdout
        // rdx << 1 => length
        self.dest.push(b"\tmov rax, 1\n\tmov rdi, 1\n\tmov rdx, 1\n\tsyscall\n\n");
    }
    
    fn emit_in(&mut self)
    {
        // rax << 0 => sys_read
        // rdi << 0 => stdin
        // rdx << 1 => length
        self.dest.push(b"\txor rax, rax\n\txor rdi, rdi\n\tmov rdx, 1\n\tsyscall\n\n");
    }
    
    fn emit_exit(&mut self)
    {
        self.dest.push(b"\tmov rax, 60\n\txor rdi, rdi\n\tsyscall\n");
    }

    fn finalize(&mut self) 
    {
        self.dest.finalize();
    }
}