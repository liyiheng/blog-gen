impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut v = vec![];
        Solution::generate(n, n, String::default(), &mut v);
        v
    }
    fn generate(left: i32, right: i32, cur: String, result: &mut Vec<String>) {
        if right == 0 && left == 0 {
            result.push(cur);
            return;
        }

        if left > 0 || left == right {
            let mut cur = cur.clone();
            cur.push('(');
            Solution::generate(left - 1, right, cur, result);
            if left == right {
                return;
            }
        }

        let mut cur_r = cur.clone();
        cur_r.push(')');
        Solution::generate(left, right - 1, cur_r, result);
        return;
    }
}

struct Solution;

fn main() {
    let a = Solution::generate_parenthesis(3);
    println!("{:?}", a);
}
