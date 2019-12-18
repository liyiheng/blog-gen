fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut s = String::new();
    if list_art.is_empty() {
        return s;
    }
    let mut counter = std::collections::HashMap::new();
    for i in list_art {
        let v: Vec<&str> = i.splitn(2, ' ').collect();
        let value = v[1].parse::<i32>().unwrap();
        if value == 0 {
            continue;
        }
        let c = v[0].chars().next().unwrap();
        let n = *counter.get(&c).unwrap_or(&0);
        counter.insert(c, n + value);
    }
    for c in list_cat {
        let n = counter
            .get(c.chars().next().as_ref().unwrap())
            .unwrap_or(&0);
        s.push_str(&format!("({} : {}) - ", c, n));
    }
    s.pop();
    s.pop();
    s.pop();
    s
}
