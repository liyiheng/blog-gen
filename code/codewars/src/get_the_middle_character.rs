fn get_middle(s: &str) -> &str {
    let c = s.chars();
    let count = c.count();
    let half = count / 2;
    if count % 2 == 1 {
        // ascii only
        &s[half..half + 1]
    } else {
        &s[half - 1..half + 1]
    }
}

//#[test]
fn example_tests() {
    assert_eq!(get_middle("test"), "es");
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("A"), "A");
    assert_eq!(get_middle("of"), "of");
}
