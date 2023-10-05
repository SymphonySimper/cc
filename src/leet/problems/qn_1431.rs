// Kids With the Greatest Number of Candies

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    candies
        .iter()
        .map(|candy| (candy + extra_candies) >= max)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }
}
