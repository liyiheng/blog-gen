use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t = input_line.trim_end().to_string().to_uppercase();
    let mut table = vec![];
    for _ in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_end().to_string();
        table.push(row.chars().collect::<Vec<char>>())
    }
    let l = l as usize;
    let indexes: Vec<usize> = t
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                c as usize - 'A' as usize
            } else {
                26
            }
        })
        .collect();
    let mut result = vec![];
    for row in 0..h as usize {
        let mut result_line = vec![];
        let line = table.get(row).unwrap();
        for i in indexes.iter() {
            let i_start = l * i;
            let i_end = l * i + l;
            for j in i_start..i_end {
                let c = line.get(j).unwrap_or(&' ');
                result_line.push(*c);
            }
        }
        result.push(result_line);
    }
    for r in result {
        let s: String = r.iter().collect();
        println!("{}", s);
    }
}
