use crate::memory::Memory;
use crate::ptr_ops::AsNonNull;
use crate::allocator::{
    AllocHeader, AllocObject, AllocTypeId, ArraySize, Mark, SizeClass,
};
use crate::raw_ptr::RawPtr;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypeList {
    ArrayBackingBytes,
    ArrayOpcode,
    ArrayU8,
    ArrayU16,
    ArrayU32,
    ByteCode,
    CallFrameList,
    Dict,
    Function,
    InstructionStream,
    List,
    NumberObject,
    Pair,
    Symbol,
    Text,
    Thread,
    Upvalue,
}

impl AllocTypeId for TypeList {}

pub struct ObjectHeader {
    mark: Mark,
    size_class: SizeClass,
    type_id: TypeList,
    size_bytes: u32,
}

impl ObjectHeader {
    pub unsafe fn get_object_fatptr(&self) -> FatPtr {
        let ptr_to_self = self.non_null_ptr();
        let object_addr = Memory::get_object(ptr_to_self);

        match self.type_id {

            _ => panic!("Invalid ObjectHeader type tag {:?}!", self.type_id),
        }
    }
}

impl AsNonNull for ObjectHeader {}

impl AllocHeader for ObjectHeader {
    type TypeId = TypeList;

    fn new<O: AllocObject<Self::TypeId>>(
        size: u32,
        size_class: SizeClass,
        mark: Mark,
    ) -> ObjectHeader {
        ObjectHeader {
            mark,
            size_class,
            type_id: O::TYPE_ID,
            size_bytes: size,
        }
    }

    fn new_array(size: ArraySize, size_class: SizeClass, mark: Mark) -> ObjectHeader {
        ObjectHeader {
            mark,
            size_class,
            type_id: TypeList::ArrayBackingBytes,
            size_bytes: size as u32,
        }
    }

    fn mark(&mut self) {
        self.mark = Mark::Marked;
    }

    fn is_marked(&self) -> bool {
        self.mark == Mark::Marked
    }

    fn size_class(&self) -> SizeClass {
        self.size_class
    }

    fn size(&self) -> u32 {
        self.size_bytes
    }

    fn type_id(&self) -> TypeList {
        self.type_id
    }
}

macro_rules! declare_allocobject {
    ($T:ty, $I:tt) => {
        impl AllocObject<TypeList> for $T {
            const TYPE_ID: TypeList = TypeList::$I;
        }
    };
}
