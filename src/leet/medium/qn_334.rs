// Increasing Triplet Subsequence

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    nums.iter()
        .try_fold((i32::MAX, i32::MAX, 0), |(i, j, k), n| {
            if *n <= i {
                Ok((*n, j, k))
            } else if *n <= j {
                Ok((i, *n, k))
            } else {
                Err((i, j, *n))
            }
        })
        .is_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(increasing_triplet(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_2() {
        assert!(!increasing_triplet(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn test_3() {
        assert!(increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    }
}
