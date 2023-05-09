struct Solution;

pub fn main() {
    let a = "e/*/eabeea/*///*c*////*dc*//bcadcde/*/acbe//*d/*/*//ae//*dc//*cc//*//*eaebb*//";
    dbg!(Solution::remove_comments(vec![a.to_string()]));
}

impl Solution {
    pub fn remove_comments(lines: Vec<String>) -> Vec<String> {
        let single = "//".as_bytes();
        let multi_s = "/*".as_bytes();
        let multi_e = "*/".as_bytes();
        let mut code = lines.join("\n").into_bytes();
        let mut i = 0;
        while i + 1 < code.len() {
            let signle_start = &code[i..i + 2] == single;
            if signle_start {
                // 从 "//" 删到换行符，换行符本身不删
                code.remove(i);
                code.remove(i);
                while i < code.len() && code[i] != b'\n' {
                    code.remove(i);
                }
                continue;
            }
            let multi_start = &code[i..i + 2] == multi_s;
            if multi_start {
                // 从 "/*" 开始删，直到 "*/"
                code.remove(i);
                code.remove(i);
                while &code[i..i + 2] != multi_e {
                    code.remove(i);
                }
                // 移除最后的 "*/"
                code.remove(i);
                code.remove(i);
                continue;
            }
            i += 1;
        }
        String::from_utf8(code)
            .unwrap()
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect()
    }
}
