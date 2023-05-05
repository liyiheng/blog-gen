struct Solution;
fn main() {
    let r = vec![1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1, 3, 2, 1, 1, 2, 3, 4];
    dbg!(Solution::candy(r));
    dbg!(Solution::candy(vec![4, 3, 3, 2, 1]));
}
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut indexes = vec![];
        for i in 0..ratings.len() {
            let mut j = i;
            let mut k = i;
            while j > 0 && ratings[j] == ratings[i] {
                j -= 1;
            }
            while k < ratings.len() && ratings[k] == ratings[i] {
                k += 1;
            }
            if i == 0 {
                if let Some(&next) = ratings.get(k) {
                    if ratings[i] < next {
                        indexes.push(i);
                    }
                }
                continue;
            }
            if ratings.get(k).is_none() {
                if ratings[i] < ratings[j] {
                    indexes.push(i);
                }
                continue;
            }
            if ratings[j] > ratings[i] && ratings[i] < ratings[k] {
                indexes.push(i);
            }
        }
        let mut candies = vec![1; ratings.len()];
        for i in indexes {
            // 从 i 往前发
            for j in (0..i).rev() {
                if ratings[j] > ratings[j + 1] {
                    candies[j] = (candies[j + 1] + 1).max(candies[j]);
                }
            }
            // 从 i 往后发
            for j in (i + 1)..candies.len() {
                if ratings[j] > ratings[j - 1] {
                    candies[j] = (candies[j - 1] + 1).max(candies[j]);
                }
            }
        }
        println!("ratings:{:?}", ratings);
        println!("candies:{:?}", candies);
        candies.into_iter().sum()
    }
}
