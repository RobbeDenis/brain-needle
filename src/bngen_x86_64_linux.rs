

use crate::bngen_emit::Emitter;
use crate::bnparse::InstrNode;

pub struct X86X64LinuxEmitter
{
    loop_stack: Vec<usize>,
    code: String,
    loop_count: usize
}

impl X86X64LinuxEmitter
{
    pub const fn new() -> X86X64LinuxEmitter
    {
        return X86X64LinuxEmitter{ loop_stack: Vec::new(), code: String::new(), loop_count: 0 };
    }
}

impl Emitter for X86X64LinuxEmitter
{
    fn emit_setup(&mut self)
    {
        self.code += "section .data\ntape times 30000 db 0\nsection .text\nglobal _start\n_start:\nlea rsi, [rel tape]\nmov [rsi], 0\n";
    }

    fn emit_arithmetic(&mut self, node: &InstrNode)
    {
        match *node {
            InstrNode::Add(value) => self.code += format!("add byte [rsi], {}\n", value).as_str(),
            InstrNode::Sub(value) => self.code += format!("sub byte [rsi], {}\n", value).as_str(),
            _ => panic!("Node found that was not Add or Sub")
        }
    }

    fn emit_shift(&mut self, node: &InstrNode)
    {
        match *node {
            InstrNode::Right(value) => self.code += format!("add rsi, {}\n", value).as_str(),
            InstrNode::Left(value) => self.code += format!("sub rsi, {}\n", value).as_str(),
            _ => panic!("Node found that was not Right or Left")
        }
    }

    fn emit_jump(&mut self, node: &InstrNode)
    {
        match *node {
            InstrNode::JumpIfZero(_value) => {
                self.loop_count += 1;
                self.loop_stack.push(self.loop_count);
                self.code += "cmp byte [rsi], 0\n";
                self.code += format!("LS_{}:\n", self.loop_count).as_str();
                self.code += format!("jz LE_{}\n", self.loop_count).as_str();
            },
            InstrNode::JumpIfNotZero(_value) => {
                let idx = self.loop_stack.pop().unwrap();
                self.code += format!("jmp LS_{}\n", idx).as_str();
                self.code += format!("LE_{}:\n", idx).as_str();
            },
            _ => panic!("Node found that was not Jump")
        }
    }
    
    fn emit_out(&mut self)
    {
        // rax << 1 => sys_write
        // rdi << 1 => stdout
        // rdx << 1 => length
        self.code +=   "mov rax, 1\nmov rdi, 1\nmov rdx, 1\nsyscall\n";
    }
    
    fn emit_in(&mut self)
    {
        // rax << 0 => sys_read
        // rdi << 0 => stdin
        // rdx << 1 => length
        self.code +=   "xor rax, rax\nxor rdi, rdi\nmov rdx, 1\nsyscall\n";
    }
    

    fn emit_exit(&mut self)
    {
        self.code +=   "mov rax, 60\nxor rdi, rdi\nsyscall";
    }

    fn clone(&self) -> String
    {
        return self.code.clone();
    }
}