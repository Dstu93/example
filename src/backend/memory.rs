
/// Memmory Management unit for storing Objects, retrieving them and and freeing them.
pub trait MemUnit<T> {
    fn allocate(&mut self, obj: T) -> Result<Ptr,AllocError>;
    fn retrieve(&mut self, ptr: &Ptr) -> Option<&mut T>;
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
                self.heap.len() - 1
            },
            Some((idx,_)) => {idx},
        };

        self.heap.insert(index,Some(obj));
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

}
