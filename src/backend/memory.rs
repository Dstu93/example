use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

type Generation<T> = RwLock<HashMap<HeapPtr,T>>;

/// Heap where objects can be stored
/// and the objects without an pointer will be freed automatic
/// by the Garbage Collector at Runtime
pub struct GCHeap{

}


#[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Hash,Debug)]
pub struct HeapPtr {
    ptr: Rc<u32>,
}

impl HeapPtr {
    fn new(n: u32) -> Self{
        HeapPtr{ptr: Rc::new(n)}
    }
    fn next(&self) -> Self{
        HeapPtr::new(*self.ptr)
    }
    pub fn invalidate(self){}
}

struct Heap<T> {
    eden: Generation<T>,
    young: Generation<T>,
    old: Generation<T>,
}

impl<T> Heap<T> {
    pub fn alloc(v: T) -> HeapPtr{
        HeapPtr::new(32)
    }
}

struct GC<T>{
    eden: Generation<T>,
    young: Generation<T>,
    old: Generation<T>,
}
