struct Solution {}
enum Parenthese {
    Left,
    Right,
    Full(i32),
}
impl Parenthese {
    pub fn content_len(&self) -> i32 {
        if let Parenthese::Full(l) = self {
            return *l;
        }
        return 0;
    }
    pub fn is_right(&self) -> bool {
        match self {
            Parenthese::Right => return true,
            _ => return false,
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            Parenthese::Left => return true,
            _ => return false,
        }
    }
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut parentheses = vec![];
        for c in s.chars() {
            let l = parentheses.len();
            if l == 0 {
                if c == '(' {
                    parentheses.push(Parenthese::Left);
                }
                continue;
            }
            let last = &parentheses[l - 1];
            if last.is_left() && c == ')' {
                parentheses[l - 1] = Parenthese::Full(2);
            } else {
                if c == '(' {
                    parentheses.push(Parenthese::Left);
                } else {
                    parentheses.push(Parenthese::Right);
                }
            }
        }
        let mut i = 0;
        while i < parentheses.len() {
            let size = parentheses[i].content_len();
            if size == 0 || i == 0 {
                i += 1;
                continue;
            }
            let left = &parentheses[i - 1];
            let left_size = left.content_len();
            if left_size > 0 {
                parentheses[i - 1] = Parenthese::Full(size + left_size);
                parentheses.remove(i);
                i -= 1;
                continue;
            }
            if i + 1 == parentheses.len() {
                i += 1;
                continue;
            }
            let right = &parentheses[i + 1];
            if left.is_left() && right.is_right() {
                parentheses[i] = Parenthese::Full(size + 2);
                parentheses.remove(i + 1);
                parentheses.remove(i - 1);
                i -= 1;
                continue;
            }
            i += 1;
        }
        let mut max = 0;
        for p in parentheses {
            let l = p.content_len();
            if l > max {
                max = l;
            }
        }
        max
    }
}

fn main() {
    assert_eq!(2, Solution::longest_valid_parentheses(String::from("(()")));
    assert_eq!(
        6,
        Solution::longest_valid_parentheses(String::from("(()())"))
    );
    assert_eq!(
        8,
        Solution::longest_valid_parentheses(String::from("(()(()()((((()()()()"))
    );
    assert_eq!(
        4,
        Solution::longest_valid_parentheses(String::from(")()())"))
    );
}
