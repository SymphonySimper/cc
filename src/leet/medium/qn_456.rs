// 132 pattern

pub fn find132pattern(nums: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    let mut third = i32::MIN;

    for &num in nums.iter().rev() {
        if num < third {
            return true;
        }

        while let Some(&top) = stack.last() {
            if top < num {
                third = stack.pop().unwrap();
            } else {
                break;
            }
        }
        stack.push(num);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert!(!find132pattern(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_2() {
        assert!(find132pattern(vec![3, 1, 4, 2]));
    }

    #[test]
    fn test_3() {
        assert!(find132pattern(vec![-1, 3, 2, 0]));
    }
}
