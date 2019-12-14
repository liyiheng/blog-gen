fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|t| !b.contains(t)).collect()
}

// Add your tests here
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    }
}
