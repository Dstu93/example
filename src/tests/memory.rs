use crate::backend::memory::{Heap, MemUnit, AllocError};

#[test]
fn heap_alloc_retrieve_test() {
    let mut heap= Heap::new(3);
    let ptr_3= heap.allocate(3).unwrap();
    let ptr_2 = heap.allocate(2).unwrap();
    let ptr_5 = heap.allocate(5).unwrap();

    let ptr_6 = heap.allocate(6);

    assert_eq!(ptr_6,Err(AllocError::OutOfMemory));
    assert_eq!(Some(&mut 3),heap.retrieve(&ptr_3));
    assert_eq!(Some(&mut 2),heap.retrieve(&ptr_2));
    assert_eq!(Some(&mut 5),heap.retrieve(&ptr_5));
}

#[test]
fn heap_reinsert_test() {
    let mut heap= Heap::new(3);
    let _ptr_3= heap.allocate(3).unwrap();
    let ptr_2 = heap.allocate(2).unwrap();
    let _ptr_5 = heap.allocate(5).unwrap();

    let ptr_6 = heap.allocate(6);
    assert_eq!(ptr_6,Err(AllocError::OutOfMemory));

    //free the 2 and we should be able to insert a new item
    heap.free(ptr_2);

    let ptr_10 = heap.allocate(10).expect("could not reallocate previously used memory");
    assert_eq!(Some(&mut 10),heap.retrieve(&ptr_10));
}
