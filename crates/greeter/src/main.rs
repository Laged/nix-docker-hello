use namegen;

pub fn greet_random() -> String {
    let name = namegen::generate();
    format!("Hello, {}!", name)
}

fn main() {
    let greeting = greet_random();
    println!("{}", greeting);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_random_format() {
        let greeting = greet_random();

        // Check if the greeting starts with "Hello, " and ends with "!"
        assert!(greeting.starts_with("Hello, "));
        assert!(greeting.ends_with("!"));

        // Extract the name part
        let name_part = &greeting[7..greeting.len() - 1]; // Slice out "Hello, " and "!"

        // Check if the name part contains exactly one hyphen
        let parts: Vec<&str> = name_part.split('-').collect();
        assert_eq!(parts.len(), 2, "Name does not contain exactly one hyphen");

        // Check if both parts are non-empty
        assert!(!parts[0].is_empty(), "First part of the name is empty");
        assert!(!parts[1].is_empty(), "Second part of the name is empty");
    }
}
