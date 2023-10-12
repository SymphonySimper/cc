// Remove Duplicate Letters

pub fn remove_duplicate_letters(s: String) -> String {
    use std::collections::{HashMap, HashSet};
    let mut occur = HashMap::new();
    let mut uniq = HashSet::new();
    let mut result = Vec::new();

    for (i, c) in s.chars().enumerate() {
        occur.insert(c, i);
    }

    for (i, c) in s.chars().enumerate() {
        if !uniq.contains(&c) {
            while let Some(&top) = result.last() {
                if c < top && &i < occur.get(&top).unwrap() {
                    uniq.remove(&result.pop().unwrap());
                } else {
                    break;
                }
            }

            uniq.insert(c);
            result.push(c);
        }
    }

    result.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(remove_duplicate_letters("bcabc".to_string()), "abc");
    }

    #[test]
    fn test_2() {
        assert_eq!(remove_duplicate_letters("cbacdcbc".to_string()), "acdb");
    }
}
