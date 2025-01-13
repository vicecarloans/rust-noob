// Shadowing is the ability to declare a new variable with the same name as an existing variable

// The first variable is shadowed by the second variable

// Shadowing is not the same as marking variable as mut
fn main() {
    let x: i32 = 5;

    // Shadowing the variable x. This is what the compiler will see
    let x = x + 1;

    {
        // Shadowing the variable x again
        let x = x * 2;
        println!("The inner scope value of x is {}", x);
    }
   

    println!("The value of x is {}", x);
}