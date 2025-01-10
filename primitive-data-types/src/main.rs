// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: Unsigned integers
// f32, f64: Floating-point numbers
// bool: Boolean values
// char: Unicode characters

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Size of i32: {} bytes", std::mem::size_of_val(&x));
    println!("Unsigned integer: {}", y);

    // i32 (32 bits) and i64 (64bits)
    // range: i32 - -2^31 to 2^31 - 1   
    // range: i64 - -2^63 to 2^63 - 1

    // u32 (32 bits) and u64 (64bits)
    // range: u32 - 0 to 2^32 - 1
    // range: u64 - 0 to 2^64 - 1

    // f32 (32 bits) and f64 (64bits)
    // range: f32 - 1.2e-38 to 3.4e38
    // range: f64 - 2.3e-308 to 1.7e308
    let pi: f64 = 3.14;
    println!("Pi: {}", pi);

    // bool: true or false
    let is_snowing: bool = true;
    println!("Is snowing: {}", is_snowing);

    // char: Unicode characters
    let letter: char = 'A';
    println!("Letter: {}", letter);
    println!("Size of char: {} bytes", std::mem::size_of_val(&letter));
}
