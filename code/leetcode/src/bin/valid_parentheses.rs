fn main() {
    println!("{:?}", Solution::is_valid("[]{}()".to_string()));
    println!("{:?}", Solution::is_valid("(())".to_string()));
    println!("{:?}", Solution::is_valid("([)]".to_string()));
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for b in s.into_bytes().into_iter() {
            let size = stack.len();
            match b {
                b'[' | b'(' | b'{' => {
                    stack.push(b);
                    continue;
                }
                _ => {
                    if size == 0 {
                        return false;
                    }
                    let last = *stack.last().unwrap();
                    if (last == b'[' && b == b']')
                        || (last == b'(' && b == b')')
                        || (last == b'{' && b == b'}')
                    {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
        stack.len() == 0
    }
}
