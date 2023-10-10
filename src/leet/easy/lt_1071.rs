// Greatest common divisor of strings

// Greatest common divisor using Eculidians formula
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if format!("{str1}{str2}") == format!("{str2}{str1}") {
        return str1[..gcd(str1.len(), str2.len())].to_string();
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC"
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB"
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(gcd_of_strings("LEET".to_string(), "CODE".to_string()), "");
    }

    #[test]
    fn test_4() {
        assert_eq!(
            gcd_of_strings("ABABABAB".to_string(), "ABAB".to_string()),
            "ABAB"
        );
    }
}
