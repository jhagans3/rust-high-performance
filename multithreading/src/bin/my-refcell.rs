use std::cell::RefCell;
use std::collections::HashMap;

// refcell is not sync 
// If it's borrowed for read, more read-only borrows can be generated
// with the `borrow()` method, but no mutable borrow can be done.
// If it's mutably borrowed with the `borrow_mut()` method, you will not
// be able to borrow it mutably or non-mutably.
// These two methods will check the current borrow status at runtime,
//  not at compile time, which is standard for Rust rules, 
// and panic if the current state is not correct. 
// They have non-panicking alternatives named try_borrow() and try_borrow_mut()

fn main() {
    let hm = HashMap::new();
    let my_cell = RefCell::new(hm);
    println!("Initial refcell value: {:?}", my_cell.borrow());

    my_cell.borrow_mut().insert("test_key", "test_value");
    println!("Final refcell value: {:?}", my_cell.borrow());
}

// output
// Initial refcell value: {}
// Final refcell value: {"test_key": "test_value"}