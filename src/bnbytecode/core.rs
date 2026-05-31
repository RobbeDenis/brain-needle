use std::vec::Vec;

///////////
// Enum  //
///////////

pub type IValue = u16;
#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Instruction {
    Add(IValue),
    Sub(IValue),
    Right(IValue),
    Left(IValue),
    Loop(IValue),
    EndLoop(IValue),
    In,
    Out,

    /*  Used for temporary start instruction
        Always keep as last instruction */
        Sentinal
}

// Biggest instruction type size + discriminant
pub const SENTINAL_OFFSET: usize = size_of::<IValue>() + 1;

impl Instruction {
    #[inline]
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const Self>::from(self).cast::<u8>() }
    }
}

#[inline]
pub fn read_u16_at(source: &[u8], disc_idx: usize) -> u16 {
    // SAFETY: JUST A TEST
    unsafe {    
        let ptr = source.as_ptr().add(disc_idx + 1);
        ptr.cast::<u16>().read_unaligned()
    }
}

#[inline]
pub fn write_u16_at(dest: &mut [u8], disc_idx: usize, source: u16) {
    // SAFETY: JUST A TEST
    unsafe {
        let ptr = dest.as_mut_ptr().add(disc_idx + 1);
        ptr.cast::<u16>().write_unaligned(source);
    }
}

#[inline]
pub fn increment_u16_at(dest: &mut [u8], disc_idx: usize) {
    // SAFETY: JUST A TEST
    unsafe {
        let ptr = dest.as_mut_ptr().add(disc_idx + 1);
        ptr.cast::<u16>().write_unaligned(ptr.cast::<u16>().read_unaligned() + 1);
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

    #[inline]
    pub fn internal(self) -> Vec<u8> {
        return self.data
    }

    #[inline]
    pub fn internal_ref(&self) -> &Vec<u8> {
        return &self.data
    }

    #[inline]
    pub fn internal_mut(&mut self) -> &mut Vec<u8> {
        return &mut self.data
    }

    #[inline]
    pub fn push(&mut self, instr: Instruction) {
        match instr {
            Instruction::Add(value) |
            Instruction::Sub(value) |
            Instruction::Right(value) |
            Instruction::Left(value) |
            Instruction::Loop(value) |
            Instruction::EndLoop(value) => {
                self.data.push(instr.discriminant());
                self.data.extend_from_slice(&value.to_ne_bytes());
            },
            Instruction::In | Instruction::Out => {
                self.data.push(instr.discriminant())
            },
            Instruction::Sentinal => {
                #[cfg(debug_assertions)]
                unreachable!("Sentinal instruction should never pushed");
            }
        }
    }
}