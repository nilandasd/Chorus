/// VM-level memory abstraction
///
/// Defines Stack, Heap and Memory types, and a MemoryView type that gives a mutator a safe
/// view into the stack and heap.
use crate::heap::ImmixHeap;
use crate::allocator::{AllocObject, AllocRaw, ArraySize};
use crate::raw_ptr::RawPtr;

use crate::error::RuntimeError;
use crate::header::{ObjectHeader, TypeList};
use crate::ptr_ops::ScopedRef;
use crate::safe_ptr::{MutatorScope, ScopedPtr, TaggedScopedPtr};
use crate::symbol_map::SymbolMap;
use crate::tagged_ptr::{FatPtr, TaggedPtr};

/// This type describes the mutator's view into memory - the heap and symbol name/ptr lookup.
///
/// It implements `MutatorScope` such that any `TaggedScopedPtr` or `Value` instances must be lifetime-
/// limited to the lifetime of this instance using `&'scope dyn MutatorScope`;
pub struct MutatorView<'memory> {
    heap: &'memory Heap,
}

impl<'memory> MutatorView<'memory> {
    fn new(mem: &'memory Memory) -> MutatorView<'memory> {
        MutatorView { heap: &mem.heap }
    }

    /// Get a Symbol pointer from its name
    // ANCHOR: DefMutatorViewLookupSym
    pub fn lookup_sym(&self, name: &str) -> TaggedScopedPtr<'_> {
        TaggedScopedPtr::new(self, self.heap.lookup_sym(name))
    }
    // ANCHOR_END: DefMutatorViewLookupSym

    /// Write an object into the heap and return a scope-limited pointer to it
    // ANCHOR: DefMutatorViewAlloc
    pub fn alloc<T>(&self, object: T) -> Result<ScopedPtr<'_, T>, RuntimeError>
    where
        T: AllocObject<TypeList>,
    {
        Ok(ScopedPtr::new(
            self,
            self.heap.alloc(object)?.scoped_ref(self),
        ))
    }
    // ANCHOR_END: DefMutatorViewAlloc

    /// Write an object into the heap and return a scope-limited runtime-tagged pointer to it
    // ANCHOR: DefMutatorViewAllocTagged
    pub fn alloc_tagged<T>(&self, object: T) -> Result<TaggedScopedPtr<'_>, RuntimeError>
    where
        FatPtr: From<RawPtr<T>>,
        T: AllocObject<TypeList>,
    {
        Ok(TaggedScopedPtr::new(self, self.heap.alloc_tagged(object)?))
    }
    // ANCHOR_END: DefMutatorViewAllocTagged

    /// Make space for an array of bytes
    pub fn alloc_array(&self, capacity: ArraySize) -> Result<RawPtr<u8>, RuntimeError> {
        self.heap.alloc_array(capacity)
    }

    /// Return a nil-initialized runtime-tagged pointer
    pub fn nil(&self) -> TaggedScopedPtr<'_> {
        TaggedScopedPtr::new(self, TaggedPtr::nil())
    }
}

impl<'memory> MutatorScope for MutatorView<'memory> {}

/// The heap implementation
// ANCHOR: DefHeapStorage
pub type HeapStorage = ImmixHeap<ObjectHeader>;
// ANCHOR_END: DefHeapStorage

/// Heap memory types.
// ANCHOR: DefHeap
struct Heap {
    heap: HeapStorage,
    syms: SymbolMap,
}
// ANCHOR_END: DefHeap

impl Heap {
    fn new() -> Heap {
        Heap {
            heap: HeapStorage::new(),
            syms: SymbolMap::new(),
        }
    }

    /// Get a Symbol pointer from its name
    // ANCHOR: DefHeapLookupSym
    fn lookup_sym(&self, name: &str) -> TaggedPtr {
        TaggedPtr::symbol(self.syms.lookup(name))
    }
    // ANCHOR_END: DefHeapLookupSym

    /// Write an object to the heap and return the raw pointer to it
    // ANCHOR: DefHeapAlloc
    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, RuntimeError>
    where
        T: AllocObject<TypeList>,
    {
        Ok(self.heap.alloc(object)?)
    }
    // ANCHOR_END: DefHeapAlloc

    /// Write an object into the heap and return a tagged pointer to it
    // ANCHOR: DefHeapAllocTagged
    fn alloc_tagged<T>(&self, object: T) -> Result<TaggedPtr, RuntimeError>
    where
        FatPtr: From<RawPtr<T>>,
        T: AllocObject<TypeList>,
    {
        Ok(TaggedPtr::from(FatPtr::from(self.heap.alloc(object)?)))
    }
    // ANCHOR_END: DefHeapAllocTagged

    fn alloc_array(&self, capacity: ArraySize) -> Result<RawPtr<u8>, RuntimeError> {
        Ok(self.heap.alloc_array(capacity)?)
    }
}

/// Wraps a heap and provides scope-limited access to the heap
// ANCHOR: DefMemory
pub struct Memory {
    heap: Heap,
}
// ANCHOR_END: DefMemory

impl Memory {
    /// Instantiate a new memory environment
    pub fn new() -> Memory {
        Memory { heap: Heap::new() }
    }

    /// Run a mutator process
    // ANCHOR: DefMemoryMutate
    pub fn mutate<M: Mutator>(&self, m: &M, input: M::Input) -> Result<M::Output, RuntimeError> {
        let mut guard = MutatorView::new(self);
        m.run(&mut guard, input)
    }
    // ANCHOR_END: DefMemoryMutate
}

/// Defines the interface a heap-mutating type must use to be allowed access to the heap
// ANCHOR: DefMutator
pub trait Mutator: Sized {
    type Input;
    type Output;

    fn run(&self, mem: &MutatorView, input: Self::Input) -> Result<Self::Output, RuntimeError>;

    // TODO
    // function to return iterator that iterates over roots
}
// ANCHOR_END: DefMutator
