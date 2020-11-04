use std::collections::HashSet;
impl Solution {
    fn find(sorted: &Vec<i32>, v: i32) -> i32 {
        if sorted.is_empty() {
            return -1;
        }
        let mut start = 0;
        let mut end = sorted.len() - 1;
        while start <= end {
            let mid = (start + end) / 2;
            if sorted[mid] == v {
                return mid as i32;
            }
            if sorted[mid] > v {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        -1
    }
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let s = arr.iter().cloned().collect::<HashSet<i32>>();
        let mut sorted = s.into_iter().collect::<Vec<i32>>();
        sorted.sort();
        for v in arr.iter_mut() {
            let i = Solution::find(&sorted, *v);
            *v = i + 1;
        }
        arr
    }
}
struct Solution;

fn main() {
    assert_eq!(
        Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
        vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
    );
    assert_eq!(
        Solution::array_rank_transform(vec![100, 100, 100]),
        vec![1, 1, 1]
    );
    assert_eq!(
        Solution::array_rank_transform(vec![100, 100, 200, 300]),
        vec![1, 1, 2, 3]
    );

    assert_eq!(
        Solution::array_rank_transform(vec![40, 10, 20, 30]),
        vec![4, 1, 2, 3]
    );
}
