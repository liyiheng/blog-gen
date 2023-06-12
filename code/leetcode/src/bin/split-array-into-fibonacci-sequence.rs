struct Solution;
fn main() {
    Solution::split_into_fibonacci("1101111".to_string());
}
fn parse(dat: &[u8]) -> i32 {
    assert!(!dat.is_empty());
    let mut ans = 0;
    for &b in dat.iter() {
        ans *= 10;
        ans += (b - b'0') as i32;
    }
    ans
}
fn dfs(ans: &mut Vec<i32>, dat: &[u8]) -> bool {
    if dat.is_empty() {
        return ans.len() >= 3;
    }
    if ans.len() > 1 {
        let target = ans[ans.len() - 1] + ans[ans.len() - 2];
        let t_s = target.to_string();
        if dat.starts_with(t_s.as_bytes()) {
            ans.push(target);
            if dfs(ans, &dat[t_s.len()..]) {
                return true;
            }
            ans.pop();
            return false;
        }
        return false;
    }
    for i in 1..=dat.len() {
        let seg = &dat[..i];
        if seg.len() > 1 && seg[0] == b'0' {
            break;
        }
        let v = parse(seg);
        ans.push(v);
        let ok = dfs(ans, &dat[seg.len()..]);
        if ok {
            return true;
        }
        ans.pop();
    }
    false
}
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut ans = vec![];
        let ok = dfs(&mut ans, num.as_bytes());
        if !ok || ans.len() < 3 {
            vec![]
        } else {
            ans
        }
    }
}
