// Number of good pairs

use std::collections::HashMap;

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    let mut total_good_pairs = 0;

    for num in nums {
        *count.entry(num).or_insert(0) += 1;
    }

    for (_, c) in count {
        if c > 1 {
            total_good_pairs += c * (c - 1) / 2;
        }
    }

    total_good_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
    }
    #[test]
    fn test_3() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
