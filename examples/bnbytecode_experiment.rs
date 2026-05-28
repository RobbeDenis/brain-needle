use std::vec::Vec;
use brain_needle::*;
use bnbytecode::core::{PackedEnumInstructions, Instruction};

fn main() {
    // 8 * (1 byte (discriminant) + 1 byte allignment + 2 bytes (sizeof biggest enum)) = 8 * 4 bytes = 32 bytes
    let mut base_enums: Vec<Instruction> = vec!{
        Instruction::Add(0), Instruction::Sub(0), Instruction::Right(0),
        Instruction::Left(0), Instruction::Loop(0), Instruction::EndLoop(0),
        Instruction::In, Instruction::Out
    };

    // 8 * 1 byte (discriminant) + 6 * 2 bytes = 8 bytes + 12 bytes = 20 bytes
    let mut packed_enums = PackedEnumInstructions::new();
    packed_enums.push(Instruction::Add(0));
    packed_enums.push(Instruction::Sub(0));
    packed_enums.push(Instruction::Right(0));
    packed_enums.push(Instruction::Left(0));
    packed_enums.push(Instruction::Loop(0));
    packed_enums.push(Instruction::EndLoop(0));
    packed_enums.push(Instruction::In);
    packed_enums.push(Instruction::Out);

    println!("\nTotal allocated bytes:");
    println!("base:   {} bytes", base_enums.capacity() * size_of::<Instruction>());
    println!("packed: {} bytes", packed_enums.allocated_size());

    println!("\nTotal allocated bytes after fit:");
    base_enums.shrink_to_fit();
    println!("base:   {} bytes", base_enums.capacity() * size_of::<Instruction>());
    packed_enums.internal_mut().shrink_to_fit();
    println!("packed: {} bytes", packed_enums.allocated_size());
}