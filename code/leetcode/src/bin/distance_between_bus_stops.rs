impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        if start == destination {
            return 0;
        }
        let cnt = distance.len();
        let (a, b) = if start < destination {
            (start, destination)
        } else {
            (start, destination + cnt as i32)
        };
        let mut dist = 0;
        for i in a..b {
            dist += distance[i as usize % cnt];
        }
        let a = a + cnt as i32;
        let mut dist2 = 0;
        for i in b..a {
            dist2 += distance[i as usize % cnt];
        }
        dist2.min(dist)
    }
}

struct Solution;
fn main() {
    println!(
        "{}",
        Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 1, 2)
    );
}
