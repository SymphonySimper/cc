pub fn find132pattern(x: i32, y: i32) -> bool {
    x > y
}

#[test]
fn test_1() {
    assert!(!find132pattern(10, 11));
}

#[test]
fn test_2() {
    assert!(find132pattern(100, 11));
}

#[test]
fn test_3() {
    assert!(!find132pattern(1, 1));
}
