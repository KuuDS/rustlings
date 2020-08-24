// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!


#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! my_macro {
        ($word:expr) => {
           "Hello ".to_owned() + $word
        };
    }
    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
