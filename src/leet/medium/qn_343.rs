// Integer break

pub fn integer_break(n: i32) -> i32 {
    // Here n <= 2
    if n <= 3 {
        return n - 1;
    }

    let count = n as u32 / 3;
    let remainder = n % 3;
    let pow = |c| 3u32.pow(c) as i32;

    match remainder {
        0 => pow(count),
        1 => pow(count - 1) * 4,
        _ => pow(count) * 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(integer_break(2), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(integer_break(10), 36);
    }
}
