fn main() {
    let original_string = String::from("Hello, Rust!");

    // Move the ownership of `original_string` to `new_string`
    let new_string = original_string;

    // Trying to use `original_string` after the move will result in a compilation error
    println!("Original string: {}", original_string);
}
