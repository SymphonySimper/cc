// Majority Element II

use std::collections::{HashMap, HashSet};

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut count: HashMap<i32, i32> = HashMap::new();
    let mut uniq_count: HashSet<i32> = HashSet::new();
    let nums_len: i32 = nums.len() as i32 / 3;

    for num in nums {
        let current_count = *count.entry(num).and_modify(|val| *val += 1).or_insert(1);

        if current_count > nums_len {
            uniq_count.insert(num);
        }
    }

    uniq_count.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
    }

    #[test]
    fn test_2() {
        assert_eq!(majority_element(vec![1]), vec![1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(majority_element(vec![1, 2]), vec![1, 2])
    }
}
