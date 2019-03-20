//use std::rc::Rc;
//use std::collections::BTreeMap;
//use std::iter::Cycle;
//
///// Heap where objects can be stored
///// and the objects without an pointer will be freed automatic
///// by the Garbage Collector at Runtime
//pub struct GCHeap<T> {
//    young: BTreeMap<u32,(u8,T)>,
//    old: BTreeMap<u32,(u8,T)>,
//    perm: BTreeMap<u32,T>,
//    next_address: Cycle<u32>,
//}
//
//impl <T>GCHeap<T> {
//
//    pub fn new() -> Self{
//        GCHeap{
//            young: BTreeMap::new(),
//            old: BTreeMap::new(),
//            perm: BTreeMap::new(),
//            next_address: Cycle::,
//        }
//    }
//
//    pub fn minor_collect(&mut self) {
//        self.young = self.young.into_iter().
//            filter(|e| e.0.ref_count() == 1 as usize)
//            .collect();
//        self.young.iter_mut().for_each(|e| (e.1).0 += 1);
//    }
//
//    pub fn collect(&mut self) {
//        self.young = self.young.into_iter().filter(|e| e.0.ref_count() == 1 as usize).collect();
//        self.old = self.old.into_iter().filter(|e| e.0.ref_count() == 1 as usize).collect();
//    }
//
//    pub fn alloc(&mut self, v: T) -> Result<Ptr,AllocError>{
//        if self.last_address == ::std::u32::MAX {
//            // TODO start Garbage Collection and search for the next free address
//            return Err(AllocError::OutOfAddressSpace);
//        }
//
//        let next = self.last_address + 1;
//        let ptr = Ptr::new(next, HeapGen::Young);
//        self.last_address = next;
//        self.young.insert(ptr.clone(),(0,v));
//        Ok(ptr)
//    }
//}
//
//
//#[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Hash,Debug)]
//pub struct Ptr {
//    ptr: Rc<(u32,HeapGen)>,
//}
//
//impl Ptr {
//
//    fn new(n: u32, gen: HeapGen) -> Self{
//        Ptr {ptr: Rc::new((n,gen))
//    }
//
//    pub fn invalidate(self){}
//
//    fn ref_count(&self) -> usize{
//        Rc::strong_count(&self.ptr)
//    }
//
//}
//
///// Enumeration of all errors that can occur when allocating new memory
//#[derive(PartialOrd, PartialEq,Copy, Clone,Ord, Eq,Debug,Hash)]
//pub enum AllocError {
//    /// The Heap is full and no extra space can be allocated anymore
//    OutOfMemory,
//    /// The number of addresses is exhausted
//    OutOfAddressSpace,
//}
//
///// Small Enum which represents different types of Heaps.
//#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Hash,Debug)]
//enum HeapGen {
//    Young,
//    Old,
//    Perm,
//}
//
