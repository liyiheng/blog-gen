#![allow(dead_code)]
// Definition for a point.
#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
struct Solution {}
impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
impl Solution {
    pub fn max_points(points: Vec<Point>) -> i32 {
        let mut max = 0;
        let length = points.len();
        for i in 0..length {
            for j in (i + 1)..length {
                let p1 = &points[i];
                let p2 = &points[j];
                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                if dx == 0 && dy == 0 {
                    continue;
                }
                let mut cnt = 0;
                for p in &points {
                    let x = p.x - p2.x;
                    let y = p.y - p2.y;
                    // x/y == dx/dy
                    // x*dy == y*dx
                    // TODO: Use gcd instead of 'i32 as i64'
                    if x as i64 * dy as i64 == y as i64 * dx as i64 {
                        cnt += 1;
                    }
                }
                if cnt > max {
                    max = cnt;
                }
            }
        }

        if max == 0 {
            length as i32
        } else {
            max
        }
    }
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }
}
fn main() {
    // [[0,0],[1,65536],[65536,0]]
    let mut dat = Vec::with_capacity(3);
    dat.push(Point { x: 0, y: 0 });
    dat.push(Point { x: 1, y: 65536 });
    dat.push(Point { x: 65536, y: 0 });
    println!("{}", Solution::max_points(dat));
}
