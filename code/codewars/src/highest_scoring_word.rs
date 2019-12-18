fn high(input: &str) -> &str {
    let segs: Vec<&str> = input.split(' ').collect();
    if segs.is_empty() {
        return "";
    }
    let mut highest_score = 0;
    let mut highest_index = 0;
    for (i, seg) in segs.iter().enumerate() {
        let score: i32 = seg.chars().map(|c| c as i32 - 'a' as i32 + 1).sum();
        if score > highest_score {
            highest_score = score;
            highest_index = i;
        }
    }
    segs.get(highest_index).unwrap()
}

//#[test]
fn test_basic() {
    assert_eq!(high("man i need a taxi up to ubud"), "taxi");
    assert_eq!(high("what time are we climbing up the volcano"), "volcano");
    assert_eq!(high("take me to semynak"), "semynak");
    assert_eq!(high("massage yes massage yes massage"), "massage");
    assert_eq!(high("take two bintang and a dance please"), "bintang");
    assert_eq!(high("aaa b"), "aaa");
    assert_eq!(high(""), "");
}
