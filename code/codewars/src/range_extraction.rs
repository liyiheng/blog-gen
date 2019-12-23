mod solution {
    struct Range {
        start: i32,
        end: i32,
    }

    pub fn range_extraction(a: &[i32]) -> String {
        let mut ranges = vec![];
        for i in 0..a.len() {
            let v = a[i];
            if ranges.is_empty() {
                ranges.push(Range { start: v, end: v });
                continue;
            }
            let last_end = ranges.last().unwrap().end;
            if last_end + 1 == v {
                ranges.last_mut().unwrap().end = v;
            } else {
                ranges.push(Range { start: v, end: v });
            }
        }
        ranges
            .into_iter()
            .map(|r| {
                if r.end - r.start < 2 {
                    (r.start..r.end + 1)
                        .into_iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                } else {
                    format!("{}-{}", r.start, r.end)
                }
            })
            .collect::<Vec<String>>()
            .join(",")
    }
}

//#[test]
fn example() {
    assert_eq!(
        "-6,-3-1,3-5,7-11,14,15,17-20",
        solution::range_extraction(&[
            -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
        ])
    );
    assert_eq!(
        "-3--1,2,10,15,16,18-20",
        solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
    );
}
