use std::cell::Cell;

// cargo run --bin my-cell
fn main() {
    //Note that the my_cell variable is not mutable, but the program still compiles
    let my_cell = Cell::new(0);
    println!("Initial cell value: {}", my_cell.get());

    my_cell.set(my_cell.get() + 1);
    println!("Final cell value: {}", my_cell.get());
}

// output 
// Initial cell value: 0
// Final cell value: 1