struct Solution;
fn main() {
    Solution::max_satisfied(
        vec![1, 0, 1, 2, 1, 1, 7, 5],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        3,
    );
}
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let number: i32 = customers
            .iter()
            .zip(grumpy.iter())
            .map(|(n, g)| n * (g ^ 1))
            .sum();
        let minutes = (minutes as usize).min(customers.len());
        let mut extra = 0;
        for i in 0..minutes {
            extra += customers[i] * grumpy[i];
        }
        let mut cur = number + extra;
        let mut ans = cur;
        for i in minutes..customers.len() {
            if grumpy[i] == 1 {
                cur += customers[i];
            }
            if grumpy[i - minutes] == 1 {
                cur -= customers[i - minutes];
            }
            ans = ans.max(cur);
        }
        ans
    }
}
