fn main() {
    hello_world();
    tell_height(180);
    human_id("John", 25, 175.5);

    let x = {
        let price: i32 = 5;
        let qty: i32 = 10;

        // This is a return statement, no semicolon needed
        price * qty
    };

    println!("Result is: {}", x);

    let y = add(3, 4);
    println!("Result is: {}", y);
    println!("Value from function 'add' is: {}", add(2, 3));

    println!("BMI is: {:.2}", bmi(84.0, 1.75)); 
}

// Hoisting - can call function anywhere in your code
fn hello_world() {
    println!("Hello, World!");
}

// you can insert input values
fn tell_height(height: u32) {
    println!("Your height is {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("Name: {}, Age: {}, Height: {}", name, age, height);
}

// Function with return value
fn add(a: i32, b: i32) -> i32 {
    // This is a return statement, no semicolon needed
    a + b
}

// Expressions and Statements
// Expression: Anything that returns a value
// Statement: Anything that does not return a value

// Expression
// 5
// true & false
// add(3,4)
// ({ code block })

// Statement
// Almost all statements end with a semicolon
// Variable declartations:
// Function definitions
// Loops


fn bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

