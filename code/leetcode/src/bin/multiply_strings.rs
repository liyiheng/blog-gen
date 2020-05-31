impl Solution {
    fn b_2_i(b: u8) -> isize {
        if b >= b'0' && b <= b'9' {
            (b - b'0') as isize
        } else {
            0
        }
    }
    fn mul(digit: u8, pos: usize, v: &[u8]) -> Vec<u8> {
        let mut v = v.to_vec();
        let digit = Solution::b_2_i(digit);
        if digit == 0 {
            v.clear();
            v.push(b'0');
            return v;
        }
        if digit == 1 && pos == 0 {
            return v;
        }
        let mut extra = 0;
        for x in v.iter_mut().rev() {
            let part = Solution::b_2_i(*x) * digit + extra;
            *x = (part % 10) as u8 + b'0';
            extra = part / 10;
        }
        if extra > 0 {
            v.insert(0, extra as u8 + b'0');
        }
        for _ in 0..pos {
            v.push(b'0');
        }
        return v;
    }
    fn add(result: &mut Vec<u8>, delta: &[u8]) {
        while result.len() < delta.len() {
            result.insert(0, b'0');
        }
        let offset = result.len() - delta.len();
        let mut extra = 0;
        for i in (0..result.len()).rev() {
            let a = Solution::b_2_i(result[i]);
            let b = if i >= offset {
                Solution::b_2_i(delta[i - offset])
            } else {
                0
            };
            let sum = a + b + extra;
            result[i] = (sum % 10) as u8 + b'0';
            extra = sum / 10;
        }
        if extra > 0 {
            result.insert(0, extra as u8 + b'0');
        }
    }

    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.into_bytes();
        let num2 = num2.into_bytes();

        let mut result = vec![];
        let l = num1.len() - 1;
        // 123*12 => 123*10 + 123*2
        for (i, b) in num1.into_iter().enumerate() {
            let step_result = Solution::mul(b, l - i, &num2);
            Solution::add(&mut result, &step_result);
        }
        // 0000123 => 123
        while !result.is_empty() && result[0] == b'0' {
            result.remove(0);
        }
        // "" => "0"
        if result.is_empty() {
            result.push(b'0');
        }
        Some(0);
        String::from_utf8(result).unwrap()
    }
}
struct Solution;

fn check(a: isize, b: isize) {
    assert_eq!(
        (a * b).to_string(),
        Solution::multiply(a.to_string(), b.to_string())
    );
}

fn main() {
    check(123, 345);
    check(123, 300);
    check(1, 2);
    check(5, 6);
}
