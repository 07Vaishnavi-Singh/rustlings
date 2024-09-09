#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();
    println!("{data}");
}

fn main() {
    let mut data = "Rust is great!".to_string();
    let c = get_char(&data);
    println!("The last character is '{c}'");
    string_uppercase(&mut data);
}
