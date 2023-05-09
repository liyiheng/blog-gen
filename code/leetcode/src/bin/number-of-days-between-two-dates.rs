struct Solution;
fn main() {}
impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let parse = |s: &str| -> (i32, i32, i32) {
            let segs: Vec<i32> = s.split("-").map(|v| v.parse().unwrap()).collect();
            (segs[0], segs[1], segs[2])
        };
        let (y1, m1, d1) = parse(&date1);
        let (y2, m2, d2) = parse(&date2);
        let calculate = |y, m, d| -> i32 {
            let mut n = 0;
            for i in 1970..y {
                n += 365;
                let leap = i % 4 == 0 && (i % 100 != 0 || i % 400 == 0);
                if leap {
                    n += 1;
                }
            }
            let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
            let md = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            for i in 1..m as usize {
                n += md[i];
                if i == 2 && leap {
                    n += 1;
                }
            }
            n + d
        };
        (calculate(y1, m1, d1) - calculate(y2, m2, d2)).abs()
    }
}
