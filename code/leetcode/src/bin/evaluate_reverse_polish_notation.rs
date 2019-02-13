impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        Solution::eval(&mut tokens)
    }
    fn eval(tokens: &mut Vec<String>) -> i32 {
        let token = tokens.pop().unwrap();
        match token.as_str() {
            "+" => {
                let b = Solution::eval(tokens);
                let a = Solution::eval(tokens);
                a + b
            }
            "-" => {
                let b = Solution::eval(tokens);
                let a = Solution::eval(tokens);
                a - b
            }
            "*" => {
                let b = Solution::eval(tokens);
                let a = Solution::eval(tokens);
                a * b
            }
            "/" => {
                let b = Solution::eval(tokens);
                let a = Solution::eval(tokens);
                a / b
            }
            v => v.parse().unwrap_or(0),
        }
    }
}

struct Solution;
fn main() {
    let v = Solution::eval_rpn(
        vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect(),
    );
    assert_eq!(22, v);
    let v = Solution::eval_rpn(
        vec!["4", "13", "5", "/", "+"]
            .iter()
            .map(|v| v.to_string())
            .collect(),
    );
    assert_eq!(6, v);
    let v = Solution::eval_rpn(
        vec!["2", "1", "+", "3", "*"]
            .iter()
            .map(|v| v.to_string())
            .collect(),
    );
    assert_eq!(9, v);
}
