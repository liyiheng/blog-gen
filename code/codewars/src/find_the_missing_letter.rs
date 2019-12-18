fn find_missing_letter(chars: &[char]) -> char {
    for i in 1..chars.len() {
        if chars[i] as u8 != chars[i - 1] as u8 + 1 {
            return (chars[i - 1] as u8 + 1) as char;
        }
    }
    chars.last().map(|c| (*c as u8 + 1) as char).unwrap()
}

//#[test]
fn example_tests() {
    assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
    assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
}
