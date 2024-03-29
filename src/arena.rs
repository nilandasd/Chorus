use std::ptr::NonNull;

use crate::allocator::{
    AllocError, AllocHeader, AllocObject, AllocRaw, ArraySize, Mark, SizeClass };
use crate::heap::ImmixHeap;
use crate::raw_ptr::RawPtr;

use crate::header::TypeList;

pub struct ArenaHeader {}

impl AllocHeader for ArenaHeader {
    type TypeId = TypeList;

    fn new<O: AllocObject<Self::TypeId>>(
        _size: u32,
        _size_class: SizeClass,
        _mark: Mark,
    ) -> ArenaHeader {
        ArenaHeader {}
    }

    fn new_array(_size: ArraySize, _size_class: SizeClass, _mark: Mark) -> ArenaHeader {
        ArenaHeader {}
    }

    fn mark(&mut self) {}

    fn is_marked(&self) -> bool {
        true
    }

    fn size_class(&self) -> SizeClass {
        SizeClass::Small
    }

    fn size(&self) -> u32 {
        1
    }

    fn type_id(&self) -> TypeList {
        TypeList::Symbol
    }
}

pub struct Arena {
    heap: ImmixHeap<ArenaHeader>,
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            heap: ImmixHeap::new(),
        }
    }
}

impl AllocRaw for Arena {
    type Header = ArenaHeader;

    // ANCHOR: DefArenaAlloc
    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
    where
        T: AllocObject<TypeList>,
    {
        self.heap.alloc(object)
    }
    // ANCHOR_END: DefArenaAlloc

    fn alloc_array(&self, _size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError> {
        unimplemented!()
    }

    fn get_header(_object: NonNull<()>) -> NonNull<Self::Header> {
        unimplemented!()
    }

    fn get_object(_header: NonNull<Self::Header>) -> NonNull<()> {
        unimplemented!()
    }
}
