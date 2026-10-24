//! The M4 sandbox trunk: two functions and their tests, so a landing's test run costs seconds.

/// Adds two numbers.
pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

/// Subtracts `b` from `a`, or `None` when `b` is larger.
pub fn subtract(a: u64, b: u64) -> Option<u64> {
    a.checked_sub(b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn adds() {
        assert_eq!(super::add(2, 3), 5);
    }

    #[test]
    fn subtracts_and_refuses_to_go_below_zero() {
        assert_eq!(super::subtract(5, 3), Some(2));
        assert_eq!(super::subtract(3, 5), None);
    }
}
