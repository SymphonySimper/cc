// Product of Array Except Self

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let nums_len = nums.len();
    let mut ans = vec![1; nums_len];
    let mut left_value = 1;
    let mut right_value = 1;
    let mut right_index = nums_len;

    for left_index in 0..nums_len {
        right_index -= 1;
        ans[left_index] *= left_value;
        ans[right_index] *= right_value;

        left_value *= nums[left_index];
        right_value *= nums[right_index];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), [24, 12, 8, 6]);
    }

    #[test]
    fn test_2() {
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);
    }

    #[test]
    fn test_3() {
        assert_eq!(product_except_self(vec![0, 0]), [0, 0]);
    }
}
