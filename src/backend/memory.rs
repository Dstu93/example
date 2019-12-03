
/// Memmory Management unit for storing Objects, retrieving them and and freeing them.
pub trait MemUnit<T> {
    fn allocate(&mut self, obj: T) -> Result<Ptr,AllocError>;
    fn retrieve(&mut self, ptr: &Ptr) -> Option<&mut T>;
    fn free(&mut self, ptr: Ptr);
}

/// Enumeration of all errors that can occur when allocating new memory
#[derive(PartialOrd, PartialEq,Copy, Clone,Ord, Eq,Debug,Hash)]
pub enum AllocError {
    /// The Heap is full and no extra space can be allocated anymore
    OutOfMemory,
}

#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Debug,Hash)]
pub struct Ptr {
    idx: usize,
}

impl Ptr {
    fn new(idx: usize) -> Ptr{
        Ptr{idx}
    }
}

/// Heap implementation which uses a vector with Option<T>.
/// If some object will removed/deleted it will replaced by None.
/// That guarantees us stable indices but has a memory overhead
pub struct Heap<T> {
    heap: Vec<Option<T>>,
    max_size: usize,
}

impl<T> Heap<T> {

    pub fn new(heap_size: usize) -> Self {
        Heap{heap: Vec::with_capacity(heap_size ),max_size: heap_size}
    }

}

impl <T> MemUnit<T> for Heap<T> {

    fn allocate(&mut self, obj: T) -> Result<Ptr, AllocError> {

        // Search for the next free position on our 'heap'
        let free = self.heap
            .iter()
            .filter(|e| e.is_none())
            .enumerate()
            .next();

        // if there is none free index and our heap cant grow we return a OOM
        if free.is_none() && (self.heap.len() >= self.max_size)  {
            return Err(AllocError::OutOfMemory);
        }

        let index = match free {
            None => {
                //We add push it, our heap get a resize and
                self.heap.push(Some(obj));
                self.heap.len() - 1
            },
            Some((idx,_)) => {
                // we swap our new object to the position in the vector
                let mut free_container = self.heap.get(idx).unwrap();
                let new_obj = Some(obj);
                std::mem::swap(&mut free_container, &mut &new_obj);
                idx
            },
        };

        let pointer = Ptr::new(index);
        Ok(pointer)
    }

    fn retrieve(&mut self, ptr: &Ptr) -> Option<&mut T> {
        let obj = self.heap.get_mut(ptr.idx);
        match obj {
            None => {None},
            Some(v) => v.as_mut(),
        }
    }

    fn free(&mut self, ptr: Ptr) {
        let mut to_free = self.heap.get(ptr.idx).expect("invalid pointer");
        let mut empty_bucket: &Option<T> = &None;
        std::mem::swap(&mut to_free,&mut empty_bucket);
    }
}
