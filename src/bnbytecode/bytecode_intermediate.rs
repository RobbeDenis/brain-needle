use crate::bnintermediate::*;
use crate::bnerror::BNError;
use crate::bnbytecode::core::*;
use std::io::Read;

pub fn generate_bytecode_intermediate_representation<R: Read>(reader: R) -> Result<Vec<u8>, BNError>
{
    let mut bytes = reader.bytes().peekable();
    let mut builder = BNIRBuilderBytecode::new();

    while let Some(byte) = bytes.next() {
        let token = BNTokenMatcher::match_token(byte?);
        builder.enter_token(token)?;
    }
    
    return Ok(builder.finalize()?);
}

pub struct BNIRBuilderBytecode {
    instructions: PackedEnumInstructions,
    loop_stack: Vec<IValue>,
    prev_disc: u8
}

impl BNIRBuilderBytecode 
{
    #[inline]
    fn mutate_or_push(&mut self, temp: Instruction) {
        let last_idx = self.instructions.internal_ref().len() - size_of::<IValue>() - 1;
        if self.prev_disc == temp.discriminant() {
            let data_range = (last_idx + 1)..(last_idx + size_of::<IValue>());
            let data_value: &mut [u8; size_of::<IValue>()] = self.instructions.internal_mut()[data_range].as_mut_array().unwrap().try_into().unwrap();
            *data_value = (IValue::from_le_bytes(*data_value) + 1).to_le_bytes();
        } else {
            self.push(temp);
        }
    }

    #[inline]
    fn push(&mut self, inst: Instruction) {
        self.prev_disc = inst.discriminant();
        // maybe make a push_mut() so we have a ref to the last full instruction which can be used in mutate_or_push()
        // most likely safer then saving last discriminant and reinterpreting the other bytes based on assuming that prev_disc is not wrongly set
        self.instructions.push(inst); 
    }
}

impl IRBuilderTrait for BNIRBuilderBytecode
{
    type Elem = u8;
    fn new() -> Self 
    {
        let mut inst = PackedEnumInstructions::new();
        let sentinal = Instruction::Sentinal;
        inst.internal_mut().reserve_exact(4096);
        inst.internal_mut()[0] = sentinal.discriminant();
        return Self { 
            instructions: inst,
            loop_stack: Vec::new(),
            prev_disc: sentinal.discriminant()
        };
    }

    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError>
    {
        match token {
            BNToken::Add => self.mutate_or_push(Instruction::Add(1)),
            BNToken::Sub => self.mutate_or_push(Instruction::Sub(1)),
            BNToken::Right => self.mutate_or_push(Instruction::Right(1)),
            BNToken::Left => self.mutate_or_push(Instruction::Left(1)),
            BNToken::In => self.push(Instruction::In),
            BNToken::Out => self.push(Instruction::Out),
            BNToken::Loop => {
                self.loop_stack.push(self.instructions.internal_ref().len() as IValue - size_of::<IValue>() as IValue);
                self.push(Instruction::Loop(0));
                return Ok(());
            },
            BNToken::EndLoop => {
                let index = self.loop_stack.pop().ok_or(BNError::EndLoopTokenMismatch)?;
                let sentinal_offset = 1;
                let loop_end_index = self.instructions.internal_ref().len() as IValue - sentinal_offset - size_of::<IValue>() as IValue;
                // To splice the value of the start loop we need byte index 1 and 2 => loop instruction {d, [v, v]}
                let data_range = (index as usize + 1)..(index as usize + 2);
                self.instructions.internal_mut().splice(data_range, loop_end_index.to_le_bytes());
                self.push(Instruction::EndLoop(index as IValue - sentinal_offset));
                return Ok(());
            },
            BNToken::None => {
                return Ok(())
            }
        }
        return Ok(());
    }

    fn finalize(self) -> Result<Vec<u8>, BNError>
    {
        if !self.loop_stack.is_empty() {
            return Err(BNError::LoopTokenMismatch);
        }
        // Skip sentinel node
        return Ok(self.instructions.internal().into_iter().skip(1).collect());
    }
}