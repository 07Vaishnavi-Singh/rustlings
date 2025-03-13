// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}

// it is important to specify the type in the variable that is accepting the value 