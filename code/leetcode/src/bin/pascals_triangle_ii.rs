impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut last = vec![1];
        if row_index == 0 {
            return last;
        }
        for i in 0..row_index as usize + 1 {
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
            last = cur;
        }
        return last;
    }
}

struct Solution {}

fn main() {
    let r = Solution::get_row(3);
    println!("{:?}", r);
}
