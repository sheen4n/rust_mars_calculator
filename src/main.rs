use std::io;

fn main() {
    // this String on the heap belongs to the variable input (variable input is the owner)
    let mut input: String = String::new();
    // s takes over pointer to String::new() as new owner... so input can no longer be owner (borrowed)
    // let mut s = input;
    some_fn(&mut input);
    // not allowed to use input as ownership of pointer transferred to s in some_fn
    io::stdin().read_line(&mut input);

    let mut mars_weight: f32 = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}g", mars_weight);

    // When this scope exits, the value of input is deallocated

    // Double free (error occurs in other languages as resources are being attempted to free twice)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// passing references as parameter means borrowing, ownership not transferred
fn some_fn(s: &mut String) {
    s.push_str("a");
}

// Ownership Rules
// 1. Each value in Rust is owned by a variable
// 2. When the owner goes out of scope, the value will be deallocated.
// 3. There can only be ONE owner at a time.