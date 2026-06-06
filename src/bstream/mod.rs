use std::marker::PhantomData;
use std::cell::Cell;
use std::vec::Vec;
use std::ops::{Add, AddAssign};
use crate::bnintermediate::*;
use crate::bnerror::BNError;

pub mod bstream_intermediate;
pub mod bstream_codegen;

// #[derive(Debug, PartialEq, Clone)]
// #[repr(u8)]
// pub enum Inst {
//     Add(u16),
//     Sub(u16),
//     Right(u16),
//     Left(u16),
//     Loop(u16),
//     EndLoop(u16),
//     In,
//     Out,

//     /*  Used for temporary start instruction
//         Always keep as last instruction */
//         Sentinal
// }

// impl Inst {
//     #[inline]
//     pub fn discriminant(&self) -> u8 {
//         // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
//         // between `repr(C)` structs, each of which has the `u8` discriminant as its first
//         // field, so we can read the discriminant without offsetting the pointer.
//         unsafe { *<*const Self>::from(self).cast::<u8>() }
//     }
// }

pub trait InstructionPacker {
    type IValue;
    const ID: u8;
    fn pack_value(dest: &mut Vec<u8>, value: Self::IValue);
}

pub struct IAdd;
impl InstructionPacker for IAdd {
    type IValue = u16;
    const ID: u8 = 0;
    #[inline(always)]
    fn pack_value(dest: &mut Vec<u8>, value: Self::IValue) {
        dest.extend_from_slice(&value.to_ne_bytes());
    }
}

pub struct ISub;
impl InstructionPacker for ISub {
    type IValue = u16;
    const ID: u8 = 1;
    #[inline(always)]
    fn pack_value(dest: &mut Vec<u8>, value: Self::IValue) {
        dest.extend_from_slice(&value.to_ne_bytes());
    }
}

pub struct IRight;
impl InstructionPacker for IRight {
    type IValue = u16;
    const ID: u8 = 2;
    #[inline(always)]
    fn pack_value(dest: &mut Vec<u8>, value: Self::IValue) {
        dest.extend_from_slice(&value.to_ne_bytes());
    }
}

pub struct ILeft;
impl InstructionPacker for ILeft {
    type IValue = u16;
    const ID: u8 = 3;
    #[inline(always)]
    fn pack_value(dest: &mut Vec<u8>, value: Self::IValue) {
        dest.extend_from_slice(&value.to_ne_bytes());
    }
}

pub struct ILoop;
impl InstructionPacker for ILoop {
    type IValue = u16;
    const ID: u8 = 4;
    #[inline(always)]
    fn pack_value(dest: &mut Vec<u8>, value: Self::IValue) {
        dest.extend_from_slice(&value.to_ne_bytes());
    }
}

pub struct IEndLoop;
impl InstructionPacker for IEndLoop {
    type IValue = u16;
    const ID: u8 = 5;
    #[inline(always)]
    fn pack_value(dest: &mut Vec<u8>, value: Self::IValue) {
        dest.extend_from_slice(&value.to_ne_bytes());
    }
}

pub trait Instruction {
    const ID: u8;
}
pub struct IIn;
impl Instruction for IIn {
    const ID: u8 = 6;
}
pub struct IOut;
impl Instruction for IOut {
    const ID: u8 = 7;
}

pub struct INone;
impl Instruction for INone {
    const ID: u8 = 8;
}

type Id<'id> = PhantomData<Cell<&'id ()>>;

#[derive(Copy, Clone)]
pub struct IndexId<'id> {
    index: usize,
    _id: Id<'id>,
}

#[derive(Copy, Clone)]
pub struct TypedIndexId<'id> {
    index: usize,
    typed_id: u8,
    _id: Id<'id>,
}

// pub struct ByteStream {
//     ctx: ByteStreamCtx<'static>,
// }

// impl ByteStream {
//     fn enter<F, R>(&mut self, f: F) -> R
//         where F: for<'id> FnOnce(&mut ByteStreamCtx<'id>) -> R
//     {
//         f(&mut self.ctx)
//     }
// }

pub struct ByteStreamCtx<'id> {
    stream: Vec<u8>,
    _id: Id<'id>,
}

impl<'id> ByteStreamCtx<'id> {
    #[inline]
    fn new() -> Self {
        return Self { 
            stream: Vec::new(),
            _id: PhantomData,
        };
    }

    #[inline]
    fn alloc<IValue: Instruction>(&mut self) -> TypedIndexId<'id> {
        let offset: usize = self.stream.len();
        self.stream.push(IValue::ID);

        return TypedIndexId { index: offset, typed_id: IValue::ID, _id: PhantomData }
    }

    #[inline]
    fn alloc_packed<Packer: InstructionPacker>(&mut self, value: Packer::IValue) -> TypedIndexId<'id> {
        let offset: usize = self.stream.len();
        self.stream.push(Packer::ID);
        Packer::pack_value(&mut self.stream, value);

        return TypedIndexId { index: offset, typed_id: Packer::ID, _id: PhantomData }
    }

    #[inline]
    fn alloc_or_increment<Packer: InstructionPacker>(&mut self, id: TypedIndexId<'id>) -> TypedIndexId<'id> 
        where Packer::IValue: Add<Output = Packer::IValue> + AddAssign<Packer::IValue> + From<u8> {
        // where Packer::IValue: Add<Output = Packer::IValue> + From<u8> {
        if id.typed_id == Packer::ID {
            unsafe {
                let offset = id.index + 1;
                let ptr = self.stream.as_mut_ptr().add(offset).cast::<Packer::IValue>();
                // ptr.write_unaligned(ptr.read_unaligned() + Packer::IValue::from(1));
                *ptr.as_mut_unchecked() += Packer::IValue::from(1);
                // *ptr += Packer::IValue::from(1);
            }
            return id
        } else {
            return self.alloc_packed::<Packer>(Packer::IValue::from(1))
        }
    }

    #[inline]
    fn write_value<Packer: InstructionPacker>(&mut self, id: IndexId<'id>, value: Packer::IValue) {
        unsafe {
            let offset = id.index + 1;
            let ptr = self.stream.as_mut_ptr().add(offset).cast::<Packer::IValue>();
            ptr.write_unaligned(value);
        }
    }

    #[inline]
    fn index_to_id(&self, index: usize) -> IndexId<'id> {
        return IndexId { index: index, _id: PhantomData }
    }
}

pub struct ByteStreamBuilder {
    ctx: ByteStreamCtx<'static>,
    head: TypedIndexId<'static>,
    loop_stack: Vec<usize>,
}

const SENTINAL_OFFSET: usize = 3;
impl IRBuilderTrait for ByteStreamBuilder {
    type Elem = u8;

    #[inline]
    fn new() -> Self {
        let mut new_ctx = ByteStreamCtx::new();
        // new_ctx.stream.reserve_exact(4096);
        new_ctx.stream.reserve_exact(400000);

        let new_head = new_ctx.alloc::<INone>();
        new_ctx.alloc::<INone>();
        new_ctx.alloc::<INone>();

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
                self.head = self.ctx.alloc_packed::<IEndLoop>((start_index - SENTINAL_OFFSET) as <IEndLoop as InstructionPacker>::IValue);
                self.ctx.write_value::<ILoop>(self.ctx.index_to_id(start_index), (self.head.index - SENTINAL_OFFSET) as <ILoop as InstructionPacker>::IValue);
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
mod bstream_tests {
    use super::*;
    use crate::bn_assert_slices_eq;
    use crate::bn_unwrap;
    use crate::bn_print_expected_found;
    use crate::bn_expect_error;

    #[test]
    fn single_token_to_node() {
        let mut builder = ByteStreamBuilder::new();

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
        let mut builder = ByteStreamBuilder::new();
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
        let mut builder = ByteStreamBuilder::new();
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
        let mut builder = ByteStreamBuilder::new();
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
        let mut builder = ByteStreamBuilder::new();
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_expect_error!(builder.finalize(), BNError::LoopTokenMismatch);
    }

    #[test]
    fn end_loop_mismatch_error()
    {
        let mut builder = ByteStreamBuilder::new();
        bn_unwrap!(builder.enter_token(BNToken::Loop));
        bn_unwrap!(builder.enter_token(BNToken::EndLoop));
        bn_expect_error!(builder.enter_token(BNToken::EndLoop), BNError::EndLoopTokenMismatch);
    }
}