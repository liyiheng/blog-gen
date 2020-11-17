impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut result = 0;
        for i in 0..16 {
            let m1 = (x >> i) << 31 >> i;
            let m2 = (x >> (31 - i)) << 31 >> (31 - i);
            result |= m1;
            result |= m2;
        }
        result
    }
}
struct Solution;
fn main() {}
