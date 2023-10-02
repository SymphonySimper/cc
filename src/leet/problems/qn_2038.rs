// Remove Colored Pieces if Both Neighbors are the Same Color

// Just loop
// pub fn winner_of_game(colors: String) -> bool {
//     let colors: Vec<char> = colors.chars().collect();
//     let mut alice = 0;
//     let mut bob = 0;
//
//     for i in 1..colors.len() - 1 {
//         if colors[i - 1] == colors[i] && colors[i] == colors[i + 1] {
//             if colors[i] == 'A' {
//                 alice += 1;
//             } else {
//                 bob += 1;
//             }
//         }
//     }
//
//     alice > bob
// }

// Window iterator
pub fn winner_of_game(colors: String) -> bool {
    colors.as_bytes().windows(3).fold(0, |score, w| {
        score
            + if w == b"AAA" {
                1
            } else if w == b"BBB" {
                -1
            } else {
                0
            }
    }) > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(winner_of_game("AAABABB".to_string()))
    }

    #[test]
    fn test_2() {
        assert!(!winner_of_game("AA".to_string()))
    }

    #[test]
    fn test_3() {
        assert!(!winner_of_game("ABBBBBBBAAA".to_string()))
    }
}
