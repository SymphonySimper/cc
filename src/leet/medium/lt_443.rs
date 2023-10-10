// String Compression

// use std::mem;
// pub fn compress(chars: &mut Vec<char>) -> i32 {
//     let mut s = String::new();
//     let mut count = 0;
//     let mut prev: char = chars[0];
//
//     // padding
//     chars.push('0');
//
//     for &c in chars.iter() {
//         if c == prev {
//             count += 1;
//         } else {
//             s.push(prev);
//             if count > 1 {
//                 s.push_str(&count.to_string());
//             }
//             count = 1;
//         }
//
//         prev = c;
//     }
//
//     let op_len = s.len();
//     let _ = mem::replace(chars, s.as_str()[..op_len].chars().collect());
//
//     op_len as i32
// }

// use std::mem::swap;
pub fn compress(chars: &mut Vec<char>) -> i32 {
    let chars_len = chars.len();
    let mut ans = String::with_capacity(chars_len);

    let mut slow = 0;
    let mut fast = 0;

    while fast < chars_len {
        while chars[slow] == chars[fast] {
            fast += 1;

            if fast >= chars_len {
                break;
            }
        }

        ans.push(chars[slow]);
        let count = fast - slow;
        if count > 1 {
            ans.push_str(count.to_string().as_str());
        }

        slow = fast;
    }

    for (i, c) in ans.chars().enumerate() {
        chars[i] = c;
    }

    ans.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(compress(&mut chars), 6);
        assert_eq!(chars, ['a', '2', 'b', '2', 'c', '3', 'c']);
    }

    #[test]
    fn test_2() {
        let mut chars = vec!['a'];
        assert_eq!(compress(&mut chars), 1);
        assert_eq!(chars, ['a']);
    }

    #[test]
    fn test_3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(compress(&mut chars), 4);
        assert_eq!(
            chars,
            ['a', 'b', '1', '2', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',]
        );
    }
}
