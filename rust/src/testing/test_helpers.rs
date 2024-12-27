#[macro_export]
macro_rules! assert_aoc {
    ($result:expr, $expected:expr) => {
        assert_eq!($result, $expected, "Results doesn't match")
    }
}
