struct Solution;
fn main() {}
impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut x = vec![a, b, c];
        x.sort();
        let a = x[0];
        let b = x[1];
        let c = x[2];
        let mut ans = vec![0, 0];
        if a + 2 == c {
            // 已经连续
            ans[0] = 0;
        } else if a + 1 == b || b + 1 == c {
            // 已有两个相邻
            ans[0] = 1;
        } else if b - a == 2 || c - b == 2 {
            // 空隙为1
            ans[0] = 1;
        } else {
            ans[0] = 2;
        }
        ans[1] = c - a - 2;
        ans
    }
}
