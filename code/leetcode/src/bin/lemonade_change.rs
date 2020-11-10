impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        if bills.is_empty() {
            return true;
        }
        let mut five = 0;
        let mut ten = 0;
        for b in bills {
            match b {
                5 => {
                    five += 1;
                }
                10 => {
                    if five == 0 {
                        return false;
                    }
                    five -= 1;
                    ten += 1;
                }
                20 => {
                    if five == 0 {
                        return false;
                    }
                    if ten > 0 {
                        ten -= 1;
                        five -= 1;
                    } else {
                        if five < 3 {
                            return false;
                        }
                        five -= 3;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}

struct Solution;
fn main() {
    assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    assert!(Solution::lemonade_change(vec![5, 5, 5, 10]));
    assert!(!Solution::lemonade_change(vec![10, 10]));
    assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
}
