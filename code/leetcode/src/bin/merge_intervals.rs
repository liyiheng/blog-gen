// Definition for an interval.
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}
impl Solution {
    pub fn merge(mut intervals: Vec<Interval>) -> Vec<Interval> {
        intervals.sort_unstable_by(|a, b| {
            if a.start > b.start {
                std::cmp::Ordering::Greater
            } else if a.start < b.start {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let mut result = vec![];
        for itvl in intervals.into_iter() {
            if result.len() == 0 {
                result.push(itvl);
                continue;
            }
            let mut last = result.last_mut().unwrap();
            if last.end >= itvl.start {
                (*last).end = itvl.end.max(last.end);
            } else {
                result.push(itvl);
            }
        }
        result
    }
}

struct Solution;
fn main() {
    println!("{:?}", Solution::merge());
}
