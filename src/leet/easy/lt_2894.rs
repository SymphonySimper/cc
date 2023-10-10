// Divisible and Non-divisible Sums Difference

pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let (num1, num2) = (1..=n).fold((0, 0), |(mut num1, mut num2), num| {
        if num % m == 0 {
            num2 += num;
        } else {
            num1 += num;
        }
        (num1, num2)
    });

    num1 - num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(difference_of_sums(10, 3), 19);
    }

    #[test]
    fn test_2() {
        assert_eq!(difference_of_sums(5, 6), 15);
    }

    #[test]
    fn test_3() {
        assert_eq!(difference_of_sums(5, 1), -15);
    }
}
