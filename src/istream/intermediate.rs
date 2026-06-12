use crate::bnerror::BNError;
use crate::istream::*;
use tokenmatcher::*;
use std::io::Read;

#[inline(never)]
pub fn generate_intermediate<R: Read>(reader: R) -> Result<Vec<u8>, BNError> {
    let mut bytes = reader.bytes().peekable();
    let mut builder = IRStreamBuilder::new();

    while let Some(byte) = bytes.next() {
        let token = BNTokenMatcher::match_token(byte?);
        builder.enter_token(token)?;
    }
    
    return Ok(builder.finalize()?);
}

pub trait IRBuilderTrait {
    type Elem;
    fn new() -> Self;
    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError>;
    fn finalize(self) -> Result<Vec<Self::Elem>, BNError>;
}

pub struct IRStreamBuilder {
    ctx: IStreamCtx<'static>,
    head: TypedIndexId<'static>,
    loop_stack: Vec<usize>,
}

const SENTINAL_OFFSET: usize = 64;
impl IRBuilderTrait for IRStreamBuilder {
    type Elem = u8;

    #[inline]
    fn new() -> Self {
        let mut new_ctx = IStreamCtx::new();
        new_ctx.stream.reserve_exact(4096);

        let new_head = new_ctx.alloc::<IPadding>();
        for _ in 1..SENTINAL_OFFSET {
            new_ctx.alloc::<IPadding>();
        }

        return Self { 
            ctx: new_ctx,
            head: new_head,
            loop_stack: Vec::new(),
        };
    }

    #[inline]
    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError> {
        match token {
            BNToken::Add => { self.head = self.ctx.alloc_or_increment::<IAdd>(self.head); },
            BNToken::Sub => { self.head = self.ctx.alloc_or_increment::<ISub>(self.head); },
            BNToken::Right => { self.head = self.ctx.alloc_or_increment::<IRight>(self.head); },
            BNToken::Left => { self.head = self.ctx.alloc_or_increment::<ILeft>(self.head); },
            BNToken::In => { self.head = self.ctx.alloc::<IIn>(); },
            BNToken::Out => { self.head = self.ctx.alloc::<IOut>(); },
            BNToken::Loop => {
                self.head = self.ctx.alloc_packed::<ILoop>(0);
                self.loop_stack.push(self.head.index);
            },
            BNToken::EndLoop => {
                let start_index = self.loop_stack.pop().ok_or(BNError::EndLoopTokenMismatch)?;
                self.head = self.ctx.alloc_packed::<IEndLoop>((start_index - SENTINAL_OFFSET) as <IEndLoop as IPacked>::Payload);
                self.ctx.write_payload::<ILoop>(self.ctx.index_to_id(start_index), (self.head.index - SENTINAL_OFFSET) as <ILoop as IPacked>::Payload);
            },
            BNToken::None => return Ok(())
        }

        return Ok(());
    }

    #[inline]
    fn finalize(self) -> Result<Vec<u8>, BNError> {
        if !self.loop_stack.is_empty() {
            return Err(BNError::LoopTokenMismatch);
        }
        return Ok(self.ctx.stream.into_iter().skip(SENTINAL_OFFSET).collect());
    }
}

/////////////////////////////
///////     TESTS      //////
/////////////////////////////

#[cfg(test)]
mod istream_tests {
    use super::*;
    use crate::bn_assert_slices_eq;
    use crate::bn_unwrap;
    use crate::bn_print_expected_found;
    use crate::bn_expect_error;
    use crate::bn_assert_eq;

    #[test]
    fn single_token_to_node() {
        let mut builder = IRStreamBuilder::new();

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
            0,0,1,0,      1,0,1,0,      2,0,1,0,
            3,0,1,0,      4,0,20,0,     5,0,16,0,
            6,            7
        ];
        
        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }

    #[test]
    fn collapsed_none_interrupted_tokens() {
        let mut builder = IRStreamBuilder::new();
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
        let expected_bytes = vec![0,0,amount,0,   1,0,amount,0,     2,0,amount,0,     3,0,amount,0];
        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }

    #[test]
    fn collapsed_interrupted_tokens()
    {
        let mut builder = IRStreamBuilder::new();
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
            0,0,half_amount,0,    4,0,12,0,      0,0,half_amount,0,    5,0,4,0,
            1,0,half_amount,0,    4,0,28,0,     1,0,half_amount,0,    5,0,20,0,
            2,0,half_amount,0,    4,0,44,0,     2,0,half_amount,0,    5,0,36,0,
            3,0,half_amount,0,    4,0,60,0,     3,0,half_amount,0,    5,0,52,0,
        ];

        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }

    #[test]
    fn bigger_byte_values()
    {
        let mut builder = IRStreamBuilder::new();
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
            0, 0, 0xFF, 0xFF,
            1, 0, 0xFF, 0xFF,
            2, 0, 0xFF, 0xFF,
            3, 0, 0xFF, 0xFF
        ];

        bn_print_expected_found!(expected_bytes, bytes);
        bn_assert_slices_eq!(expected_bytes, bytes, "byte");
    }

    #[test]
    fn cache_line_boundary()
    {
        let exp_cl = vec![
            0, 0, 1, 0
        ];

        for offset in 0..(IAdd::SIZE as usize) {
            let insert_at = CACHE_LINE_SIZE - offset;
            let mut builder = IRStreamBuilder::new();
            for _ in 0..insert_at {
                bn_unwrap!(builder.enter_token(BNToken::Out));
            }
            bn_unwrap!(builder.enter_token(BNToken::Add));
            let stream = bn_unwrap!(builder.finalize());
            let next_cl: Vec<u8> = stream.iter().skip(CACHE_LINE_SIZE).cloned().collect();
            let padding: Vec<u8> = stream.iter().skip(insert_at).take(offset).copied().collect();
            let exp_pad = vec![IPadding::ID; offset];

            bn_print_expected_found!(exp_pad, padding);
            bn_assert_slices_eq!(exp_pad, padding, "padding");

            bn_print_expected_found!(exp_cl, next_cl);
            bn_assert_slices_eq!(exp_cl, next_cl, "cache line");
        }

        let offset = IAdd::SIZE as usize;
        let insert_at = CACHE_LINE_SIZE - offset;
        let mut builder = IRStreamBuilder::new();
        for _ in 0..insert_at {
            bn_unwrap!(builder.enter_token(BNToken::Out));
        }
        bn_unwrap!(builder.enter_token(BNToken::Add));
        let stream = bn_unwrap!(builder.finalize());
        let next_cl: Vec<u8> = stream.iter().skip(CACHE_LINE_SIZE).cloned().collect();
        let insert_v: Vec<u8> = stream.iter().skip(insert_at).take(offset).copied().collect();

        bn_print_expected_found!(exp_cl, insert_v);
        bn_assert_slices_eq!(exp_cl, insert_v);

        bn_assert_eq!(0, next_cl.len(), "len");
    }

    #[test]
    fn loop_mismatch_error()
    {
        let mut builder = IRStreamBuilder::new();
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_expect_error!(builder.finalize(), BNError::LoopTokenMismatch);
    }

    #[test]
    fn end_loop_mismatch_error()
    {
        let mut builder = IRStreamBuilder::new();
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_expect_error!(builder.enter_token(BNToken::EndLoop), BNError::EndLoopTokenMismatch);
    }
}