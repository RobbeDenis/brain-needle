use std::marker::PhantomData;
use std::cell::Cell;
use crate::bnintermediate::*;
use crate::bnerror::BNError;

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

pub struct ByteStream {
    ctx: ByteStreamCtx<'static>,
}

impl ByteStream {
    fn enter<F, R>(&mut self, f: F) -> R
        where F: for<'id> FnOnce(&mut ByteStreamCtx<'id>) -> R
    {
        f(&mut self.ctx)
    }
}

pub struct ByteStreamCtx<'id> {
    stream: Vec<u8>,
    _id: Id<'id>,
}

impl<'id> ByteStreamCtx<'id> {
    #[inline]
    fn new() -> Self {
        return Self { 
            stream: std::vec::Vec::new(),
            _id: PhantomData,
        };
    }

    fn alloc<IValue: Instruction>(&mut self) -> IndexId<'id> {
        let offset: usize = self.stream.len();
        self.stream.push(IValue::ID);

        return IndexId { index: offset, _id: PhantomData }
    }

    fn alloc_packed<Packer: InstructionPacker>(&mut self, value: Packer::IValue) -> IndexId<'id> {
        let offset: usize = self.stream.len();
        self.stream.push(Packer::ID);
        Packer::pack_value(&mut self.stream, value);

        return IndexId { index: offset, _id: PhantomData }
    }

    fn at(&self, id: IndexId<'id>) -> u8 {
        self.stream[id.index]
    }

    fn get_mut(&mut self, id: IndexId<'id>) -> &mut u8 {
        &mut self.stream[id.index]
    }

    fn index_to_id(&self, index: usize) -> IndexId<'id> {
        IndexId {
            index: index,
            _id: PhantomData,
        }
    }
}

pub struct ByteStreamBuilder {
    ctx: ByteStreamCtx<'static>,
}

impl IRBuilderTrait for ByteStreamBuilder {
    type Elem = u8;

    #[inline]
    fn new() -> Self {
        return Self { 
            ctx: ByteStreamCtx::new(),
        };
    }

    #[inline]
    fn enter_token(&mut self, token: BNToken) -> Result<(), BNError> {
        match token {
            BNToken::Add => { self.ctx.alloc_packed::<IAdd>(1); },
            BNToken::Sub => { self.ctx.alloc_packed::<ISub>(1); },
            BNToken::Right => { self.ctx.alloc_packed::<IRight>(1); },
            BNToken::Left => { self.ctx.alloc_packed::<ISub>(1); },
            BNToken::In => { self.ctx.alloc::<IIn>(); },
            BNToken::Out => { self.ctx.alloc::<IOut>(); },
            BNToken::Loop => {
                self.ctx.alloc_packed::<ILoop>(1);
                // let value: IValue = 32;
                // self.ctx.alloc_id_slice(&value.to_ne_bytes(), Inst::Loop(0).discriminant());
            },
            BNToken::EndLoop => {
                self.ctx.alloc_packed::<IEndLoop>(1);
                // let value: IValue = 64;
                // self.ctx.alloc_id_slice(&value.to_ne_bytes(), Inst::EndLoop(0).discriminant());
            },
            BNToken::None => { }
        }
        return Ok(());
    }

    #[inline]
    fn finalize(self) -> Result<Vec<u8>, BNError> {
        return Ok(self.ctx.stream.into_iter().collect());
    }
}

mod bnstream_test {
    use super::*;

    #[test]
    fn test() {
        // let mut stream = ByteStream{ ctx: ByteStreamCtx::new() };
        let mut builder = ByteStreamBuilder::new();

        builder.enter_token(BNToken::Add).unwrap();
        builder.enter_token(BNToken::Sub).unwrap();
        builder.enter_token(BNToken::Right).unwrap();
        builder.enter_token(BNToken::Left).unwrap();
        builder.enter_token(BNToken::Loop).unwrap();
        builder.enter_token(BNToken::EndLoop).unwrap();
        builder.enter_token(BNToken::In).unwrap();
        builder.enter_token(BNToken::Out).unwrap();

        let stream = builder.finalize().unwrap();

        println!("{:?}", stream);
    }
}