fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut result = vec![];
    let get_value = |d: &Direction| match *d {
        Direction::NORTH => (0, 1),
        Direction::SOUTH => (0, -1),
        Direction::WEST => (-1, 0),
        Direction::EAST => (1, 0),
    };
    for d in arr.iter() {
        if result.is_empty() {
            result.push(*d);
            continue;
        }
        let last = result.last().unwrap();
        let (x1, y1) = get_value(last);
        let (x2, y2) = get_value(d);
        if x1 + x2 == 0 && y1 + y2 == 0 {
            result.pop();
        } else {
            result.push(*d);
        }
    }
    result
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

// #[test]
fn returns_expected() {
    use Direction::*;
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
    assert_eq!(dir_reduc(&a), [WEST]);
    let a = [NORTH, WEST, SOUTH, EAST];
    assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
}
