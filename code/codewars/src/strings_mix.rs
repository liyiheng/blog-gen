fn mix(s1: &str, s2: &str) -> String {
    use std::collections::BTreeMap;
    let mut all = BTreeMap::new();
    let mut set1 = BTreeMap::new();
    let mut set2 = BTreeMap::new();
    s1.chars().filter(|c| c.is_lowercase()).for_each(|c| {
        let n = *set1.get(&c).unwrap_or(&0);
        set1.insert(c, n + 1);
    });
    s2.chars().filter(|c| c.is_lowercase()).for_each(|c| {
        let n = *set2.get(&c).unwrap_or(&0);
        set2.insert(c, n + 1);
    });
    all.extend(set1.iter());
    for (k, v) in set2.iter() {
        let x = *all.get(k).unwrap_or(&0);
        if x < *v {
            all.insert(*k, *v);
        }
    }
    let mut results = vec![];
    for (k, mut v) in all.into_iter() {
        if v < 2 {
            continue;
        }
        let mut s = String::new();
        let prefix = {
            let v1 = *set1.get(&k).unwrap_or(&0);
            let v2 = *set2.get(&k).unwrap_or(&0);
            if v1 == v2 {
                '='
            } else if v1 == v {
                '1'
            } else {
                '2'
            }
        };
        s.push(prefix);
        s.push(':');
        while v > 0 {
            s.push(k);
            v -= 1;
        }
        results.push(s);
    }
    results.sort_by(|s1, s2| {
        if s1.len() == s2.len() {
            s1.cmp(&s2)
        } else {
            s2.len().cmp(&s1.len())
        }
    });
    results.join("/")
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

//#[test]
fn basics_mix() {
    testing(
        "Are they here",
        "yes, they are here",
        "2:eeeee/2:yy/=:hh/=:rr",
    );
    testing(
        "looping is fun but dangerous",
        "less dangerous than coding",
        "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
    );
    testing(
        " In many languages",
        " there's a pair of functions",
        "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt",
    );
    testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing("codewars", "codewars", "");
    testing(
        "A generation must confront the looming ",
        "codewarrs",
        "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr",
    );
}
