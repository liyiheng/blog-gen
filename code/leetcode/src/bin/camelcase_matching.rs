struct Solution;
pub fn main() {
    dbg!(Solution::camel_match(
        vec!["FooBar".to_string()],
        "FoBaT".to_string()
    ));
}
impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        queries
            .iter()
            .map(|q| Solution::check(q.as_str(), pattern.as_str()))
            .collect()
    }
    fn check(query: &str, pattern: &str) -> bool {
        let mut query = query.as_bytes().iter().peekable();
        let mut pattern = pattern.as_bytes().iter().peekable();
        'outer: loop {
            if pattern.peek().is_none() {
                return query.find(|v| v.is_ascii_uppercase()).is_none();
            }
            let &p = pattern.next().unwrap();
            while query.peek().is_some() {
                let &q = query.next().unwrap();
                if p == q {
                    continue 'outer;
                }
                if q.is_ascii_uppercase() {
                    return false;
                }
            }
            return false;
        }
    }
}

