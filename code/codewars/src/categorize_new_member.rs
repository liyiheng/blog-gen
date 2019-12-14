fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.into_iter()
        .map(|(a, b)| {
            if a >= 55 && b > 7 {
                "Senior".to_owned()
            } else {
                "Open".to_owned()
            }
        })
        .collect()
}

mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(
            open_or_senior(vec![(45, 12), (55, 21), (19, -2), (104, 20)]),
            vec!["Open", "Senior", "Open", "Senior"]
        );
        assert_eq!(
            open_or_senior(vec![(3, 12), (55, 1), (91, -2), (54, 23)]),
            vec!["Open", "Open", "Open", "Open"]
        );
    }
}
