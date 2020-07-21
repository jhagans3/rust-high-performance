use std::cell::Cell;

// to use a Cell the contained type must be Copy
// when using the get() method, you get a copy of the current value
// and not a reference to it. That’s why you can only
// use elements implementing Copy with a Cell.
#[derive(Copy, Clone)]
struct House {
    bedrooms: u8,
}

impl Default for House {
    fn default() -> Self {
        House { bedrooms: 1 }
    }
}

// cargo run --bin shared-pointers-cell
fn main() {
    // we first create two houses
    let my_house = House { bedrooms: 2 };
    let my_dream_house = House { bedrooms: 5 };

    // we select one of them as the current one (my_house)
    let my_cell = Cell::new(my_house);
    // .get() to get a copy of the value inside
    println!("My house has {} bedrooms.", my_cell.get().bedrooms);

    // and we keep mutating the current and the new ones
    // .set() to set the value inside
    my_cell.set(my_dream_house);
    println!("My new house has {} bedrooms.", my_cell.get().bedrooms);

    // As you can see, you don’t really mutate the value inside,
    // but you replace it with another value.
    // You can either retrieve the old value or lose it.
    let my_new_old_house = my_cell.replace(my_house);
    println!(
        "My house has {} bedrooms, it was better with {}",
        my_cell.get().bedrooms,
        my_new_old_house.bedrooms
    );

    let my_new_cell = Cell::new(my_dream_house);

    my_cell.swap(&my_new_cell);
    println!(
        "Yay! my current house has {} bedrooms! (my new house {})",
        my_cell.get().bedrooms,
        my_new_cell.get().bedrooms
    );

    // use the take() method, only available for types
    // implementing the Default trait. This method will return
    // the current value, replacing it with the default value
    // .take() to get a copy of the value inside
    // AND reset the value inside to default.
    let my_final_house = my_cell.take();

    println!(
        "My final house has {} bedrooms, the shared one {}",
        my_final_house.bedrooms,
        my_cell.get().bedrooms
    );
}

// Output:
// My house has 2 bedrooms.
// My new house has 5 bedrooms.
// My house has 2 bedrooms, it was better with 5
// Yay! my current house has 5 bedrooms! (my new house 2)
// My final house has 5 bedrooms, the shared one 1
