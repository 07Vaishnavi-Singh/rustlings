fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string.
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // Add " world!" to the string
    format!("{} world!", input) 
}

// mutably is not required as there is no mutatoin being done to the string
fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!"); // these are not being called mutably so it should be received mutably 
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}