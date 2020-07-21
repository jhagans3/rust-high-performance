use std::cell::RefCell;

// RefCell is similar to Cell,
// except that it accepts non-Copy data.
// This also means that when modifying the underlying object,
// it cannot simply copy it when returning it,
// it will need to return references.
#[derive(Debug)]
struct House {
    bedrooms: u8,
}

// This is a nightly-only experimental API. (refcell_take #71395)
// impl Default for &House {
//     fn default() -> Self {
//         &House { bedrooms: 1 }
//     }
// }

fn main() {
    let my_house = House { bedrooms: 2 };
    let my_dream_house = House { bedrooms: 5 };

    let my_refcell = RefCell::new(&my_house);
    // borrow() will get a read-only borrow
    // you can have as many immutable borrows in a scope
    println!("My house has {} bedrooms.", my_refcell.borrow().bedrooms);

    // borrow_mut() will return a read-write borrow, and you will
    // only be able to have one in scope to follow the mutability rules
    *my_refcell.borrow_mut() = &my_dream_house;
    println!(
        "My new house has {} bedrooms.",
        my_refcell.borrow().bedrooms
    );

    // & used because my_house would be moved into my_refcell's fn
    let my_new_old_house = my_refcell.replace(&my_house);
    println!(
        "My house has {} bedrooms, it was better with {}",
        my_refcell.borrow().bedrooms,
        my_new_old_house.bedrooms
    );

    let my_new_refcell = RefCell::new(&my_dream_house);

    my_refcell.swap(&my_new_refcell);
    println!(
        "Yay! my current house has {} bedrooms! (my new house {})",
        my_refcell.borrow().bedrooms,
        my_new_refcell.borrow().bedrooms,
    );

    // try_borrow() and try_borrow_mut() will try to borrow the data
    // (the first read-only and the second read/write)
    // and if there are incompatible borrows present
    // they will return a Result::Err
    // so that you can handle the error without panicking
    let hold_mut_ref = my_refcell.borrow_mut();
    println!("House {:?}", *hold_mut_ref);
    println!("bedrooms {}", hold_mut_ref.bedrooms);
    match my_refcell.try_borrow_mut() {
        Ok(_) => {
            *my_refcell.borrow_mut() = &my_dream_house;
            println!("Is this broken {:?}?", my_refcell.borrow().bedrooms);
        }
        Err(_) => {
            println!("Exclusive ref is still active");
        }
    };

    // This is a nightly-only experimental API. (refcell_take #71395)
    // let my_final_house = my_refcell.take();
    // println!(
    //     "My final house has {} bedrooms, the shared one {}",
    //     my_final_house.bedrooms,
    //     my_refcell.borrow().bedrooms,
    // );
}

// Output
// My house has 2 bedrooms.
// My new house has 5 bedrooms.
// My house has 2 bedrooms, it was better with 5
// Yay! my current house has 5 bedrooms! (my new house 2)
// House House { bedrooms: 5 }
// bedrooms 5
// Exclusive ref is still active

// Note
// If you try to do a borrow_mut() after a borrow()
// in the same scope, or a borrow() after a borrow_mut(), the thread will panic.
// You must successfully borrow_mut() the RefCell<> in order to set the value (by dereferencing)
// and then simply borrow() it to retrieve the value
// Remember, any number of read-only references or exactly 1 read-write
// reference and nothing else – although for RefCell, this is enforced at run-time
// let answer = RefCell::new(0);
// let break_things = answer.borrow_mut();
// println!("The initial value is : {}", *break_things);
// *answer.borrow_mut() = 42;
// println!("The answer is : {}", answer.borrow());
