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
    pub fn insert(mut intervals: Vec<Interval>, new_interval: Interval) -> Vec<Interval> {
        let l = intervals.len();
        if l == 0 {
            return vec![new_interval];
        }
        let mut lo = 0;
        let mut hi = l as i32 - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if intervals[mid as usize].start == new_interval.start {
                lo = mid;
                break;
            }
            if intervals[mid as usize].start > new_interval.start {
                hi = mid - 1;
                continue;
            }
            lo = mid + 1;
        }
        let i = if lo >= l as i32 {
            l
        } else if intervals[lo as usize].start > new_interval.start {
            lo as usize
        } else {
            lo as usize + 1
        };
        intervals.insert(i, new_interval);
        let mut start = if i == 0 { 0 } else { i - 1 };
        while start + 1 < intervals.len() && start <= i {
            let (b_start, b_end) = {
                let tmp = &intervals[start + 1];
                (tmp.start, tmp.end)
            };
            let a = &mut intervals[start];
            if a.end >= b_start {
                (*a).end = a.end.max(b_end);
                intervals.remove(start + 1);
            } else {
                start += 1;
            }
        }
        intervals
    }
}

struct Solution;

fn main() {
    let mut a = vec![];
    a.push(Interval::new(1, 2));
    a.push(Interval::new(3, 5));
    a.push(Interval::new(6, 7));
    a.push(Interval::new(8, 10));
    a.push(Interval::new(12, 16));
    let b = Solution::insert(a, Interval::new(4, 8));
    println!("{:?}", b);
    let b = Solution::insert(
        vec![Interval::new(0, 5), Interval::new(9, 12)],
        Interval::new(7, 16),
    );
    println!("{:?}", b);
}
