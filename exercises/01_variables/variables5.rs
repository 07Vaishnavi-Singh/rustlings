fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}

// caoncept of variable shadowing - The key concept here is variable shadowing. In Rust, you can declare a new variable with the same name as a previous variable. This creates a new variable that shadows the previous one, effectively hiding it.
// Shadowing allows you to:

// Reuse the same variable name
// Change the type of the variable (unlike mut which only allows changing the value while keeping the same type)

// In this case, the first number is a string, and the second number is an integer. They're actually two different variables that happen to have the same name, with different scopes.