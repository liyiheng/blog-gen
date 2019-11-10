use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let message = input_line.trim_end().to_string();
    println!("{}", conv(message));
}

#[test]
fn test() {
    println!("{}", conv("C".to_owned()));
    println!("{}", conv("CC".to_owned()));
}

fn conv(message: String) -> String {
    let data = message.as_bytes();
    let mask = 0b01000000;
    let mut info = vec![];
    data.iter().for_each(|v| {
        for i in 0..7 {
            let a = v & (mask >> i);
            let a: u8 = if a == 0 { 0 } else { 1 };
            if info.is_empty() {
                info.push((a, 1));
                continue;
            }
            if info.last().unwrap().0 == a {
                info.last_mut().unwrap().1 += 1;
            } else {
                info.push((a, 1));
            }
        }
    });
    let mut result = info.iter().fold(String::new(), |mut s, (v, c)| {
        s.push('0');
        if *v == 0 {
            s.push('0');
        }
        s.push(' ');
        for _ in 0..*c {
            s.push('0');
        }
        s.push(' ');
        s
    });
    result.pop();
    result
}
