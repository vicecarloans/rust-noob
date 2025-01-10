// Compound Data Types
// Arrays, Slices, Tuples, and Strings (slice string)

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    println!("Size of array: {} bytes", std::mem::size_of_val(&arr));
    println!("First element: {}", arr[0]);

    //Tuples
    let human: (String, i32,bool) = ("Alice".to_string(), 30, true);
    println!("Human Tuple: {:?}", human);
    println!("Name: {}", human.0);
    println!("Age: {}", human.1);
    println!("Is Human: {}", human.2);

    let mix_tuple = ("Kratos", 30, true, [1,2,3,4,5]);
    println!("Mix tuple: {:?}", mix_tuple);

    // Slices
    let number_slices: &[i32] = &[1,2,3,4,5];

    println!("Number slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Dog", "Cat", "Bird", "Fish", "Snake"];
    println!("Animal slices: {:?}", animal_slices);

    let books_slices: &[String] = &["The Great Gatsby".to_string(), "To Kill a Mockingbird".to_string(), "1984".to_string()];
    println!("Books slices: {:?}", books_slices);

    // Strings vs String Slices (&str)
    // Strings [ growable, mutable, owned string type ]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");

    // B- &str (String slice) reference to a string. This is good for memory efficiency. Used when you want to work with string data without owning it. 
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];

    println!("Slice Value: {}", slice);

}
