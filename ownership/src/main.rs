// Ownership Rule
// 1 - Each value in Rust has a variable that's called its owner
// 2 - There can only be one owner at a time
// 3 - When the owner goes out of scope, the value will be dropped

// fn main() {
//     let s1 = String::from("RUST");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// 2 - There can only be one owner at a time

// fn main() {
//     let s1 = String::from("RUST");
//     // s1 no longer own the string, s2 does
//     let s2 = s1;

//     // borrow of moved value: `s1`
//     println!("s1: {}, s2: {}", s1, s2);
// }

// 3 - When the owner goes out of scope, the value will be dropped

fn main() {
    let s1 = String::from("RUST");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// s1 goes out of scope, and the memory is freed

fn calculate_length(s: &String) -> usize {
    s.len()
}