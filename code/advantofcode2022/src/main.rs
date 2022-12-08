#![allow(dead_code)]
fn main() {
    day08::run();
    println!("Hello, world!");
}
pub fn read_dat() -> Option<String> {
    std::env::args()
        .last()
        .map(|p| std::fs::read_to_string(p).ok())
        .flatten()
}
mod day08 {
    use std::collections::HashSet;
    pub fn run() {
        let dat = super::read_dat();
        if dat.is_none() {
            return;
        }
        let dat = dat.unwrap();
        let grid: Vec<Vec<u8>> = dat
            .lines()
            .into_iter()
            .map(|line| line.as_bytes().to_vec())
            .collect();
        let mut pos = HashSet::new();
        for y in 0..grid[0].len() {
            // top -> bottom
            let mut max = 0;
            for x in 0..grid.len() {
                let b = grid[x][y];
                if b < max {
                    continue;
                }
                if b > max || x == 0 {
                    max = b;
                    pos.insert((x, y));
                }
            }
            // bottom -> top
            let mut max = 0;
            for x in (0..grid.len()).rev() {
                let b = grid[x][y];
                if b < max {
                    continue;
                }
                if b > max || x + 1 == grid.len() {
                    max = b;
                    pos.insert((x, y));
                }
            }
        }
        for (x, line) in grid.iter().enumerate() {
            // left to right
            let mut max = 0;
            for (y, &b) in line.iter().enumerate() {
                if b < max {
                    continue;
                }
                if b > max || y == 0 {
                    max = b;
                    pos.insert((x, y));
                }
            }
            // right to left
            let mut max = 0;
            for (y, &b) in line.iter().enumerate().rev() {
                if b < max {
                    continue;
                }
                if b > max || y + 1 == line.len() {
                    max = b;
                    pos.insert((x, y));
                }
            }
        }
        println!("day08 part1: {}", pos.len());
        let mut max = 0;
        for (x, line) in grid.iter().enumerate() {
            for (y, &b) in line.iter().enumerate() {
                let mut right = 0;
                for y1 in y + 1..line.len() {
                    right += 1;
                    if grid[x][y1] >= b {
                        break;
                    }
                }
                let mut left = 0;
                for y1 in (0..y).rev() {
                    left += 1;
                    if grid[x][y1] >= b {
                        break;
                    }
                }
                let mut up = 0;
                for x1 in (0..x).rev() {
                    up += 1;
                    if grid[x1][y] >= b {
                        break;
                    }
                }
                let mut down = 0;
                for x1 in x + 1..grid.len() {
                    down += 1;
                    if grid[x1][y] >= b {
                        break;
                    }
                }
                println!("({},{}): {}", x, y, right * left * up * down);
                if (x == 1 || x == 3) && y == 2 {
                    println!("{}*{}*{}*{}", up, left, right, down);
                }
                max = max.max(right * left * up * down);
            }
        }
        println!("day08 part2: {}", max);
    }
}
mod day07 {
    use std::collections::HashMap;
    enum Node {
        File(String, i64),
        Dir(String, HashMap<String, Node>),
    }
    pub fn run() {
        let dat = super::read_dat();
        if dat.is_none() {
            return;
        }
        let dat = dat.unwrap();
        let mut cur: HashMap<String, Node> = HashMap::new();
        for line in dat.lines() {
            if line.starts_with("$ cd") {
                let n = line.replace("$ cd", "");
                let cur = cur.get_mut(&n).unwrap();
            }
        }
    }
}
mod day06 {
    use std::collections::HashSet;
    pub fn run() {
        let dat = super::read_dat();
        if dat.is_none() {
            return;
        }
        let dat = dat.unwrap();
        for i in 3..dat.len() {
            let a = dat.as_bytes()[i - 3];
            let b = dat.as_bytes()[i - 2];
            let c = dat.as_bytes()[i - 1];
            let d = dat.as_bytes()[i - 0];
            let mut s = HashSet::new();
            s.insert(a);
            s.insert(b);
            s.insert(c);
            s.insert(d);
            if s.len() == 4 {
                println!("day 06 part1 {}", i + 1);
                break;
            }
        }
        for i in 13..dat.len() {
            let x = &dat.as_bytes()[i - 13..i + 1];
            let s: HashSet<u8> = x.into_iter().cloned().collect();
            if s.len() == 14 {
                println!("day 06 part2 {}", i + 1);
                break;
            }
        }
    }
}
mod day05 {
    use std::collections::HashMap;
    pub fn run() {
        //                 [V]     [C]     [M]
        // [V]     [J]     [N]     [H]     [V]
        // [R] [F] [N]     [W]     [Z]     [N]
        // [H] [R] [D]     [Q] [M] [L]     [B]
        // [B] [C] [H] [V] [R] [C] [G]     [R]
        // [G] [G] [F] [S] [D] [H] [B] [R] [S]
        // [D] [N] [S] [D] [H] [G] [J] [J] [G]
        // [W] [J] [L] [J] [S] [P] [F] [S] [L]
        //  1   2   3   4   5   6   7   8   9
        let mut stacks = HashMap::new();
        stacks.insert(1, vec!['W', 'D', 'G', 'B', 'H', 'R', 'V']);
        stacks.insert(2, vec!['J', 'N', 'G', 'C', 'R', 'F']);
        stacks.insert(3, vec!['L', 'S', 'F', 'H', 'D', 'N', 'J']);
        stacks.insert(4, vec!['J', 'D', 'S', 'V']);
        stacks.insert(5, vec!['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V']);
        stacks.insert(6, vec!['P', 'G', 'H', 'C', 'M']);
        stacks.insert(7, vec!['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C']);
        stacks.insert(8, vec!['S', 'J', 'R']);
        stacks.insert(9, vec!['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M']);
        let mut stacks2 = stacks.clone();

        let dat = super::read_dat();
        if dat.is_none() {
            return;
        }
        let dat = dat.unwrap();
        for line in dat.lines() {
            let segs: Vec<_> = line.split(" ").collect();
            let cnt: u32 = segs[1].parse().unwrap();
            let src: i32 = segs[3].parse().unwrap();
            let dst: i32 = segs[5].parse().unwrap();
            for _ in 0..cnt {
                let c = stacks.get_mut(&src).unwrap().pop().unwrap();
                stacks.get_mut(&dst).unwrap().push(c);
            }
            let mut tmp = vec![];
            for _ in 0..cnt {
                let c = stacks2.get_mut(&src).unwrap().pop().unwrap();
                tmp.push(c);
            }
            tmp.reverse();
            stacks2.get_mut(&dst).unwrap().append(&mut tmp);
        }
        println!("day05 part 1 top:");
        for i in 1..=9 {
            print!("{}", stacks.get(&i).unwrap().last().cloned().unwrap_or('*'));
        }
        println!("");
        println!("day05 part 2 top:");
        for i in 1..=9 {
            print!(
                "{}",
                stacks2.get(&i).unwrap().last().cloned().unwrap_or('*')
            );
        }
        println!("");
    }
}
mod day04 {
    pub fn run() {
        let dat = super::read_dat();
        if dat.is_none() {
            return;
        }
        let dat = dat.unwrap();
        let parse_range = |s: &str| -> (i32, i32) {
            let (a, b) = s.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        };
        let mut cnt_full = 0;
        let mut cnt_overlap = 0;
        for line in dat.lines() {
            let (r1, r2) = line.split_once(",").unwrap();
            let r1 = parse_range(r1);
            let r2 = parse_range(r2);
            if (r1.0 <= r2.0 && r1.1 >= r2.1) || (r1.0 >= r2.0 && r1.1 <= r2.1) {
                cnt_full += 1;
            }
            if !(r1.0 > r2.1 || r2.0 > r1.1) {
                cnt_overlap += 1;
            }
        }
        println!("day04 part 1 num: {}", cnt_full);
        println!("day04 part 2 num: {}", cnt_overlap);
    }
}
mod day03 {
    use std::collections::HashSet;
    pub fn run() {
        let dat = super::read_dat();
        if dat.is_none() {
            return;
        }
        let dat = dat.unwrap();
        let mut sum = 0;
        for line in dat.lines() {
            let tmp = line.as_bytes();
            let (head, tail) = tmp.split_at(tmp.len() / 2);
            let head: HashSet<u8> = head.iter().cloned().collect();
            let tail: HashSet<u8> = tail.iter().cloned().collect();
            let b = *head.intersection(&tail).next().unwrap();
            let p = if b.is_ascii_lowercase() {
                b - b'a' + 1
            } else {
                b - b'A' + 27
            };
            sum += p as i32;
        }
        println!("day03 part 1 sum: {}", sum);
        sum = 0;
        let lines: Vec<_> = dat.lines().collect();
        for i in (0..lines.len()).step_by(3) {
            let a: HashSet<u8> = lines[i].as_bytes().into_iter().cloned().collect();
            let b: HashSet<u8> = lines[i + 1].as_bytes().into_iter().cloned().collect();
            let c: HashSet<u8> = lines[i + 2].as_bytes().into_iter().cloned().collect();
            let a = a.intersection(&b).cloned().collect::<HashSet<u8>>();
            let b = *a.intersection(&c).next().unwrap();
            let p = if b.is_ascii_lowercase() {
                b - b'a' + 1
            } else {
                b - b'A' + 27
            };
            sum += p as i32;
        }
        println!("day03 part 2 sum: {}", sum);
    }
}
mod day02 {
    fn get_score(line: &str) -> i32 {
        let tmp = line.as_bytes();
        let a = tmp[0] - b'A' + 1;
        let b = tmp[2] - b'X' + 1;
        let s2 = if a == b {
            3
        } else if (a < b && b - a == 1) || (a == 3 && b == 1) {
            6
        } else {
            0
        };
        (b + s2) as i32
    }
    fn get_score2(line: &str) -> i32 {
        let tmp = line.as_bytes();
        let a = tmp[0] - b'A';
        let b = tmp[2] - b'X';
        match (a, b) {
            (0, 0) => 3 + 0, // 石头 vs 剪刀
            (0, 1) => 1 + 3, // 石头 vs 石头
            (0, 2) => 2 + 6, // 石头 vs 布
            (1, 0) => 1 + 0, // 布 vs 石头
            (1, 1) => 2 + 3, // 布 vs 布
            (1, 2) => 3 + 6, // 布 vs 剪刀
            (2, 0) => 2 + 0,
            (2, 1) => 3 + 3,
            (2, 2) => 1 + 6,
            _ => 0,
        }
    }
    pub(crate) fn calc_score() {
        let dat = super::read_dat();
        if dat.is_none() {
            return;
        }
        let dat = dat.unwrap();
        let sum: i32 = dat.lines().map(|line| get_score(line)).sum();
        println!("day 02 part 1: {}", sum);
        let sum: i32 = dat.lines().map(|line| get_score2(line)).sum();
        println!("day 02 part 2: {}", sum);
    }
}
mod day01 {
    pub(crate) fn count() {
        let dat = super::read_dat();
        if dat.is_none() {
            return;
        }
        let dat = dat.unwrap();
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::with_capacity(3);
        let mut cur = 0;
        for l in dat.lines() {
            if l.is_empty() {
                heap.push(Reverse(cur));
                if heap.len() > 3 {
                    heap.pop();
                }
                cur = 0;
                continue;
            }
            let v: isize = l.parse().unwrap();
            cur += v;
        }
        heap.push(Reverse(cur));
        if heap.len() > 3 {
            heap.pop();
        }
        let third = heap.pop().unwrap_or_default().0;
        let second = heap.pop().unwrap_or_default().0;
        let max = heap.pop().unwrap_or_default().0;
        println!(
            "total: {}, max:{},2nd:{},3rd:{}",
            max + second + third,
            max,
            second,
            third
        );
    }
}
