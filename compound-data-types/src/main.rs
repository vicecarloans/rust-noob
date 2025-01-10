// Compound Data Types
// Arrays, Slices, Tuples, and Strings (slice string)

fn main() {
    let arr [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    println!("Size of array: {} bytes", std::mem::size_of_val(&arr));
}