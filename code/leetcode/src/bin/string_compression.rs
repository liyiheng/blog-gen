struct Solution;

fn main() {
    let mut x = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let n = Solution::compress(&mut x);
    println!("{}, {:?}", n, x);

    let mut x = vec!['a'];
    let n = Solution::compress(&mut x);
    println!("{}, {:?}", n, x);

    let mut x = vec!['a', 'b', 'c'];
    let n = Solution::compress(&mut x);
    println!("{}, {:?}", n, x);

    let mut x = vec!['a', 'b', 'c', 'c'];
    let n = Solution::compress(&mut x);
    println!("{}, {:?}", n, x);
}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let conv_cnt = |n: i32| -> Vec<char> {
            if n == 1 {
                vec![]
            } else {
                let s = n.to_string();
                s.chars().collect::<Vec<char>>()
            }
        };
        let mut pos = 0;
        let mut cnt = 0;
        for i in 0..chars.len() {
            let c = chars[i];
            if i == 0 {
                cnt = 1;
                continue;
            }
            if c == chars[pos] {
                cnt += 1;
                continue;
            }
            pos += 1;
            if cnt == 1 {
                chars[pos] = c;
                continue;
            }
            let digits = conv_cnt(cnt);
            for (j, &d) in digits.iter().enumerate() {
                chars[pos + j] = d;
            }
            pos += digits.len();
            chars[pos] = c;
            cnt = 1;
        }
        pos += 1;
        let digits = conv_cnt(cnt);
        for (j, &d) in digits.iter().enumerate() {
            chars[pos + j] = d;
        }
        (digits.len() + pos) as i32
    }
}
