// Can place flowers

use std::iter;

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    n == 0
        || flowerbed
            .iter()
            .chain(iter::once(&0))
            .fold((1, 0), |(prev, open_pots), current| {
                if *current == 1 {
                    (0, open_pots)
                } else if prev == 2 {
                    (1, open_pots + 1)
                } else {
                    (prev + 1, open_pots)
                }
            })
            .1
            >= n
}

// pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
//     let mut flowerbed = flowerbed;
//     let mut n = n;
//
//     flowerbed.push(0);
//     flowerbed.insert(0, 0);
//     let len = flowerbed.len();
//
//     for i in 1..len - 1 {
//         let current = flowerbed[i];
//
//         if current == 0 {
//             if n != 0 {
//                 let prev = flowerbed[i - 1];
//                 let next = flowerbed[i + 1];
//
//                 if prev == 0 && next == 0 {
//                     n -= 1;
//                     flowerbed[i] = 1;
//                 }
//             } else {
//                 break;
//             }
//         }
//     }
//
//     n == 0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn test_2() {
        assert!(!can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
