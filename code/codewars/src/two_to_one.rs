fn longest(a1: &str, a2: &str) -> String {
    use std::collections::HashSet;
    let mut a: HashSet<char> = a1.chars().collect();
    a.extend(a2.chars());
    let mut v: Vec<char> = a.into_iter().collect();
    v.sort();
    v.into_iter().collect()
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    println!("s1:{:?} s2:{:?}", s1, s2);
    println!("{:?} {:?}", longest(s1, s2), exp);
    println!("{}", longest(s1, s2) == exp);
    assert_eq!(&longest(s1, s2), exp)
}

//#[test]
fn basic_tests() {
    testing("aretheyhere", "yestheyarehere", "aehrsty");
    testing(
        "loopingisfunbutdangerous",
        "lessdangerousthancoding",
        "abcdefghilnoprstu",
    );
}
