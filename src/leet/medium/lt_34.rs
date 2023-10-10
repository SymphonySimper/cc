// Find First and Last Position of Element in Sorted Array

use std::collections::HashMap;
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut range: HashMap<i32, Vec<i32>> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        range
            .entry(*num)
            .and_modify(|v| v.push(i as i32))
            .or_insert(vec![i as i32]);
    }

    if let Some(val) = range.get(&target) {
        if val.len() > 1 {
            vec![val[0], *val.last().unwrap()]
        } else {
            vec![val[0], val[0]]
        }
    } else {
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), [3, 4]);
    }

    #[test]
    fn test_2() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), [-1, -1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(search_range(vec![], 0), [-1, -1]);
    }

    #[test]
    fn test_4() {
        assert_eq!(search_range(vec![1], 0), [-1, -1]);
    }

    #[test]
    fn test_5() {
        assert_eq!(search_range(vec![1], 1), [0, 0]);
    }

    #[test]
    fn test_6() {
        assert_eq!(search_range(vec![2, 2], 3), [-1, -1]);
    }

    #[test]
    fn test_7() {
        assert_eq!(search_range(vec![2, 2], 1), [-1, -1]);
    }

    #[test]
    fn test_8() {
        assert_eq!(search_range(vec![1, 4], 4), [1, 1]);
    }

    #[test]
    fn test_9() {
        assert_eq!(search_range(vec![1, 2, 3], 1), [0, 0]);
    }
}
