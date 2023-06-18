struct Solution;
mod detect_border;
fn main() {
    let results: Vec<_> = std::env::args()
        .skip(1)
        .map(|p| (p.clone(), detect_border::detect(p)))
        .collect();
    results.iter().filter(|r| r.1.is_err()).for_each(|(p, r)| {
        println!("detect failed: {}, {:?}", p, r.as_ref().unwrap_err());
    });
    results
        .iter()
        .filter(|r| r.1.is_ok())
        .map(|(p, r)| (p, r.as_ref().unwrap().clone()))
        .for_each(|(p, r)| {
            let s = r.problems(3);
            if !s.is_empty() {
                println!("{}\t{}", p, s);
            }
        });
}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let half = nums.len() as i32 / 2;
        use std::collections::HashMap;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            let c = counter.entry(n).or_default();
            *c += 1;
            if *c > half {
                return n;
            }
        }
        unreachable!()
    }
}
