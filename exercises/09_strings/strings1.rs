// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
     String::from("blue")
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
// String is s hap allocated string 
// &str is a immutable reference to a string variable 
// &str can be converted to String using .to_string() 