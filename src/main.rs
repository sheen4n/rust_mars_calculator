use std::io;

fn main() {
    let mut input: String = String::new();
    
    io::stdin().read_line(&mut input);
    println!("Input: {}", input);
    let mars_weight: f32 = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// passing references as parameter means borrowing, ownership not transferred

// Ownership Rules
// 1. Each value in Rust is owned by a variable
// 2. When the owner goes out of scope, the value will be deallocated.
// 3. There can only be ONE owner at a time.


// Borrowing Rules
// 1. You can have only 1 mutable borrows, or many immutable borrows at the same time