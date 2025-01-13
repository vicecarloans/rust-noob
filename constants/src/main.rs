// Constants


fn main() {
    
    let mut x: i32 = 5;
    // Constants are not allowed to be used mut 
    // const mut y: i32 = 10;

    // Constants must be init with capital letters
    const Y: i32 = 3;
    println!("The value of PI is {}", PI);

    println!("The value of Y is {}", Y);

    println!("The value of THREE_HOURS_IN_SECONDS is {}", THREE_HOURS_IN_SECONDS);
}

// You can declare a constant here with a type annotation
const PI: f64 = 3.14159;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;