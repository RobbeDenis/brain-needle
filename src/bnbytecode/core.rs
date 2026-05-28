#[cfg(not(debug_assertions))]
use std::hint::unreachable_unchecked;
use std::{vec::Vec};

///////////
// Enum  //
///////////

pub type IValue = u16;
#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Instruction {
    Add(u16),
    Sub(u16),
    Right(u16),
    Left(u16),
    Loop(u16),
    EndLoop(u16),
    In,
    Out,

    /*  Used for temporary start instruction
        Always keep as last instruction */
        Sentinal
}

impl Instruction {
    pub fn discriminant(&self) -> u8 {
        // Rust it's own words:
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    #[inline]
    pub fn increment(&mut self) {
        match self {
            Instruction::Add(value) |
            Instruction::Sub(value) |
            Instruction::Right(value) |
            Instruction::Left(value) => *value += 1,
            _ => { }
        }
    }
}

pub struct PackedEnumInstructions {
    data: Vec<u8>
}

impl PackedEnumInstructions {
    pub fn new() -> Self {
        return Self { data: Vec::new() }
    }

    pub fn allocated_size(&self) -> usize {
        return self.data.capacity() * size_of::<u8>()
    }

    pub fn internal(self) -> Vec<u8> {
        return self.data
    }

    pub fn internal_ref(&self) -> &Vec<u8> {
        return &self.data
    }

    pub fn internal_mut(&mut self) -> &mut Vec<u8> {
        return &mut self.data
    }

    pub fn push(&mut self, instr: Instruction) {
        match instr {
            Instruction::Add(value) |
            Instruction::Sub(value) |
            Instruction::Right(value) |
            Instruction::Left(value) |
            Instruction::Loop(value) |
            Instruction::EndLoop(value) => {
                self.data.push(instr.discriminant());
                self.data.extend_from_slice(&value.to_le_bytes())
            },
            Instruction::In | Instruction::Out => {
                self.data.push(instr.discriminant())
            },
            Instruction::Sentinal => {
                #[cfg(debug_assertions)]
                unreachable!("Sentinal instruction should never pushed");
                #[cfg(not(debug_assertions))]
                unreachable_unchecked();
            }
        }
        println!("packed: {} bytes [{:?}]", self.allocated_size(), instr);
    }
}