struct Solution;
fn main() {
    Solution::have_conflict(
        vec!["10:00".to_string(), "11:00".to_string()],
        vec!["14:00".to_string(), "15:00".to_string()],
    );
}
impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let start1 = event1[0].as_str();
        let end1 = event1[1].as_str();
        let start2 = event2[0].as_str();
        let end2 = event2[1].as_str();
        end2.min(end1) >= start2.max(start1)
    }
}
