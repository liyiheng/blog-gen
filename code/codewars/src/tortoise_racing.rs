fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }
    let mut total_seconds = g * 3600 / (v2 - v1);
    let hour = total_seconds / 3600;
    total_seconds %= 3600;
    let min = total_seconds / 60;
    let seconds = total_seconds % 60;
    return Some(vec![hour, min, seconds]);
}

//#[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 81, 550), None);
}
