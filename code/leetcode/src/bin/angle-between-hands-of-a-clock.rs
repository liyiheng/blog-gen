struct Solution;
fn main() {}
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour = hour as f64;
        let minutes = minutes as f64;
        let h_a = (hour + minutes / 60.0) * 30.0;
        let m_a = minutes * 6.0;
        let angle = (h_a - m_a).abs();
        angle.min(360.0 - angle)
    }
}
