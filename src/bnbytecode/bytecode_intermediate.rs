use crate::bnintermediate::*;
use crate::bnerror::BNError;
use crate::bnbytecode::core::*;
use std::io::Read;

pub fn generate_bytecode_intermediate_representation<R: Read>(reader: R) -> Result<Vec<u8>, BNError> {
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

impl BNIRBuilderBytecode  {
    #[inline]
    fn increment_or_push(&mut self, temp: Instruction) {
        let data_idx = self.instructions.internal_ref().len() - size_of::<IValue>();
        if self.prev_disc == temp.discriminant() {
            let len_offset = 1;
            let data_range = (data_idx)..(data_idx + size_of::<IValue>() - len_offset);
            let data_value: &mut [u8; size_of::<IValue>()] = self.instructions.internal_mut()[data_range].as_mut_array().unwrap().try_into().unwrap();
            // Increment data_value
            *data_value = (IValue::from_ne_bytes(*data_value) + 1).to_ne_bytes();
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

impl IRBuilderTrait for BNIRBuilderBytecode {
    type Elem = u8;
    fn new() -> Self {
        let mut inst = PackedEnumInstructions::new();
        let sentinal = Instruction::Sentinal;
        inst.internal_mut().reserve_exact(4096);
        // Sentinal discriminant * max bytes of a single instruction
        inst.internal_mut().push(sentinal.discriminant());
        inst.internal_mut().push(sentinal.discriminant());
        inst.internal_mut().push(sentinal.discriminant());
        return Self { 
            instructions: inst,
            loop_stack: Vec::new(),
            prev_disc: sentinal.discriminant()
        };
    }

    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError> {
        match token {
            BNToken::Add => self.increment_or_push(Instruction::Add(1)),
            BNToken::Sub => self.increment_or_push(Instruction::Sub(1)),
            BNToken::Right => self.increment_or_push(Instruction::Right(1)),
            BNToken::Left => self.increment_or_push(Instruction::Left(1)),
            BNToken::In => self.push(Instruction::In),
            BNToken::Out => self.push(Instruction::Out),
            BNToken::Loop => {
                self.loop_stack.push(self.instructions.internal_ref().len() as IValue);
                self.push(Instruction::Loop(0));
                return Ok(());
            },
            BNToken::EndLoop => {
                let s_current_idx = self.loop_stack.pop().ok_or(BNError::EndLoopTokenMismatch)?;
                let sentinal_offset = 1 + size_of::<IValue>() as IValue;
                let e_idx = self.instructions.internal_ref().len() as IValue - sentinal_offset;
                // To splice the value of the start loop we need byte index 1 and 2 => loop instruction {d, [v, v]}
                let data_offset = 1;
                let s_data_range = (s_current_idx as usize + data_offset)..=(s_current_idx as usize + size_of::<IValue>());
                self.instructions.internal_mut().splice(s_data_range, e_idx.to_ne_bytes());
                self.push(Instruction::EndLoop(s_current_idx as IValue - sentinal_offset));
                return Ok(());
            },
            BNToken::None => {
                return Ok(())
            }
        }
        return Ok(());
    }

    fn finalize(self) -> Result<Vec<u8>, BNError> {
        if !self.loop_stack.is_empty() {
            return Err(BNError::LoopTokenMismatch);
        }
        // Skip sentinel bytes (maximum size of an instruction)
        return Ok(self.instructions.internal().into_iter().skip(3).collect());
    }
}

#[cfg(test)]
mod bytecode_ir_builder_tests {
    use super::*;
    use crate::bn_assert_slices_eq;
    use crate::bn_unwrap;
    use crate::bn_print_expected_found;

    #[test]
    fn single_token_to_node() {
        let mut builder = BNIRBuilderBytecode::new();
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Add));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Sub));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Right));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Left));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::In));
        bn_unwrap!(builder.enter_token(BNToken::None));
        bn_unwrap!(builder.enter_token(BNToken::Out));
        bn_unwrap!(builder.enter_token(BNToken::None));

        let bytes = bn_unwrap!(builder.finalize());
        let expected_bytes = vec![0,1,0, 1,1,0, 2,1,0, 3,1,0, 4,15,0, 5,12,0, 6, 7];
        
        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }
}