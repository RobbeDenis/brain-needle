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
        let disc_idx = self.instructions.internal_ref().len() - size_of::<IValue>() - 1;

        // look into a & b => into match arm
        if self.prev_disc == temp.discriminant() {
            increment_u16_at(self.instructions.internal_mut(), disc_idx);
        } else {
            self.push(temp);
        }
    }

    #[inline]
    fn push(&mut self, inst: Instruction) {
        self.prev_disc = inst.discriminant();
        // maybe make a push_mut() so we have a ref to the last full instruction which can be used in mutate_or_push() TODO
        // most likely safer then saving last discriminant and reinterpreting the other bytes based on assuming that prev_disc is not wrongly set
        self.instructions.push(inst);
    }
}

impl IRBuilderTrait for BNIRBuilderBytecode {
    type Elem = u8;

    #[inline]
    fn new() -> Self {
        let mut inst = PackedEnumInstructions::new();
        inst.internal_mut().reserve_exact(4096);
        
        // Set the sentinal offset = largest instruction size
        let sentinal = Instruction::Sentinal;
        for _ in 0..SENTINAL_OFFSET {
            inst.internal_mut().push(sentinal.discriminant());
        }

        return Self { 
            instructions: inst,
            loop_stack: Vec::new(),
            prev_disc: sentinal.discriminant()
        };
    }

    #[inline]
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
                let e_idx_value = (self.instructions.internal_ref().len() - SENTINAL_OFFSET) as IValue ;
                let s_idx = self.loop_stack.pop().ok_or(BNError::EndLoopTokenMismatch)?;
                write_u16_at(self.instructions.internal_mut(), s_idx as usize, e_idx_value);
                self.push(Instruction::EndLoop(s_idx - SENTINAL_OFFSET as IValue));
                return Ok(());
            },
            BNToken::None => {
                return Ok(())
            }
        }
        return Ok(());
    }

    #[inline]
    fn finalize(self) -> Result<Vec<u8>, BNError> {
        if !self.loop_stack.is_empty() {
            return Err(BNError::LoopTokenMismatch);
        }

        return Ok(self.instructions.internal().into_iter().skip(SENTINAL_OFFSET).collect());
    }
}

/////////////////////////////
///////     TESTS      //////
/////////////////////////////

#[cfg(test)]
mod bytecode_ir_builder_tests {
    use super::*;
    use crate::bn_assert_slices_eq;
    use crate::bn_unwrap;
    use crate::bn_print_expected_found;
    use crate::bn_expect_error;

    #[test]
    fn single_token_to_node() {
        let mut builder = BNIRBuilderBytecode::new();
        let tokens = [
            BNToken::None, BNToken::Add,    BNToken::None, BNToken::Sub,
            BNToken::None, BNToken::Right,  BNToken::None, BNToken::Left,
            BNToken::None, BNToken::Loop,   BNToken::None, BNToken::EndLoop,
            BNToken::None, BNToken::In,     BNToken::None, BNToken::Out,
            BNToken::None,
        ];

        for token in tokens {
            bn_unwrap!(builder.enter_token(token));
        }

        let bytes = bn_unwrap!(builder.finalize());
        let expected_bytes = vec![
            0,1,0,      1,1,0,      2,1,0,
            3,1,0,      4,15,0,     5,12,0,
            6,          7
        ];
        
        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }

    #[test]
    fn collapsed_none_interrupted_tokens() {
        let mut builder = BNIRBuilderBytecode::new();
        let amount = 10;
        let half_amount = amount / 2;
        let tokens = [
            BNToken::Add,   BNToken::Sub,
            BNToken::Right, BNToken::Left
        ];

        for token in tokens { 
            for _ in 0..half_amount {
                bn_unwrap!(builder.enter_token(token.clone()));
            } 
            bn_unwrap!(builder.enter_token(BNToken::None));
            for _ in 0..half_amount {
                bn_unwrap!(builder.enter_token(token.clone()));
        }}

        let bytes = bn_unwrap!(builder.finalize());
        let expected_bytes = vec![0,amount,0,   1,amount,0,     2,amount,0,     3,amount,0];
        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }

    #[test]
    fn collapsed_interrupted_tokens()
    {
        let mut builder = BNIRBuilderBytecode::new();
        let amount = 10;
        let half_amount = amount / 2;
        let tokens = [
            BNToken::Add,   BNToken::Sub,
            BNToken::Right, BNToken::Left
        ];

        for token in tokens { 
            for _ in 0..half_amount {
                bn_unwrap!(builder.enter_token(token.clone()));
            } 
            bn_unwrap!(builder.enter_token(BNToken::Loop));
            for _ in 0..half_amount {
                bn_unwrap!(builder.enter_token(token.clone()));
            }
            bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        }

        let bytes = bn_unwrap!(builder.finalize());
        let expected_bytes = vec![
            0,half_amount,0,    4,9,0,      0,half_amount,0,    5,3,0,
            1,half_amount,0,    4,21,0,     1,half_amount,0,    5,15,0,
            2,half_amount,0,    4,33,0,     2,half_amount,0,    5,27,0,
            3,half_amount,0,    4,45,0,     3,half_amount,0,    5,39,0,
        ];

        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }

    #[test]
    fn bigger_byte_values()
    {
        let mut builder = BNIRBuilderBytecode::new();
        let amount = u16::MAX;
        let tokens = [
            BNToken::Add,   BNToken::Sub,
            BNToken::Right, BNToken::Left
        ];

        for token in tokens {
            for _ in 0..amount {
                bn_unwrap!(builder.enter_token(token.clone()));
            }
        }

        let bytes = bn_unwrap!(builder.finalize());
        let expected_bytes = vec![
            0, 0xFF, 0xFF,
            1, 0xFF, 0xFF,
            2, 0xFF, 0xFF,
            3, 0xFF, 0xFF
        ];

        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }

    #[test]
    fn loop_mismatch_error()
    {
        let mut builder = BNIRBuilderBytecode::new();
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_expect_error!(builder.finalize(), BNError::LoopTokenMismatch);
    }

    #[test]
    fn end_loop_mismatch_error()
    {
        let mut builder = BNIRBuilderBytecode::new();
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_expect_error!(builder.enter_token(BNToken::EndLoop), BNError::EndLoopTokenMismatch);
    }
}