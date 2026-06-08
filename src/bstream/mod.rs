use std::marker::PhantomData;
use std::cell::Cell;
use std::vec::Vec;
use std::ops::{Add, AddAssign};
use crate::bnintermediate::*;
use crate::bnerror::BNError;

pub mod intermediate;
pub mod codegen;
pub mod emit;
pub mod emit_interpreted;

pub trait Instruction {
    const ID: u8;
}

pub trait IConst: Instruction {
    const SIZE: usize = 1;

    #[inline(always)]
    fn alloc(dest: &mut Vec<u8>) {
        dest.push(Self::ID);
    }
}

pub trait IPacked: Instruction {
    type Payload: Sized + Copy;
    const PAYLOAD_OFFSET: usize = align_of::<Self::Payload>();
    const SIZE: usize = Self::PAYLOAD_OFFSET + size_of::<Self::Payload>();
    
    #[inline(always)]
    fn alloc(dest: &mut Vec<u8>, payload: Self::Payload) {
        dest.push(Self::ID);
        dest.extend(std::iter::repeat_n(0, Self::PAYLOAD_OFFSET - size_of::<u8>()));

        let bytes = unsafe {
            std::slice::from_raw_parts(
                    &payload as *const Self::Payload as *const u8,
                    size_of::<Self::Payload>())
        };
        dest.extend_from_slice(bytes);
    }

    #[inline(always)]
    fn read_payload(src: &Vec<u8>, index: usize) -> Self::Payload {
        let payload_index = index + Self::PAYLOAD_OFFSET;
        unsafe {
            let ptr = src.as_ptr().add(payload_index).cast::<Self::Payload>();
            return ptr.read();
        }
    }
}

pub struct IAdd;
impl Instruction for IAdd {
    const ID: u8 = 0;
}
impl IPacked for IAdd {
    type Payload = u16; 
}

pub struct ISub;
impl Instruction for ISub {
    const ID: u8 = 1;
}
impl IPacked for ISub {
    type Payload = u16; 
}

pub struct IRight;
impl Instruction for IRight {
    const ID: u8 = 2;
}
impl IPacked for IRight {
    type Payload = u16; 
}

pub struct ILeft;
impl Instruction for ILeft {
    const ID: u8 = 3;
}
impl IPacked for ILeft {
    type Payload = u16; 
}

pub struct ILoop;
impl Instruction for ILoop {
    const ID: u8 = 4;
}
impl IPacked for ILoop {
    type Payload = u16; 
}

pub struct IEndLoop;
impl Instruction for IEndLoop {
    const ID: u8 = 5;
}
impl IPacked for IEndLoop {
    type Payload = u16; 
}

pub struct IIn;
impl Instruction for IIn {
    const ID: u8 = 6;
}
impl IConst for IIn {}

pub struct IOut;
impl Instruction for IOut {
    const ID: u8 = 7;
}
impl IConst for IOut {}

pub struct INone;
impl Instruction for INone {
    const ID: u8 = 8;
}
impl IConst for INone {}

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
    fn alloc<I: IConst>(&mut self) -> TypedIndexId<'id> {
        let offset: usize = self.stream.len();
        I::alloc(&mut self.stream);

        return TypedIndexId { index: offset, typed_id: I::ID, _id: PhantomData }
    }

    #[inline]
    fn alloc_packed<I: IPacked>(&mut self, payload: I::Payload) -> TypedIndexId<'id> {
        let offset: usize = self.stream.len();
        I::alloc(&mut self.stream, payload);

        return TypedIndexId { index: offset, typed_id: I::ID, _id: PhantomData }
    }

    #[inline]
    fn alloc_or_increment<I: IPacked>(&mut self, id: TypedIndexId<'id>) -> TypedIndexId<'id> 
        where I::Payload: Add<Output = I::Payload> + AddAssign<I::Payload> + From<u8> {
        // where Packer::IValue: Add<Output = Packer::IValue> + From<u8> {
        if id.typed_id == I::ID {
            let offset = id.index + I::PAYLOAD_OFFSET;
            unsafe {
                // SAFETY: Safety will have to guaranteed by macro codegen
                // but currently still testing different methods
                let ptr = self.stream.as_mut_ptr().add(offset).cast::<I::Payload>();
                ptr.write(ptr.read() + I::Payload::from(1));
                // *ptr.as_mut_unchecked() += I::Payload::from(1);
            }
            return id
        } else {
            return self.alloc_packed::<I>(I::Payload::from(1))
        }
    }

    #[inline]
    fn write_payload<I: IPacked>(&mut self, id: IndexId<'id>, payload: I::Payload) {
        let offset = id.index + I::PAYLOAD_OFFSET;
        unsafe {
            let ptr = self.stream.as_mut_ptr().add(offset).cast::<I::Payload>();
            ptr.write(payload);
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

const SENTINAL_OFFSET: usize = 4;
impl IRBuilderTrait for ByteStreamBuilder {
    type Elem = u8;

    #[inline]
    fn new() -> Self {
        let mut new_ctx = ByteStreamCtx::new();
        new_ctx.stream.reserve_exact(4096);
        // new_ctx.stream.reserve_exact(400000);

        let new_head = new_ctx.alloc::<INone>();
        for _ in 1..SENTINAL_OFFSET {
            new_ctx.alloc::<INone>();
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