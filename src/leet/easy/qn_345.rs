//  Reverse Vowels of a String

use std::collections::HashSet;

pub fn reverse_vowels(s: String) -> String {
    let vowels = HashSet::from(['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u']);
    let mut s_vowels: Vec<char> = s.chars().filter(|c| vowels.get(c).is_some()).collect();

    s.chars()
        .map(|c| {
            if vowels.get(&c).is_some() {
                s_vowels.pop().unwrap()
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse_vowels("hello".to_string()), "holle");
    }
    #[test]
    fn test_2() {
        assert_eq!(reverse_vowels("leetcode".to_string()), "leotcede");
    }
}
