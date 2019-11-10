use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // Number of elements which make up the association table.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let q = parse_input!(input_line, i32); // Number Q of file names to be analyzed.
    let mut kvs = std::collections::HashMap::<String, String>::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let ext = inputs[0].trim().to_string(); // file extension
        let mt = inputs[1].trim().to_string(); // MIME type.
        kvs.insert(ext.to_uppercase(), mt);
    }
    let mut result = vec![];
    'outer: for i in 0..q as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let fname = input_line.trim_end().to_string().to_uppercase(); // One file name per line.
        let segs = fname.split(".").collect::<Vec<&str>>();
        if segs.len() == 1 {
            result.push(None);
            continue;
        }
        let l = segs.last();
        for seg in segs.iter() {
            if segs.len() == 0 {
                result.push(None);
                eprintln!("{}", input_line);
                continue 'outer;
            }
        }

        if l.is_none() {
            result.push(None);
        } else {
            result.push(kvs.get(&l.unwrap().to_string()).clone());
        }
    }

    result.iter().for_each(|mt| {
        let v = mt.map(|v| v.as_ref()).unwrap_or("UNKNOWN");
        println!("{}", v);
    });
}
