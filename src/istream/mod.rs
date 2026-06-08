use std::marker::PhantomData;
use std::cell::Cell;
use std::vec::Vec;
use std::ops::{Add, AddAssign};

pub mod intermediate;
pub mod codegen;
pub mod emit;
pub mod emit_interpreted;
pub mod tokenmatcher;

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

// pub struct IStream {
//     ctx: IStreamCtx<'static>,
// }

// impl IStream {
//     fn enter<F, R>(&mut self, f: F) -> R
//         where F: for<'id> FnOnce(&mut IStreamCtx<'id>) -> R
//     {
//         f(&mut self.ctx)
//     }
// }

pub struct IStreamCtx<'id> {
    stream: Vec<u8>,
    _id: Id<'id>,
}

impl<'id> IStreamCtx<'id> {
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
        if id.typed_id == I::ID {
            let offset = id.index + I::PAYLOAD_OFFSET;
            unsafe {
                // SAFETY: Safety will have to guaranteed by macro codegen
                // but currently still testing different methods
                let ptr = self.stream.as_mut_ptr().add(offset).cast::<I::Payload>();
                ptr.write(ptr.read() + I::Payload::from(1));
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