fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    let mut b_to_l = BTreeMap::new();
    triplets.iter().for_each(|triplet| {
        let a = triplet[0];
        let b = triplet[1];
        let c = triplet[2];
        if !b_to_l.contains_key(&a) {
            b_to_l.insert(a, BTreeSet::new());
        }
        if !b_to_l.contains_key(&b) {
            b_to_l.insert(b, BTreeSet::new());
        }
        if !b_to_l.contains_key(&c) {
            b_to_l.insert(c, BTreeSet::new());
        }
        b_to_l.get_mut(&c).unwrap().insert(b);
        b_to_l.get_mut(&c).unwrap().insert(a);
        b_to_l.get_mut(&b).unwrap().insert(a);
    });
    let mut reslut = vec![];
    while b_to_l.len() > 0 {
        let mut c = None;
        b_to_l.iter().any(|(k, set)| {
            if set.is_empty() {
                c = Some(*k);
                true
            } else {
                false
            }
        });
        let c = c.unwrap();
        b_to_l.remove(&c);
        b_to_l.iter_mut().for_each(|(_, set)| {
            set.remove(&c);
        });
        reslut.push(c);
    }
    reslut.into_iter().collect()
}

//#[test]
fn example_test() {
    assert_eq!(
        recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']
        ]),
        "whatisup"
    );
}
