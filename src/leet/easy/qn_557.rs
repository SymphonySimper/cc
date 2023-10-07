// Reverse words in string 3

pub fn reverse_words(s: String) -> String {
    s.chars()
        .rev()
        .collect::<String>()
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc"
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse_words("God Ding".to_string()), "doG gniD");
    }
}
