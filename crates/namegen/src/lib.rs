use names::Generator;
use std::fmt;

pub struct Name(String);
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn generate() -> Name {
    let mut generator = Generator::default();
    let random = generator.next().unwrap();
    return Name(random);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = generate();
        let hyphen_count = result.0.matches('-').count();
        assert_eq!(
            hyphen_count, 1,
            "Generated name does not contain exactly one hyphen"
        );
    }
}
