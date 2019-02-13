impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut sums = vec![];
        for i in 0..nums.len() {
            if i == 0 {
                sums.push((nums[i], true));
                continue;
            }
            let a = nums[i];
            let (b, continous) = sums.last_mut().unwrap();
            if *continous {
                let c = a + *b;
                let max = a.max(*b).max(c);
                if max == *b {
                    *continous = false;
                    if c > 0 {
                        sums.push((c, true));
                    }
                } else {
                    *b = max;
                }
            } else {
                sums.push((a, true));
            }
        }
        sums.iter()
            .fold(std::i32::MIN, |max, &v| if v.0 > max { v.0 } else { max })
    }
}

struct Solution;

fn main() {
    assert_eq!(7, Solution::max_sub_array(vec![-2, 3, 1, 3]));
    assert_eq!(
        6,
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
}
