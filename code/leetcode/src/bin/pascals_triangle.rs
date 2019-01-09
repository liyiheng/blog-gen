/*
Input: 5
Output:
[
     [1],
    [1,1],
   [1,2,1],
  [1,3,3,1],
 [1,4,6,4,1]
]
*/
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if num_rows == 0 {
            return result;
        }
        result.push(vec![1]);
        if num_rows == 1 {
            return result;
        }
        for i in 1..num_rows as usize {
            let last = &result[i - 1];
            let mut cur = vec![];
            for j in 0..i + 1 {
                let mut a = 0;
                let mut b = 0;
                if j > 0 {
                    a = last[j - 1];
                }
                if j < last.len() {
                    b = last[j];
                }
                cur.push(a + b);
            }
            result.push(cur);
        }
        return result;
    }
}

struct Solution {}

fn main() {
    let r = Solution::generate(5);
    println!("{:?}", r);
}
