// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let s1 = String::from("abc");
        let s2 = String::from("abc");
        assert_eq!(s1, s2.to_string());
    }
}
