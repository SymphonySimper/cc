// Title: 283. Move Zeroes
// URL: https://leetcode.com/problems/move-zeroes

pub fn move_zeroes(nums: &mut Vec<i32>) -> Vec<i32> {
    let nums_len = nums.len();
    let mut j: i32 = -1;
    for i in 0..nums_len {
        let curr = nums[i];

        if curr == 0 {
            continue;
        }

        j += 1;
        nums.swap(i, j as usize);
    }

    nums.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            move_zeroes(&mut [0, 1, 0, 3, 12].to_vec()),
            [1, 3, 12, 0, 0]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(move_zeroes(&mut [0].to_vec()), [0]);
    }

    #[test]
    fn test_3() {
        assert_eq!(move_zeroes(&mut [0, 0, 1].to_vec()), [1, 0, 0]);
    }
}
