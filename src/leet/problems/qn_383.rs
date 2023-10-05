use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() > magazine.len() {
        return false;
    }

    let mut magazine_count = HashMap::new();

    for c in magazine.chars() {
        magazine_count
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for c in ransom_note.chars() {
        let count = *magazine_count.get(&c).unwrap_or(&0);
        if count == 0 {
            return false;
        }

        magazine_count.insert(c, count - 1);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!can_construct("aa".to_string(), "ab".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(can_construct("aa".to_string(), "aab".to_string()));
    }
}
