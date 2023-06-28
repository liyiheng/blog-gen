fn main() {
    let mut h = BrowserHistory::new("leetcode.com".to_owned());
    h.visit("google.com".to_owned());
    h.visit("facebook.com".to_owned());
    h.visit("youtube.com".to_owned());
    dbg!(h.back(1));
    dbg!(h.back(1));
    dbg!(h.forward(1));
    h.visit("linkedin.com".to_owned());
    dbg!(h.forward(2));
    dbg!(h.back(2));
    dbg!(h.back(7));
}
struct BrowserHistory {
    urls: Vec<String>,
    current: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            urls: vec![homepage],
            current: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.current += 1;
        if self.current == self.urls.len() {
            self.urls.push(url);
        } else {
            self.urls[self.current] = url;
            while self.current + 1 < self.urls.len() {
                self.urls.pop();
            }
        }
        assert!(self.current + 1 == self.urls.len());
    }

    fn back(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        self.current = self.current - steps.min(self.current);
        self.urls[self.current].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let target = steps as usize + self.current;
        self.current = target.min(self.urls.len() - 1);
        self.urls[self.current].clone()
    }
}
