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

    pub fn new() -> Self{
        GCHeap{heap: Heap::new(),gc: GarbageCollector::new()}
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
        Ptr::new(*self.ptr)
    }

    pub fn invalidate(self){}

    pub fn ref_count(&self) -> usize{
        Rc::strong_count(&self.ptr)
    }
}

struct Heap<T> {
    inner: BTreeMap<Ptr,T>,
    next_address: u64,
    size: usize,
}

impl<T> Heap<T> {
    pub fn new(size: usize) -> Self{
        Heap{inner: BTreeMap::new(),next_address: 0,size}
    }
    pub fn alloc(&mut self,v: T) -> Result<Ptr,AllocError>{
        if  self.inner.len() == self.size || self.next_address == 2^64{
            return Err(AllocError::OutOfMemory)
        }
        //TODO build Ptr, check if is already on heap
        Ok(Ptr::new(32))
    }
}

struct GarbageCollector{

}

impl GarbageCollector {

    pub fn new() -> Self{
        GarbageCollector{}
    }
}


#[derive(PartialOrd, PartialEq,Copy, Clone,Ord, Eq,Debug,Hash)]
enum AllocError {
    OutOfMemory,
}