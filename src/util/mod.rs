mod tests {
    #[macro_export]
    macro_rules! assert_match {
        ($string:expr, $pattern:expr) => {
            assert!(regex!($pattern).is_match($string));
        };
    }
}
