mod tests {
    #[macro_export]
    macro_rules! assert_match {
        ($string:expr, $pattern:expr) => {
            assert!(Regex::new($pattern).unwrap().is_match($string.as_str()));
        };
    }
}
