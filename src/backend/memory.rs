use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use std::collections::BTreeMap;

/// Heap where objects can be stored
/// and the objects without an pointer will be freed automatic
/// by the Garbage Collector at Runtime
pub struct GCHeap<T> {
    heap: Heap<T>,
    gc: GarbageCollector,
}

impl <T>GCHeap<T> {

    pub fn new(heap_size: u32) -> Self{
        GCHeap{heap: Heap::new(heap_size),gc: GarbageCollector::new()}
    }
    pub fn collect(&mut self) {}
    pub fn alloc(&mut self,v: T) -> Result<Ptr,AllocError>{
        //TODO
        Err(AllocError::OutOfMemory)
    }
}


#[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Hash,Debug)]
pub struct Ptr {
    ptr: Rc<u32>,
}

impl Ptr {

    fn new(n: u32) -> Self{
        Ptr {ptr: Rc::new(n)}
    }

    fn next(&self) -> Self{
        Ptr::new(*self.ptr +1)
    }

    pub fn invalidate(self){}

    fn ref_count(&self) -> usize{
        Rc::strong_count(&self.ptr)
    }
}

struct Heap<T> {
    inner: BTreeMap<Ptr,T>,
    next_address: u32,
    size: u32,
}

impl<T> Heap<T> {
    pub fn new(size: u32) -> Self{
        Heap{inner: BTreeMap::new(),next_address: 0,size}
    }
    pub fn alloc(&mut self,v: T) -> Result<Ptr,AllocError>{
        if self.inner.len() == (self.size as usize) {
            return Err(AllocError::OutOfMemory);
        }
        if self.next_address == ::std::u32::MAX {
            return Err(AllocError::OutOfAddressSpace);
        }
        //TODO build Ptr, check if is already on heap
        Ok(Ptr::new(32))
    }

    pub fn remove(&mut self, ptr: Ptr){
        drop(self.inner.remove(&ptr));
    }
}

struct GarbageCollector {

}

impl GarbageCollector {

    pub fn new() -> Self{
        GarbageCollector{}
    }
}


/// Enumeration of all errors that can occur when allocating new memory
#[derive(PartialOrd, PartialEq,Copy, Clone,Ord, Eq,Debug,Hash)]
enum AllocError {
    /// The Heap is full and no extra space can be allocated anymore
    OutOfMemory,
    /// The number of addresses is exhausted
    OutOfAddressSpace,
}

