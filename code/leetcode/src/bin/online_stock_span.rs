fn main() {
    let mut s = StockSpanner::new();
    println!(
        "{},{},{},{},{},{},{}",
        s.next(100),
        s.next(80),
        s.next(60),
        s.next(70),
        s.next(60),
        s.next(75),
        s.next(85)
    );
}

//struct StockSpanner {
//    dat: Vec<i32>,
//}
//
//impl StockSpanner {
//    fn new() -> Self {
//        StockSpanner { dat: vec![] }
//    }
//
//    fn next(&mut self, price: i32) -> i32 {
//        self.dat.push(price);
//        let mut span = 0;
//        // Slow !!!
//        for p in self.dat.iter().rev() {
//            if *p > price {
//                break;
//            }
//            span += 1;
//        }
//        span
//    }
//}

// github.com/stackcats
struct StockSpanner {
    st: Vec<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner { st: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        if self.st.len() == 0 {
            self.st.push((1, price));
            return 1;
        }

        let mut curr = (1, price);
        while self.st.len() > 0 && self.st[self.st.len() - 1].1 <= price {
            let top = self.st.pop().unwrap();
            curr.0 += top.0;
        }

        self.st.push(curr);

        curr.0
    }
}
