// Power of Two
pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && n & (n - 1) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(is_power_of_two(1))
    }

    #[test]
    fn test2() {
        assert!(is_power_of_two(16))
    }

    #[test]
    fn test3() {
        assert!(!is_power_of_two(3))
    }
}
