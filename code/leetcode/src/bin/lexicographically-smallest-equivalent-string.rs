struct Solution;
fn main() {}
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        use std::collections::BTreeSet;
        let mut sets: Vec<BTreeSet<u8>> = vec![];
        for (&a, &b) in s1.as_bytes().iter().zip(s2.as_bytes().iter()) {
            let mut inserted = false;
            for s in sets.iter_mut() {
                if s.contains(&a) {
                    s.insert(b);
                    inserted = true;
                }
                if s.contains(&b) {
                    s.insert(a);
                    inserted = true;
                }
            }
            if !inserted {
                let mut m = BTreeSet::new();
                m.insert(a);
                m.insert(b);
                sets.push(m);
            }
        }
        for i in 0..sets.len() {
            for j in (i + 1)..sets.len() {
                if sets[i].intersection(&sets[j]).next().is_some() {
                    let mut tmp = sets[j].clone();
                    sets[i].append(&mut tmp);
                    sets[j].clear();
                }
            }
        }
        let sets: Vec<_> = sets.into_iter().filter(|v| v.len() > 0).collect();
        let data = base_str
            .as_bytes()
            .iter()
            .map(|b| {
                for s in sets.iter() {
                    if s.contains(b) {
                        return s.iter().next().unwrap();
                    }
                }
                b
            })
            .cloned()
            .collect();
        String::from_utf8(data).unwrap()
    }
}
