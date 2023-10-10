// Merge strings merge alternately

// Peekable
// pub fn merge_alternately(word1: String, word2: String) -> String {
//     let mut word1 = word1.chars().peekable();
//     let mut word2 = word2.chars().peekable();
//     let mut result = String::new();
//
//     loop {
//         if let Some(c) = word1.next() {
//             result.push(c);
//         }
//
//         if let Some(c) = word2.next() {
//             result.push(c);
//         }
//
//         if word1.peek().is_none() && word2.peek().is_none() {
//             return result;
//         }
//     }
// }

// Using iterators
pub fn merge_alternately(word1: String, word2: String) -> String {
    word1
        .chars()
        .zip(word2.chars())
        .flat_map(|(c1, c2)| [c1, c2])
        .chain(word2.chars().skip(word1.len()))
        .chain(word1.chars().skip(word2.len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr"
        )
    }
    #[test]
    fn test_2() {
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs"
        )
    }
    #[test]
    fn test_3() {
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd"
        )
    }
}
